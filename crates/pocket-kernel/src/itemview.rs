//! The two item controls from `commctrl.dll`: `SysTreeView32` and
//! `SysListView32`.
//!
//! On a device these are windows whose procedure lives in the common
//! controls library. The application only talks to them in messages —
//! `TVM_INSERTITEM` to add a node, `LVM_SETITEMTEXT` to fill a cell — and
//! hears back through `WM_NOTIFY` when the user taps, selects or expands
//! something. File managers, registry editors and most "explorer" style
//! programs are built on them: Pocket Explorer creates a tree, inserts a
//! single root, expands it, and fills every level in from its
//! `TVN_ITEMEXPANDING` handler.
//!
//! This module owns the item models, their painting and their stylus and
//! key handling. Turning messages into calls on these models, and
//! [`ViewEvent`]s into `WM_NOTIFY`s in guest memory, is the dispatcher's
//! job.

use std::collections::HashMap;

use crate::font;
use crate::gdi::{ImageList, Surface};

/// `COLOR_WINDOW`.
const WINDOW: u16 = 0xFFFF;
/// `COLOR_WINDOWTEXT`.
const TEXT: u16 = 0x0000;
/// `COLOR_HIGHLIGHT` on a Pocket PC: the same dark blue as a caption.
const HIGHLIGHT: u16 = 0x0010;
/// `COLOR_HIGHLIGHTTEXT`.
const HIGHLIGHT_TEXT: u16 = 0xFFFF;
/// Grey of the dotted tree lines and the expand box frame.
const LINE: u16 = 0x8410;
/// `COLOR_BTNFACE`, for list view column headers.
const FACE: u16 = 0xC618;
const LIGHT: u16 = 0xFFFF;
const SHADOW: u16 = 0x8410;

/// Width of the scroll strip drawn when the items do not fit.
const SCROLL_W: i32 = 9;

/// `WS_BORDER`; a control with it gets a one-pixel black frame.
const WS_BORDER: u32 = 0x0080_0000;

/// `TVS_*` styles.
pub const TVS_HASBUTTONS: u32 = 0x0001;
pub const TVS_HASLINES: u32 = 0x0002;
pub const TVS_LINESATROOT: u32 = 0x0004;

/// `TVIS_*` item states.
pub const TVIS_SELECTED: u32 = 0x0002;
pub const TVIS_BOLD: u32 = 0x0010;
pub const TVIS_EXPANDED: u32 = 0x0020;
pub const TVIS_EXPANDEDONCE: u32 = 0x0040;

/// `I_CHILDRENCALLBACK`: the item may have children the application has
/// not inserted yet. It still gets an expand box.
pub const I_CHILDRENCALLBACK: i32 = -1;

/// `TVE_*` actions for `TVM_EXPAND`.
pub const TVE_COLLAPSE: u32 = 0x0001;
pub const TVE_EXPAND: u32 = 0x0002;
pub const TVE_TOGGLE: u32 = 0x0003;
pub const TVE_COLLAPSERESET: u32 = 0x8000;

/// Special `hInsertAfter` and `hParent` values.
pub const TVI_ROOT: u32 = 0xFFFF_0000;
pub const TVI_FIRST: u32 = 0xFFFF_0001;
pub const TVI_LAST: u32 = 0xFFFF_0002;
pub const TVI_SORT: u32 = 0xFFFF_0003;

/// `LVS_*` view styles (the low two bits of the style).
pub const LVS_ICON: u32 = 0x0000;
pub const LVS_REPORT: u32 = 0x0001;
pub const LVS_SMALLICON: u32 = 0x0002;
pub const LVS_LIST: u32 = 0x0003;
pub const LVS_TYPEMASK: u32 = 0x0003;
pub const LVS_SINGLESEL: u32 = 0x0004;
pub const LVS_NOCOLUMNHEADER: u32 = 0x4000;

/// `LVIS_*` item states.
pub const LVIS_FOCUSED: u32 = 0x0001;
pub const LVIS_SELECTED: u32 = 0x0002;

/// One node of a tree view.
#[derive(Debug, Clone, Default)]
pub struct TreeItem {
    /// `HTREEITEM` handed to the application.
    pub handle: u32,
    /// Parent handle, or `0` for a top-level item.
    pub parent: u32,
    pub children: Vec<u32>,
    pub text: String,
    pub image: i32,
    pub selected_image: i32,
    /// `cChildren`: `0`, `1`, or [`I_CHILDRENCALLBACK`].
    pub has_children: i32,
    pub lparam: u32,
    pub state: u32,
}

impl TreeItem {
    pub fn expanded(&self) -> bool {
        self.state & TVIS_EXPANDED != 0
    }

    /// Does the item get an expand box? Either it already has children,
    /// or the application said it will have some.
    pub fn expandable(&self) -> bool {
        !self.children.is_empty() || self.has_children != 0
    }
}

/// What a tree or list wants the application told, as the payload of a
/// `WM_NOTIFY` the dispatcher builds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ViewEvent {
    /// `TVN_ITEMEXPANDING` followed by `TVN_ITEMEXPANDED`. `action` is
    /// `TVE_EXPAND` or `TVE_COLLAPSE`.
    TreeExpand { item: u32, action: u32 },
    /// `TVN_SELCHANGED`. `by` is `TVC_BYMOUSE` (1), `TVC_BYKEYBOARD` (2)
    /// or `TVC_UNKNOWN` (0).
    TreeSelect { old: u32, new: u32, by: u32 },
    /// `NM_CLICK` on a tree.
    TreeClick,
    /// `NM_DBLCLK` / `NM_RETURN` — a second tap on the selected item, or
    /// Enter.
    TreeActivate { item: u32 },
    /// `LVN_ITEMCHANGED` for one item's state.
    ListChanged {
        item: i32,
        new_state: u32,
        old_state: u32,
    },
    /// `NM_CLICK` on a list.
    ListClick { item: i32 },
    /// `LVN_ITEMACTIVATE` — a second tap on the selected item, or Enter.
    ListActivate { item: i32 },
    /// `LVN_COLUMNCLICK` on a report view header.
    ColumnClick { column: i32 },
}

/// The model behind one `SysTreeView32`.
#[derive(Debug, Clone, Default)]
pub struct TreeData {
    pub items: HashMap<u32, TreeItem>,
    /// Top-level items in display order.
    pub roots: Vec<u32>,
    /// `TVGN_CARET` item, or `0`.
    pub selected: u32,
    /// First visible row.
    pub top: usize,
    pub image_list: u32,
    /// `(cx, cy)` of [`Self::image_list`], recorded when it is set so
    /// layout and hit testing do not need the image lists themselves.
    pub image_size: Option<(i32, i32)>,
    pub state_image_list: u32,
    pub indent: i32,
    next_handle: u32,
}

impl TreeData {
    /// First handle handed out. Kept well away from the `TVI_*` values
    /// and from zero, and odd-looking enough to spot in a trace.
    const HANDLE_BASE: u32 = 0x00E0_0010;

    pub fn new() -> Self {
        Self {
            indent: 16,
            next_handle: Self::HANDLE_BASE,
            ..Self::default()
        }
    }

    pub fn get(&self, handle: u32) -> Option<&TreeItem> {
        self.items.get(&handle)
    }

    pub fn get_mut(&mut self, handle: u32) -> Option<&mut TreeItem> {
        self.items.get_mut(&handle)
    }

    fn siblings(&self, parent: u32) -> &Vec<u32> {
        if parent == 0 {
            &self.roots
        } else {
            &self.items[&parent].children
        }
    }

    fn siblings_mut(&mut self, parent: u32) -> &mut Vec<u32> {
        if parent == 0 {
            &mut self.roots
        } else {
            &mut self.items.get_mut(&parent).unwrap().children
        }
    }

    /// `TVM_INSERTITEM`. `parent` is `0`/[`TVI_ROOT`] for a top-level
    /// item; `after` is an item handle or one of the `TVI_*` positions.
    /// Returns the new handle, or `0` if the parent does not exist.
    pub fn insert(&mut self, parent: u32, after: u32, mut item: TreeItem) -> u32 {
        let parent = if parent == TVI_ROOT { 0 } else { parent };
        if parent != 0 && !self.items.contains_key(&parent) {
            return 0;
        }
        let handle = self.next_handle;
        self.next_handle += 1;
        item.handle = handle;
        item.parent = parent;
        item.children.clear();
        let text = item.text.to_lowercase();
        self.items.insert(handle, item);
        let sort_pos = {
            let siblings = self.siblings(parent);
            siblings
                .iter()
                .position(|h| self.items[h].text.to_lowercase() > text)
                .unwrap_or(siblings.len())
        };
        let siblings = self.siblings_mut(parent);
        let at = match after {
            TVI_FIRST => 0,
            TVI_SORT => sort_pos,
            TVI_LAST | 0 => siblings.len(),
            other => siblings
                .iter()
                .position(|h| *h == other)
                .map(|i| i + 1)
                .unwrap_or(siblings.len()),
        };
        siblings.insert(at, handle);
        handle
    }

    /// `TVM_DELETEITEM`: drop an item and everything under it, or the
    /// whole tree for [`TVI_ROOT`]/`0`. Returns the handles removed.
    pub fn delete(&mut self, handle: u32) -> Vec<u32> {
        if handle == TVI_ROOT || handle == 0 {
            let all: Vec<u32> = self.items.keys().copied().collect();
            self.items.clear();
            self.roots.clear();
            self.selected = 0;
            self.top = 0;
            return all;
        }
        let Some(item) = self.items.get(&handle) else {
            return Vec::new();
        };
        let parent = item.parent;
        if parent == 0 || self.items.contains_key(&parent) {
            self.siblings_mut(parent).retain(|h| *h != handle);
        }
        let mut gone = Vec::new();
        let mut stack = vec![handle];
        while let Some(h) = stack.pop() {
            if let Some(it) = self.items.remove(&h) {
                stack.extend(it.children);
                gone.push(h);
            }
        }
        if gone.contains(&self.selected) {
            self.selected = 0;
        }
        gone
    }

    /// Change an item's expanded state. Returns the notification the
    /// change calls for: only the first expansion of an item notifies,
    /// as with `TVM_EXPAND` on a device, while a tap on the expand box
    /// always does (see [`Self::toggle_by_user`]).
    pub fn expand(&mut self, handle: u32, code: u32) -> Option<ViewEvent> {
        let item = self.items.get_mut(&handle)?;
        let was = item.expanded();
        let want = match code & 0x3 {
            TVE_COLLAPSE => false,
            TVE_EXPAND => true,
            TVE_TOGGLE => !was,
            _ => return None,
        };
        let first = want && item.state & TVIS_EXPANDEDONCE == 0;
        if want {
            item.state |= TVIS_EXPANDED | TVIS_EXPANDEDONCE;
        } else {
            item.state &= !TVIS_EXPANDED;
        }
        if code & TVE_COLLAPSERESET != 0 && !want {
            item.state &= !TVIS_EXPANDEDONCE;
            let kids = std::mem::take(&mut item.children);
            for kid in kids {
                let mut stack = vec![kid];
                while let Some(h) = stack.pop() {
                    if let Some(it) = self.items.remove(&h) {
                        stack.extend(it.children);
                    }
                }
            }
        }
        self.clamp_top();
        first.then_some(ViewEvent::TreeExpand {
            item: handle,
            action: TVE_EXPAND,
        })
    }

    /// The stylus hit an expand box: always tell the application, which
    /// is how a file browser fills a folder in on demand.
    pub fn toggle_by_user(&mut self, handle: u32) -> Option<ViewEvent> {
        let item = self.items.get_mut(&handle)?;
        let action = if item.expanded() {
            item.state &= !TVIS_EXPANDED;
            TVE_COLLAPSE
        } else {
            item.state |= TVIS_EXPANDED | TVIS_EXPANDEDONCE;
            TVE_EXPAND
        };
        self.clamp_top();
        Some(ViewEvent::TreeExpand {
            item: handle,
            action,
        })
    }

    /// `TVM_SELECTITEM(TVGN_CARET)`.
    pub fn select(&mut self, handle: u32, by: u32) -> Option<ViewEvent> {
        if handle != 0 && !self.items.contains_key(&handle) {
            return None;
        }
        let old = self.selected;
        if old == handle {
            return None;
        }
        if let Some(prev) = self.items.get_mut(&old) {
            prev.state &= !TVIS_SELECTED;
        }
        if let Some(it) = self.items.get_mut(&handle) {
            it.state |= TVIS_SELECTED;
        }
        self.selected = handle;
        // Selecting an item reveals it: its ancestors open.
        let mut p = self.items.get(&handle).map(|i| i.parent).unwrap_or(0);
        while let Some(it) = self.items.get_mut(&p) {
            it.state |= TVIS_EXPANDED | TVIS_EXPANDEDONCE;
            p = it.parent;
        }
        Some(ViewEvent::TreeSelect {
            old,
            new: handle,
            by,
        })
    }

    /// Rows currently shown, top to bottom, with their depth.
    pub fn visible_rows(&self) -> Vec<(u32, i32)> {
        let mut out = Vec::new();
        let mut stack: Vec<(u32, i32)> = self.roots.iter().rev().map(|h| (*h, 0)).collect();
        while let Some((h, depth)) = stack.pop() {
            let Some(it) = self.items.get(&h) else {
                continue;
            };
            out.push((h, depth));
            if it.expanded() {
                stack.extend(it.children.iter().rev().map(|c| (*c, depth + 1)));
            }
        }
        out
    }

    /// `TVM_GETNEXTITEM`.
    pub fn next_item(&self, code: u32, from: u32) -> u32 {
        const TVGN_ROOT: u32 = 0;
        const TVGN_NEXT: u32 = 1;
        const TVGN_PREVIOUS: u32 = 2;
        const TVGN_PARENT: u32 = 3;
        const TVGN_CHILD: u32 = 4;
        const TVGN_FIRSTVISIBLE: u32 = 5;
        const TVGN_NEXTVISIBLE: u32 = 6;
        const TVGN_PREVIOUSVISIBLE: u32 = 7;
        const TVGN_DROPHILITE: u32 = 8;
        const TVGN_CARET: u32 = 9;
        const TVGN_LASTVISIBLE: u32 = 10;
        match code {
            TVGN_ROOT => self.roots.first().copied().unwrap_or(0),
            TVGN_CARET => self.selected,
            TVGN_DROPHILITE => 0,
            TVGN_CHILD => {
                if from == 0 || from == TVI_ROOT {
                    self.roots.first().copied().unwrap_or(0)
                } else {
                    self.get(from)
                        .and_then(|i| i.children.first().copied())
                        .unwrap_or(0)
                }
            }
            TVGN_PARENT => self.get(from).map(|i| i.parent).unwrap_or(0),
            TVGN_NEXT | TVGN_PREVIOUS => {
                let Some(it) = self.get(from) else { return 0 };
                let sib = self.siblings(it.parent);
                let Some(i) = sib.iter().position(|h| *h == from) else {
                    return 0;
                };
                let j = if code == TVGN_NEXT {
                    i.checked_add(1)
                } else {
                    i.checked_sub(1)
                };
                j.and_then(|j| sib.get(j)).copied().unwrap_or(0)
            }
            TVGN_FIRSTVISIBLE => self.visible_rows().get(self.top).map(|r| r.0).unwrap_or(0),
            TVGN_LASTVISIBLE => self.visible_rows().last().map(|r| r.0).unwrap_or(0),
            TVGN_NEXTVISIBLE | TVGN_PREVIOUSVISIBLE => {
                let rows = self.visible_rows();
                let Some(i) = rows.iter().position(|r| r.0 == from) else {
                    return 0;
                };
                let j = if code == TVGN_NEXTVISIBLE {
                    i.checked_add(1)
                } else {
                    i.checked_sub(1)
                };
                j.and_then(|j| rows.get(j)).map(|r| r.0).unwrap_or(0)
            }
            _ => 0,
        }
    }

    /// `TVM_SORTCHILDREN`: case-insensitive by text, as the control does
    /// without a callback.
    pub fn sort_children(&mut self, parent: u32) {
        let parent = if parent == TVI_ROOT { 0 } else { parent };
        if parent != 0 && !self.items.contains_key(&parent) {
            return;
        }
        let mut kids = self.siblings(parent).clone();
        kids.sort_by_key(|h| self.items[h].text.to_lowercase());
        *self.siblings_mut(parent) = kids;
    }

    fn clamp_top(&mut self) {
        let n = self.visible_rows().len();
        if self.top >= n {
            self.top = n.saturating_sub(1);
        }
    }

    /// Scroll so `handle` is on screen, given `rows` that fit.
    pub fn ensure_visible(&mut self, handle: u32, rows_fit: usize) {
        let rows = self.visible_rows();
        let Some(i) = rows.iter().position(|r| r.0 == handle) else {
            return;
        };
        let fit = rows_fit.max(1);
        if i < self.top {
            self.top = i;
        } else if i >= self.top + fit {
            self.top = i + 1 - fit;
        }
    }
}

/// One column of a report view.
#[derive(Debug, Clone, Default)]
pub struct ListColumn {
    pub text: String,
    pub width: i32,
    /// `LVCFMT_LEFT` (0), `RIGHT` (1) or `CENTER` (2).
    pub fmt: u32,
}

/// One row of a list view: the item text plus one string per subitem.
#[derive(Debug, Clone, Default)]
pub struct ListItem {
    pub texts: Vec<String>,
    pub image: i32,
    pub lparam: u32,
    pub state: u32,
    pub indent: i32,
}

impl ListItem {
    pub fn text(&self, sub: usize) -> &str {
        self.texts.get(sub).map(String::as_str).unwrap_or("")
    }

    pub fn set_text(&mut self, sub: usize, text: String) {
        if self.texts.len() <= sub {
            self.texts.resize(sub + 1, String::new());
        }
        self.texts[sub] = text;
    }
}

/// The model behind one `SysListView32`.
#[derive(Debug, Clone, Default)]
pub struct ListData {
    pub items: Vec<ListItem>,
    pub columns: Vec<ListColumn>,
    pub top: usize,
    pub image_list: u32,
    pub small_image_list: u32,
    pub state_image_list: u32,
    /// `(cx, cy)` of the large and small image lists, recorded when set.
    pub large_size: Option<(i32, i32)>,
    pub small_size: Option<(i32, i32)>,
    pub ex_style: u32,
    /// Index the stylus last went down on, so a second tap on the same
    /// item activates it.
    pub focus: i32,
}

impl ListData {
    pub fn new() -> Self {
        Self {
            focus: -1,
            ..Self::default()
        }
    }

    /// `LVM_INSERTITEM`: returns the index the item landed at.
    pub fn insert(&mut self, at: i32, item: ListItem) -> i32 {
        let at = (at.max(0) as usize).min(self.items.len());
        self.items.insert(at, item);
        at as i32
    }

    pub fn delete(&mut self, at: i32) -> bool {
        if at < 0 || at as usize >= self.items.len() {
            return false;
        }
        self.items.remove(at as usize);
        let n = self.items.len();
        if self.top >= n {
            self.top = n.saturating_sub(1);
        }
        true
    }

    pub fn selected_count(&self) -> usize {
        self.items
            .iter()
            .filter(|i| i.state & LVIS_SELECTED != 0)
            .count()
    }

    /// `LVM_GETNEXTITEM`: the first item after `start` whose state has
    /// every bit of `flags` (only the state bits are modelled).
    pub fn next_item(&self, start: i32, flags: u32) -> i32 {
        let want = flags & (LVIS_SELECTED | LVIS_FOCUSED);
        let from = if start < 0 { 0 } else { start as usize + 1 };
        (from..self.items.len())
            .find(|&i| self.items[i].state & want == want)
            .map(|i| i as i32)
            .unwrap_or(-1)
    }

    /// Set the state bits in `mask` to `state` on one item, or on every
    /// item for `-1`. Returns one `LVN_ITEMCHANGED` per item that changed.
    pub fn set_state(&mut self, index: i32, state: u32, mask: u32) -> Vec<ViewEvent> {
        let range = if index < 0 {
            0..self.items.len()
        } else if (index as usize) < self.items.len() {
            index as usize..index as usize + 1
        } else {
            return Vec::new();
        };
        let mut events = Vec::new();
        for i in range {
            let old = self.items[i].state;
            let new = (old & !mask) | (state & mask);
            if new != old {
                self.items[i].state = new;
                events.push(ViewEvent::ListChanged {
                    item: i as i32,
                    new_state: new,
                    old_state: old,
                });
            }
        }
        events
    }

    /// Make `index` the one selected and focused item.
    pub fn select_only(&mut self, index: i32) -> Vec<ViewEvent> {
        let mut events = Vec::new();
        for i in 0..self.items.len() {
            let want = if i as i32 == index {
                LVIS_SELECTED | LVIS_FOCUSED
            } else {
                0
            };
            events.extend(self.set_state(i as i32, want, LVIS_SELECTED | LVIS_FOCUSED));
        }
        self.focus = index;
        events
    }
}

/// Geometry shared by painting and hit testing.
struct Frame {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
}

impl Frame {
    fn of(x: i32, y: i32, w: i32, h: i32, style: u32) -> Self {
        let inset = i32::from(style & WS_BORDER != 0);
        Self {
            x: x + inset,
            y: y + inset,
            w: (w - 2 * inset).max(0),
            h: (h - 2 * inset).max(0),
        }
    }
}

/// Height of one row: the font plus padding, or the icons if taller.
fn row_height(icon: Option<(i32, i32)>) -> i32 {
    (font::GLYPH_H + 6).max(icon.map(|(_, cy)| cy + 1).unwrap_or(0))
}

/// Draw image `index` of `list` at `(x, y)`, clipped to `right`/`bottom`.
#[allow(clippy::too_many_arguments)]
fn draw_image(
    surf: &mut Surface<'_>,
    lists: &HashMap<u32, ImageList>,
    list: u32,
    index: i32,
    x: i32,
    y: i32,
    right: i32,
    bottom: i32,
) -> i32 {
    let Some(il) = lists.get(&list) else { return 0 };
    let Some((pixels, clear)) = usize::try_from(index).ok().and_then(|i| il.images.get(i)) else {
        return il.cx as i32;
    };
    let (cx, cy) = (il.cx as i32, il.cy as i32);
    for j in 0..cy {
        if y + j >= bottom {
            break;
        }
        for i in 0..cx {
            if x + i >= right {
                break;
            }
            let at = (j * cx + i) as usize;
            if clear.get(at).copied().unwrap_or(false) {
                continue;
            }
            if let Some(&p) = pixels.get(at) {
                surf.put_pixel(x + i, y + j, p);
            }
        }
    }
    cx
}

fn draw_text_clipped(surf: &mut Surface<'_>, x: i32, y: i32, text: &str, color: u16, right: i32) {
    let room = (right - x).max(0);
    let n = (room / font::GLYPH_W) as usize;
    if n == 0 {
        return;
    }
    let shown: String = text.chars().take(n).collect();
    font::draw_str(surf, x, y, &shown, color);
}

/// Scroll strip on the right: a track with a thumb sized to the share of
/// rows on screen.
fn draw_scroll(surf: &mut Surface<'_>, f: &Frame, top: usize, fit: usize, total: usize) {
    let sx = f.x + f.w - SCROLL_W;
    surf.fill_rect(sx, f.y, SCROLL_W, f.h, FACE);
    surf.fill_rect(sx, f.y, 1, f.h, SHADOW);
    if total == 0 {
        return;
    }
    let th = ((f.h as i64 * fit as i64) / total as i64).clamp(6, f.h as i64) as i32;
    let ty = f.y + ((f.h - th) as i64 * top as i64 / (total - fit).max(1) as i64) as i32;
    surf.fill_rect(sx + 1, ty, SCROLL_W - 1, th, FACE);
    crate::controls::draw_edge(surf, sx + 1, ty, SCROLL_W - 1, th, LIGHT, SHADOW);
}

/// Where the stylus landed in a tree.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TreeHit {
    Nowhere,
    /// The expand box of this item.
    Button(u32),
    /// The icon or label of this item.
    Item(u32),
    /// The scroll strip; `true` is the lower half.
    Scroll(bool),
}

impl TreeData {
    fn layout(&self, f: &Frame) -> (i32, usize, bool) {
        let rh = row_height(self.image_size);
        let fit = (f.h / rh).max(1) as usize;
        let scroll = self.visible_rows().len() > fit;
        (rh, fit, scroll)
    }

    /// Indent of the first column, where the root expand boxes go.
    fn root_indent(&self, style: u32) -> i32 {
        if style & TVS_LINESATROOT != 0 {
            self.indent
        } else {
            0
        }
    }

    pub fn rows_fit(&self, h: i32, style: u32) -> usize {
        self.layout(&Frame::of(0, 0, 0, h, style)).1
    }

    /// Paint the tree into the control rectangle `(x, y, w, h)`.
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        surf: &mut Surface<'_>,
        lists: &HashMap<u32, ImageList>,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        style: u32,
    ) {
        if style & WS_BORDER != 0 {
            crate::controls::stroke(surf, x, y, w, h, TEXT);
        }
        let f = Frame::of(x, y, w, h, style);
        surf.fill_rect(f.x, f.y, f.w, f.h, WINDOW);
        let (rh, fit, scroll) = self.layout(&f);
        let right = f.x + f.w - if scroll { SCROLL_W } else { 0 };
        let rows = self.visible_rows();
        let base = f.x + 2 + self.root_indent(style);
        for (n, &(handle, depth)) in rows.iter().skip(self.top).take(fit).enumerate() {
            let item = &self.items[&handle];
            let ry = f.y + n as i32 * rh;
            let lx = base + depth * self.indent;
            if style & TVS_HASLINES != 0 && (depth > 0 || style & TVS_LINESATROOT != 0) {
                let mx = lx - self.indent / 2;
                let my = ry + rh / 2;
                for i in (mx..lx - 1).step_by(2) {
                    surf.put_pixel(i, my, LINE);
                }
                // Up to the item above, unless this is the very first
                // root; down to the next sibling, if there is one.
                let first_root = depth == 0 && self.roots.first() == Some(&handle);
                let has_next = self.next_item(1, handle) != 0;
                for j in (ry..ry + rh).step_by(2) {
                    if (j <= my && !first_root) || (j > my && has_next) {
                        surf.put_pixel(mx, j, LINE);
                    }
                }
                // Each ancestor with siblings still to come keeps its own
                // line running through this row.
                let mut anc = item.parent;
                let mut level = depth - 1;
                while let Some(a) = self.items.get(&anc) {
                    if level < 0 || (level == 0 && style & TVS_LINESATROOT == 0) {
                        break;
                    }
                    if self.next_item(1, anc) != 0 {
                        let ax = base + level * self.indent - self.indent / 2;
                        for j in (ry..ry + rh).step_by(2) {
                            surf.put_pixel(ax, j, LINE);
                        }
                    }
                    anc = a.parent;
                    level -= 1;
                }
            }
            if item.expandable()
                && style & TVS_HASBUTTONS != 0
                && (depth > 0 || style & TVS_LINESATROOT != 0)
            {
                let bx = lx - self.indent / 2 - 4;
                let by = ry + rh / 2 - 4;
                surf.fill_rect(bx, by, 9, 9, WINDOW);
                crate::controls::stroke(surf, bx, by, 9, 9, LINE);
                surf.fill_rect(bx + 2, by + 4, 5, 1, TEXT);
                if !item.expanded() {
                    surf.fill_rect(bx + 4, by + 2, 1, 5, TEXT);
                }
            }
            let selected = handle == self.selected;
            let image = if selected {
                item.selected_image
            } else {
                item.image
            };
            let mut tx = lx;
            if self.image_list != 0 {
                let iy = ry + (rh - self.image_size.map(|s| s.1).unwrap_or(0)) / 2;
                tx += draw_image(
                    surf,
                    lists,
                    self.image_list,
                    image,
                    lx,
                    iy,
                    right,
                    f.y + f.h,
                ) + 3;
            }
            let ty = ry + (rh - font::GLYPH_H) / 2;
            let tw = font::str_width(&item.text).min(right - tx - 2);
            let color = if selected {
                surf.fill_rect(tx - 1, ry + 1, tw + 3, rh - 2, HIGHLIGHT);
                HIGHLIGHT_TEXT
            } else {
                TEXT
            };
            draw_text_clipped(surf, tx + 1, ty, &item.text, color, right - 1);
            if item.state & TVIS_BOLD != 0 {
                draw_text_clipped(surf, tx + 2, ty, &item.text, color, right);
            }
        }
        if scroll {
            draw_scroll(surf, &f, self.top, fit, rows.len());
        }
    }

    /// What is under `(px, py)`, both in the same space as `x, y`.
    #[allow(clippy::too_many_arguments)]
    pub fn hit(&self, x: i32, y: i32, w: i32, h: i32, style: u32, px: i32, py: i32) -> TreeHit {
        let f = Frame::of(x, y, w, h, style);
        let (rh, fit, scroll) = self.layout(&f);
        if scroll && px >= f.x + f.w - SCROLL_W {
            return TreeHit::Scroll(py >= f.y + f.h / 2);
        }
        let row = (py - f.y) / rh;
        if py < f.y || row as usize >= fit {
            return TreeHit::Nowhere;
        }
        let rows = self.visible_rows();
        let Some(&(handle, depth)) = rows.get(self.top + row as usize) else {
            return TreeHit::Nowhere;
        };
        let lx = f.x + 2 + self.root_indent(style) + depth * self.indent;
        let item = &self.items[&handle];
        if item.expandable() && px < lx && px >= lx - self.indent {
            return TreeHit::Button(handle);
        }
        TreeHit::Item(handle)
    }

    /// `TVM_GETITEMRECT`: the row's rectangle, or just the label's when
    /// `text_only`, relative to the control's client area.
    pub fn item_rect(
        &self,
        w: i32,
        h: i32,
        style: u32,
        handle: u32,
        text_only: bool,
    ) -> Option<(i32, i32, i32, i32)> {
        let f = Frame::of(0, 0, w, h, style);
        let (rh, fit, _) = self.layout(&f);
        let rows = self.visible_rows();
        let i = rows.iter().position(|r| r.0 == handle)?;
        if i < self.top || i >= self.top + fit {
            return None;
        }
        let top = (i - self.top) as i32 * rh;
        if !text_only {
            return Some((0, top, f.w, top + rh));
        }
        let depth = rows[i].1;
        let mut lx = 2 + self.root_indent(style) + depth * self.indent;
        if let Some((cx, _)) = self.image_size {
            lx += cx + 3;
        }
        let tw = font::str_width(&self.items[&handle].text) + 3;
        Some((lx - 1, top, lx - 1 + tw, top + rh))
    }
}

/// Where the stylus landed in a list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ListHit {
    Nowhere,
    Item(i32),
    Header(i32),
    Scroll(bool),
}

impl ListData {
    fn view(style: u32) -> u32 {
        style & LVS_TYPEMASK
    }

    fn has_header(style: u32) -> bool {
        Self::view(style) == LVS_REPORT && style & LVS_NOCOLUMNHEADER == 0
    }

    /// Height of the header strip in report view.
    const HEADER_H: i32 = font::GLYPH_H + 6;

    /// Large-icon cell size.
    fn icon_cell(&self) -> (i32, i32) {
        let (cx, cy) = self.large_size.unwrap_or((32, 32));
        ((cx + 24).max(56), cy + font::GLYPH_H + 8)
    }

    fn small_list(&self) -> u32 {
        self.small_image_list
    }

    /// Row height, rows per page, columns per row (icon view), whether a
    /// scroll strip is needed, and the y of the first row.
    fn layout(&self, f: &Frame, style: u32) -> (i32, usize, usize, bool, i32) {
        let header = if Self::has_header(style) {
            Self::HEADER_H
        } else {
            0
        };
        let body = (f.h - header).max(1);
        if Self::view(style) == LVS_ICON {
            let (cw, ch) = self.icon_cell();
            let per_row = ((f.w - SCROLL_W) / cw).max(1) as usize;
            let rows_fit = (body / ch).max(1) as usize;
            let total_rows = self.items.len().div_ceil(per_row);
            return (ch, rows_fit, per_row, total_rows > rows_fit, f.y + header);
        }
        let rh = row_height(self.small_size);
        let fit = (body / rh).max(1) as usize;
        (rh, fit, 1, self.items.len() > fit, f.y + header)
    }

    /// Default width of a report column the application gave no width.
    fn column_width(&self, i: usize, w: i32) -> i32 {
        match self.columns.get(i) {
            Some(c) if c.width > 0 => c.width,
            _ => w,
        }
    }

    /// Paint the list into the control rectangle `(x, y, w, h)`.
    #[allow(clippy::too_many_arguments)]
    pub fn render(
        &self,
        surf: &mut Surface<'_>,
        lists: &HashMap<u32, ImageList>,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        style: u32,
    ) {
        if style & WS_BORDER != 0 {
            crate::controls::stroke(surf, x, y, w, h, TEXT);
        }
        let f = Frame::of(x, y, w, h, style);
        surf.fill_rect(f.x, f.y, f.w, f.h, WINDOW);
        let (rh, fit, per_row, scroll, body_y) = self.layout(&f, style);
        let right = f.x + f.w - if scroll { SCROLL_W } else { 0 };
        let bottom = f.y + f.h;
        if Self::has_header(style) {
            let mut cx = f.x;
            for (i, col) in self.columns.iter().enumerate() {
                let cw = self.column_width(i, f.w);
                surf.fill_rect(cx, f.y, cw, Self::HEADER_H, FACE);
                crate::controls::draw_edge(surf, cx, f.y, cw, Self::HEADER_H, LIGHT, SHADOW);
                draw_text_clipped(
                    surf,
                    cx + 3,
                    f.y + 3,
                    &col.text,
                    TEXT,
                    (cx + cw - 2).min(right),
                );
                cx += cw;
            }
            if cx < right {
                surf.fill_rect(cx, f.y, right - cx, Self::HEADER_H, FACE);
                crate::controls::draw_edge(
                    surf,
                    cx,
                    f.y,
                    right - cx,
                    Self::HEADER_H,
                    LIGHT,
                    SHADOW,
                );
            }
        }
        if Self::view(style) == LVS_ICON {
            let (cw, ch) = self.icon_cell();
            let (icx, icy) = self.large_size.unwrap_or((32, 32));
            for (n, item) in self
                .items
                .iter()
                .enumerate()
                .skip(self.top * per_row)
                .take(fit * per_row)
            {
                let k = n - self.top * per_row;
                let cx = f.x + (k % per_row) as i32 * cw;
                let cy = body_y + (k / per_row) as i32 * ch;
                draw_image(
                    surf,
                    lists,
                    self.image_list,
                    item.image,
                    cx + (cw - icx) / 2,
                    cy + 2,
                    right,
                    bottom,
                );
                let label = item.text(0);
                let max = ((cw - 2) / font::GLYPH_W) as usize;
                let shown: String = label.chars().take(max).collect();
                let tw = font::str_width(&shown);
                let tx = cx + (cw - tw) / 2;
                let ty = cy + icy + 4;
                let color = if item.state & LVIS_SELECTED != 0 {
                    surf.fill_rect(tx - 1, ty - 1, tw + 2, font::GLYPH_H + 2, HIGHLIGHT);
                    HIGHLIGHT_TEXT
                } else {
                    TEXT
                };
                draw_text_clipped(surf, tx, ty, &shown, color, right);
            }
        } else {
            let report = Self::view(style) == LVS_REPORT;
            let full_row = self.ex_style & 0x20 != 0; // LVS_EX_FULLROWSELECT
            for (n, item) in self.items.iter().enumerate().skip(self.top).take(fit) {
                let ry = body_y + (n - self.top) as i32 * rh;
                let selected = item.state & LVIS_SELECTED != 0;
                let mut cx = f.x + 2 + item.indent * 16;
                let col0_right = if report && !self.columns.is_empty() {
                    (f.x + self.column_width(0, f.w)).min(right)
                } else {
                    right
                };
                if self.small_list() != 0 {
                    let iy = ry + (rh - self.small_size.map(|s| s.1).unwrap_or(0)) / 2;
                    cx += draw_image(
                        surf,
                        lists,
                        self.small_list(),
                        item.image,
                        cx,
                        iy,
                        col0_right,
                        bottom,
                    ) + 2;
                }
                let ty = ry + (rh - font::GLYPH_H) / 2;
                let label = item.text(0);
                let tw = font::str_width(label).min(col0_right - cx - 2);
                if selected {
                    let (hx, hw) = if full_row && report {
                        (cx - 1, right - cx + 1)
                    } else {
                        (cx - 1, tw + 3)
                    };
                    surf.fill_rect(hx, ry + 1, hw, rh - 2, HIGHLIGHT);
                }
                let color = if selected { HIGHLIGHT_TEXT } else { TEXT };
                draw_text_clipped(surf, cx + 1, ty, label, color, col0_right - 1);
                if report {
                    let sub_color = if selected && full_row {
                        HIGHLIGHT_TEXT
                    } else {
                        TEXT
                    };
                    let mut sx = f.x + self.column_width(0, f.w);
                    for (i, col) in self.columns.iter().enumerate().skip(1) {
                        let cw = self.column_width(i, f.w);
                        if sx >= right {
                            break;
                        }
                        let text = item.text(i);
                        let tw = font::str_width(text).min(cw - 6);
                        let tx = match col.fmt & 3 {
                            1 => sx + cw - 3 - tw,
                            2 => sx + (cw - tw) / 2,
                            _ => sx + 3,
                        };
                        draw_text_clipped(surf, tx, ty, text, sub_color, (sx + cw - 2).min(right));
                        sx += cw;
                    }
                }
            }
        }
        if scroll {
            let total = if Self::view(style) == LVS_ICON {
                self.items.len().div_ceil(per_row)
            } else {
                self.items.len()
            };
            let sf = Frame {
                x: f.x,
                y: body_y,
                w: f.w,
                h: (f.y + f.h - body_y).max(1),
            };
            draw_scroll(surf, &sf, self.top, fit, total);
        }
    }

    pub fn rows_fit(&self, w: i32, h: i32, style: u32) -> usize {
        self.layout(&Frame::of(0, 0, w, h, style), style).1
    }

    /// What is under `(px, py)`.
    #[allow(clippy::too_many_arguments)]
    pub fn hit(&self, x: i32, y: i32, w: i32, h: i32, style: u32, px: i32, py: i32) -> ListHit {
        let f = Frame::of(x, y, w, h, style);
        let (rh, fit, per_row, scroll, body_y) = self.layout(&f, style);
        if Self::has_header(style) && py < body_y {
            let mut cx = f.x;
            for i in 0..self.columns.len() {
                let cw = self.column_width(i, f.w);
                if px >= cx && px < cx + cw {
                    return ListHit::Header(i as i32);
                }
                cx += cw;
            }
            return ListHit::Nowhere;
        }
        if scroll && px >= f.x + f.w - SCROLL_W {
            return ListHit::Scroll(py >= body_y + (f.y + f.h - body_y) / 2);
        }
        if py < body_y {
            return ListHit::Nowhere;
        }
        let row = ((py - body_y) / rh) as usize;
        if row >= fit {
            return ListHit::Nowhere;
        }
        let index = if Self::view(style) == LVS_ICON {
            let (cw, _) = self.icon_cell();
            let col = ((px - f.x) / cw) as usize;
            if col >= per_row {
                return ListHit::Nowhere;
            }
            (self.top + row) * per_row + col
        } else {
            self.top + row
        };
        if index < self.items.len() {
            ListHit::Item(index as i32)
        } else {
            ListHit::Nowhere
        }
    }

    /// `LVM_GETITEMRECT` (`LVIR_BOUNDS`), relative to the client area.
    pub fn item_rect(
        &self,
        w: i32,
        h: i32,
        style: u32,
        index: i32,
    ) -> Option<(i32, i32, i32, i32)> {
        let f = Frame::of(0, 0, w, h, style);
        let (rh, _, per_row, _, body_y) = self.layout(&f, style);
        let i = usize::try_from(index)
            .ok()
            .filter(|&i| i < self.items.len())?;
        if Self::view(style) == LVS_ICON {
            let (cw, ch) = self.icon_cell();
            let row = (i / per_row) as i32 - self.top as i32;
            let col = (i % per_row) as i32;
            return Some((
                col * cw,
                body_y + row * ch,
                col * cw + cw,
                body_y + row * ch + ch,
            ));
        }
        let row = i as i32 - self.top as i32;
        Some((0, body_y + row * rh, f.w, body_y + row * rh + rh))
    }

    pub fn ensure_visible(&mut self, index: i32, fit: usize) {
        let Ok(i) = usize::try_from(index) else {
            return;
        };
        let fit = fit.max(1);
        if i < self.top {
            self.top = i;
        } else if i >= self.top + fit {
            self.top = i + 1 - fit;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn item(text: &str) -> TreeItem {
        TreeItem {
            text: text.to_string(),
            ..TreeItem::default()
        }
    }

    #[test]
    fn tree_inserts_in_order_and_sorts_on_request() {
        let mut t = TreeData::new();
        let root = t.insert(TVI_ROOT, TVI_LAST, item("My Device"));
        let b = t.insert(root, TVI_LAST, item("Windows"));
        let a = t.insert(root, TVI_SORT, item("My Documents"));
        let c = t.insert(root, TVI_FIRST, item("Zip"));
        assert_eq!(t.get(root).unwrap().children, vec![c, a, b]);
        t.sort_children(root);
        assert_eq!(t.get(root).unwrap().children, vec![a, b, c]);
        assert_eq!(t.next_item(4, root), a);
        assert_eq!(t.next_item(1, a), b);
        assert_eq!(t.next_item(3, b), root);
    }

    #[test]
    fn first_expand_notifies_and_shows_children() {
        let mut t = TreeData::new();
        let root = t.insert(
            TVI_ROOT,
            TVI_LAST,
            TreeItem {
                has_children: 1,
                ..item("My Device")
            },
        );
        assert_eq!(t.visible_rows().len(), 1);
        assert_eq!(
            t.expand(root, TVE_EXPAND),
            Some(ViewEvent::TreeExpand {
                item: root,
                action: TVE_EXPAND
            })
        );
        t.insert(root, TVI_LAST, item("Windows"));
        assert_eq!(t.visible_rows(), vec![(root, 0), (root + 1, 1)]);
        // Expanded once already: a second TVM_EXPAND is silent.
        t.expand(root, TVE_COLLAPSE);
        assert_eq!(t.expand(root, TVE_EXPAND), None);
        // A tap on the box always tells the application.
        assert!(matches!(
            t.toggle_by_user(root),
            Some(ViewEvent::TreeExpand {
                action: TVE_COLLAPSE,
                ..
            })
        ));
    }

    #[test]
    fn deleting_a_node_drops_its_subtree_and_selection() {
        let mut t = TreeData::new();
        let root = t.insert(TVI_ROOT, TVI_LAST, item("r"));
        let kid = t.insert(root, TVI_LAST, item("k"));
        let grand = t.insert(kid, TVI_LAST, item("g"));
        t.select(grand, 0);
        assert!(
            t.get(root).unwrap().expanded(),
            "selecting reveals the item"
        );
        assert_eq!(t.delete(kid).len(), 2);
        assert_eq!(t.selected, 0);
        assert!(t.get(root).unwrap().children.is_empty());
    }

    #[test]
    fn tree_hit_tells_the_box_from_the_label() {
        let mut t = TreeData::new();
        let style = TVS_HASBUTTONS | TVS_HASLINES | TVS_LINESATROOT;
        let root = t.insert(
            TVI_ROOT,
            TVI_LAST,
            TreeItem {
                has_children: 1,
                ..item("r")
            },
        );
        // Row 0, box sits in the root indent, label right of it.
        assert_eq!(t.hit(0, 0, 100, 100, style, 8, 5), TreeHit::Button(root));
        assert_eq!(t.hit(0, 0, 100, 100, style, 30, 5), TreeHit::Item(root));
        assert_eq!(t.hit(0, 0, 100, 100, style, 30, 60), TreeHit::Nowhere);
    }

    #[test]
    fn list_selection_reports_each_change() {
        let mut l = ListData::new();
        for name in ["a", "b", "c"] {
            let n = l.items.len() as i32;
            l.insert(
                n,
                ListItem {
                    texts: vec![name.into()],
                    ..ListItem::default()
                },
            );
        }
        let ev = l.select_only(1);
        assert_eq!(ev.len(), 1);
        assert_eq!(l.next_item(-1, LVIS_SELECTED), 1);
        let ev = l.select_only(2);
        assert_eq!(ev.len(), 2, "old one cleared, new one set");
        assert_eq!(l.selected_count(), 1);
    }

    #[test]
    fn report_view_hit_test_finds_rows_and_headers() {
        let mut l = ListData::new();
        l.columns.push(ListColumn {
            text: "Name".into(),
            width: 80,
            fmt: 0,
        });
        l.columns.push(ListColumn {
            text: "Size".into(),
            width: 40,
            fmt: 1,
        });
        for i in 0..3 {
            l.insert(
                i,
                ListItem {
                    texts: vec![format!("f{i}"), "1K".into()],
                    ..ListItem::default()
                },
            );
        }
        let style = LVS_REPORT;
        assert_eq!(l.hit(0, 0, 200, 100, style, 90, 3), ListHit::Header(1));
        let rh = font::GLYPH_H + 6;
        let body = ListData::HEADER_H;
        assert_eq!(
            l.hit(0, 0, 200, 100, style, 10, body + rh + 2),
            ListHit::Item(1)
        );
        assert_eq!(
            l.hit(0, 0, 200, 100, style, 10, body + 3 * rh + 2),
            ListHit::Nowhere
        );
    }
}

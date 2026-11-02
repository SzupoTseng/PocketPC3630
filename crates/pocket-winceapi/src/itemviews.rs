//! Messages to and notifications from the built-in tree and list views
//! (`SysTreeView32`, `SysListView32`).
//!
//! The item models live in [`pocket_kernel::itemview`]; this module reads
//! and writes the `TVITEMW` / `LVITEMW` / `LVCOLUMNW` structures the guest
//! passes, and turns the views' [`ViewEvent`]s into `WM_NOTIFY` messages
//! whose `NMTREEVIEWW` / `NMLISTVIEW` payload lives in guest memory.

use std::cell::RefCell;

use pocket_kernel::controls::ControlClass;
use pocket_kernel::itemview::{
    ListColumn, ListItem, TreeItem, ViewEvent, LVIS_FOCUSED, LVIS_SELECTED, TVE_COLLAPSE,
    TVIS_EXPANDED, TVIS_EXPANDEDONCE,
};
use pocket_kernel::KernelError;

use crate::CallCtx;

const WM_NOTIFY: u32 = 0x004E;
/// `LPSTR_TEXTCALLBACKW`: the application supplies the text on demand.
const TEXT_CALLBACK: u32 = 0xFFFF_FFFF;

/// `TVIF_*` / `LVIF_*` mask bits.
const TVIF_TEXT: u32 = 0x0001;
const TVIF_IMAGE: u32 = 0x0002;
const TVIF_PARAM: u32 = 0x0004;
const TVIF_STATE: u32 = 0x0008;
const TVIF_HANDLE: u32 = 0x0010;
const TVIF_SELECTEDIMAGE: u32 = 0x0020;
const TVIF_CHILDREN: u32 = 0x0040;
const LVIF_TEXT: u32 = 0x0001;
const LVIF_IMAGE: u32 = 0x0002;
const LVIF_PARAM: u32 = 0x0004;
const LVIF_STATE: u32 = 0x0008;
const LVIF_INDENT: u32 = 0x0010;
const LVCF_FMT: u32 = 0x0001;
const LVCF_WIDTH: u32 = 0x0002;
const LVCF_TEXT: u32 = 0x0004;

fn rd(ctx: &mut CallCtx<'_>, at: u32) -> Result<u32, KernelError> {
    let b = ctx.cpu.read_mem(at, 4)?;
    Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
}

fn wr(ctx: &mut CallCtx<'_>, at: u32, v: u32) -> Result<(), KernelError> {
    ctx.cpu.write_mem(at, &v.to_le_bytes())?;
    Ok(())
}

/// Read a guest string, or `None` for a null or callback pointer.
fn read_text(ctx: &mut CallCtx<'_>, p: u32) -> Option<String> {
    if p == 0 || p == TEXT_CALLBACK {
        return None;
    }
    let units = crate::coredll::read_wstr(ctx, p, 1024).ok()?;
    Some(
        String::from_utf16_lossy(&units)
            .trim_end_matches('\0')
            .to_string(),
    )
}

/// Copy `text` into a guest buffer of `room` UTF-16 units, terminated.
/// Returns the number of units copied, not counting the terminator.
fn write_text(ctx: &mut CallCtx<'_>, p: u32, room: u32, text: &str) -> Result<u32, KernelError> {
    if p == 0 || p == TEXT_CALLBACK || room == 0 {
        return Ok(0);
    }
    let units: Vec<u16> = text.encode_utf16().take(room as usize - 1).collect();
    let mut bytes: Vec<u8> = units.iter().flat_map(|u| u.to_le_bytes()).collect();
    bytes.extend_from_slice(&[0, 0]);
    ctx.cpu.write_mem(p, &bytes)?;
    Ok(units.len() as u32)
}

fn image_size(ctx: &CallCtx<'_>, list: u32) -> Option<(i32, i32)> {
    ctx.kernel
        .image_lists
        .get(&list)
        .map(|il| (il.cx as i32, il.cy as i32))
}

/// Handle a message sent to a tree or list view. `None` means `hwnd` is
/// not a view, and the caller answers as for any other control.
pub(crate) fn view_message(
    ctx: &mut CallCtx<'_>,
    hwnd: u32,
    msg: u32,
    wparam: u32,
    lparam: u32,
) -> Result<Option<u32>, KernelError> {
    let class = match ctx.kernel.controls.get(hwnd) {
        Some(c) => c.class,
        None => return Ok(None),
    };
    let r = match class {
        ControlClass::TreeView => tree_message(ctx, hwnd, msg, wparam, lparam)?,
        ControlClass::ListView => list_message(ctx, hwnd, msg, wparam, lparam)?,
        _ => return Ok(None),
    };
    flush_notifications(ctx);
    Ok(Some(r))
}

fn tree_message(
    ctx: &mut CallCtx<'_>,
    hwnd: u32,
    msg: u32,
    wparam: u32,
    lparam: u32,
) -> Result<u32, KernelError> {
    const TVM_DELETEITEM: u32 = 0x1101;
    const TVM_EXPAND: u32 = 0x1102;
    const TVM_GETITEMRECT: u32 = 0x1104;
    const TVM_GETCOUNT: u32 = 0x1105;
    const TVM_GETINDENT: u32 = 0x1106;
    const TVM_SETINDENT: u32 = 0x1107;
    const TVM_GETIMAGELIST: u32 = 0x1108;
    const TVM_SETIMAGELIST: u32 = 0x1109;
    const TVM_GETNEXTITEM: u32 = 0x110A;
    const TVM_SELECTITEM: u32 = 0x110B;
    const TVM_GETVISIBLECOUNT: u32 = 0x1110;
    const TVM_HITTEST: u32 = 0x1111;
    const TVM_SORTCHILDREN: u32 = 0x1113;
    const TVM_ENSUREVISIBLE: u32 = 0x1114;
    const TVM_SORTCHILDRENCB: u32 = 0x1115;
    const TVM_SETITEMHEIGHT: u32 = 0x111B;
    const TVM_GETITEMHEIGHT: u32 = 0x111C;
    const TVM_GETITEMSTATE: u32 = 0x1127;
    const TVM_INSERTITEMW: u32 = 0x1132;
    const TVM_GETITEMW: u32 = 0x113E;
    const TVM_SETITEMW: u32 = 0x113F;
    const TVGN_FIRSTVISIBLE: u32 = 5;
    const TVGN_CARET: u32 = 9;

    let child = ctx.kernel.controls.get(hwnd).cloned().unwrap();
    let (w, h, style) = (child.w, child.h, child.style);
    let fit = child.rows_fit();
    let r = match msg {
        TVM_INSERTITEMW if lparam != 0 => {
            let parent = rd(ctx, lparam)?;
            let after = rd(ctx, lparam + 4)?;
            let it = lparam + 8;
            let mask = rd(ctx, it)?;
            let mut item = TreeItem::default();
            if mask & TVIF_TEXT != 0 {
                let p = rd(ctx, it + 16)?;
                item.text = read_text(ctx, p).unwrap_or_default();
            }
            if mask & TVIF_IMAGE != 0 {
                item.image = rd(ctx, it + 24)? as i32;
            }
            if mask & TVIF_SELECTEDIMAGE != 0 {
                item.selected_image = rd(ctx, it + 28)? as i32;
            } else {
                item.selected_image = item.image;
            }
            if mask & TVIF_CHILDREN != 0 {
                item.has_children = rd(ctx, it + 32)? as i32;
            }
            if mask & TVIF_PARAM != 0 {
                item.lparam = rd(ctx, it + 36)?;
            }
            if mask & TVIF_STATE != 0 {
                let state = rd(ctx, it + 8)?;
                let smask = rd(ctx, it + 12)?;
                item.state = state & smask & !(TVIS_EXPANDEDONCE);
            }
            let tree = tree_of(ctx, hwnd);
            let handle = tree.insert(parent, after, item);
            log::debug!("TVM_INSERTITEM(parent=0x{parent:08x}) -> 0x{handle:08x}");
            handle
        }
        TVM_DELETEITEM => u32::from(!tree_of(ctx, hwnd).delete(lparam).is_empty() || lparam == 0),
        TVM_EXPAND => {
            let tree = tree_of(ctx, hwnd);
            let exists = tree.get(lparam).is_some();
            if let Some(ev) = tree.expand(lparam, wparam) {
                ctx.kernel.controls.notify(hwnd, ev);
            }
            u32::from(exists)
        }
        TVM_GETITEMRECT if lparam != 0 => {
            let handle = rd(ctx, lparam)?;
            let tree = tree_of(ctx, hwnd);
            match tree.item_rect(w, h, style, handle, wparam != 0) {
                Some((l, t, r, b)) => {
                    for (i, v) in [l, t, r, b].into_iter().enumerate() {
                        wr(ctx, lparam + 4 * i as u32, v as u32)?;
                    }
                    1
                }
                None => 0,
            }
        }
        TVM_GETCOUNT => tree_of(ctx, hwnd).items.len() as u32,
        TVM_GETINDENT => tree_of(ctx, hwnd).indent as u32,
        TVM_SETINDENT => {
            tree_of(ctx, hwnd).indent = (wparam as i32).clamp(8, 64);
            1
        }
        TVM_GETIMAGELIST => {
            let tree = tree_of(ctx, hwnd);
            if wparam == 2 { tree.state_image_list } else { tree.image_list }
        }
        TVM_SETIMAGELIST => {
            let size = image_size(ctx, lparam);
            let tree = tree_of(ctx, hwnd);
            if wparam == 2 {
                std::mem::replace(&mut tree.state_image_list, lparam)
            } else {
                tree.image_size = size;
                std::mem::replace(&mut tree.image_list, lparam)
            }
        }
        TVM_GETNEXTITEM => tree_of(ctx, hwnd).next_item(wparam, lparam),
        TVM_SELECTITEM => {
            let tree = tree_of(ctx, hwnd);
            match wparam {
                TVGN_CARET => {
                    let exists = lparam == 0 || tree.get(lparam).is_some();
                    let ev = tree.select(lparam, 0);
                    tree.ensure_visible(lparam, fit);
                    if let Some(ev) = ev {
                        ctx.kernel.controls.notify(hwnd, ev);
                    }
                    u32::from(exists)
                }
                TVGN_FIRSTVISIBLE => {
                    let rows = tree.visible_rows();
                    match rows.iter().position(|r| r.0 == lparam) {
                        Some(i) => {
                            tree.top = i;
                            1
                        }
                        None => 0,
                    }
                }
                _ => 1,
            }
        }
        TVM_GETVISIBLECOUNT => fit as u32,
        TVM_HITTEST if lparam != 0 => {
            let px = rd(ctx, lparam)? as i32;
            let py = rd(ctx, lparam + 4)? as i32;
            let tree = tree_of(ctx, hwnd);
            let (flags, item) = match tree.hit(0, 0, w, h, style, px, py) {
                pocket_kernel::itemview::TreeHit::Button(i) => (0x0010, i),
                pocket_kernel::itemview::TreeHit::Item(i) => (0x0004, i),
                _ => (0x0001, 0),
            };
            wr(ctx, lparam + 8, flags)?;
            wr(ctx, lparam + 12, item)?;
            item
        }
        TVM_SORTCHILDREN | TVM_SORTCHILDRENCB => {
            let parent = if msg == TVM_SORTCHILDRENCB && lparam != 0 {
                rd(ctx, lparam)?
            } else {
                lparam
            };
            tree_of(ctx, hwnd).sort_children(parent);
            1
        }
        TVM_ENSUREVISIBLE => {
            let tree = tree_of(ctx, hwnd);
            let mut p = tree.get(lparam).map(|i| i.parent).unwrap_or(0);
            while let Some(it) = tree.get_mut(p) {
                it.state |= TVIS_EXPANDED | TVIS_EXPANDEDONCE;
                p = it.parent;
            }
            tree.ensure_visible(lparam, fit);
            1
        }
        TVM_GETITEMHEIGHT | TVM_SETITEMHEIGHT => {
            let tree = tree_of(ctx, hwnd);
            (pocket_kernel::font::GLYPH_H + 6).max(tree.image_size.map(|s| s.1 + 1).unwrap_or(0)) as u32
        }
        TVM_GETITEMSTATE => tree_of(ctx, hwnd)
            .get(wparam)
            .map(|i| i.state & lparam)
            .unwrap_or(0),
        TVM_GETITEMW if lparam != 0 => {
            let mask = rd(ctx, lparam)?;
            let handle = rd(ctx, lparam + 4)?;
            let Some(item) = tree_of(ctx, hwnd).get(handle).cloned() else {
                return Ok(0);
            };
            if mask & TVIF_STATE != 0 {
                let smask = rd(ctx, lparam + 12)?;
                wr(ctx, lparam + 8, item.state & smask)?;
            }
            if mask & TVIF_TEXT != 0 {
                let p = rd(ctx, lparam + 16)?;
                let room = rd(ctx, lparam + 20)?;
                write_text(ctx, p, room, &item.text)?;
            }
            if mask & TVIF_IMAGE != 0 {
                wr(ctx, lparam + 24, item.image as u32)?;
            }
            if mask & TVIF_SELECTEDIMAGE != 0 {
                wr(ctx, lparam + 28, item.selected_image as u32)?;
            }
            if mask & TVIF_CHILDREN != 0 {
                let n = if item.children.is_empty() { item.has_children } else { 1 };
                wr(ctx, lparam + 32, n as u32)?;
            }
            if mask & TVIF_PARAM != 0 {
                wr(ctx, lparam + 36, item.lparam)?;
            }
            1
        }
        TVM_SETITEMW if lparam != 0 => {
            let mask = rd(ctx, lparam)?;
            let handle = rd(ctx, lparam + 4)?;
            let state = rd(ctx, lparam + 8)?;
            let smask = rd(ctx, lparam + 12)?;
            let text = if mask & TVIF_TEXT != 0 {
                let p = rd(ctx, lparam + 16)?;
                read_text(ctx, p)
            } else {
                None
            };
            let image = rd(ctx, lparam + 24)? as i32;
            let selected_image = rd(ctx, lparam + 28)? as i32;
            let children = rd(ctx, lparam + 32)? as i32;
            let param = rd(ctx, lparam + 36)?;
            let tree = tree_of(ctx, hwnd);
            let Some(item) = tree.get_mut(handle) else { return Ok(0) };
            if let Some(text) = text {
                item.text = text;
            }
            if mask & TVIF_IMAGE != 0 {
                item.image = image;
            }
            if mask & TVIF_SELECTEDIMAGE != 0 {
                item.selected_image = selected_image;
            }
            if mask & TVIF_CHILDREN != 0 {
                item.has_children = children;
            }
            if mask & TVIF_PARAM != 0 {
                item.lparam = param;
            }
            if mask & TVIF_STATE != 0 {
                item.state = (item.state & !smask) | (state & smask);
            }
            1
        }
        _ => {
            log::debug!("tree view 0x{hwnd:08x} ignoring message 0x{msg:04x}");
            0
        }
    };
    let _ = TVIF_HANDLE;
    Ok(r)
}

fn tree_of<'a>(ctx: &'a mut CallCtx<'_>, hwnd: u32) -> &'a mut pocket_kernel::itemview::TreeData {
    ctx.kernel
        .controls
        .get_mut(hwnd)
        .and_then(|c| c.tree.as_deref_mut())
        .expect("tree view without a tree")
}

fn list_of<'a>(ctx: &'a mut CallCtx<'_>, hwnd: u32) -> &'a mut pocket_kernel::itemview::ListData {
    ctx.kernel
        .controls
        .get_mut(hwnd)
        .and_then(|c| c.list.as_deref_mut())
        .expect("list view without a list")
}

fn list_message(
    ctx: &mut CallCtx<'_>,
    hwnd: u32,
    msg: u32,
    wparam: u32,
    lparam: u32,
) -> Result<u32, KernelError> {
    const LVM_SETBKCOLOR: u32 = 0x1001;
    const LVM_GETIMAGELIST: u32 = 0x1002;
    const LVM_SETIMAGELIST: u32 = 0x1003;
    const LVM_GETITEMCOUNT: u32 = 0x1004;
    const LVM_DELETEITEM: u32 = 0x1008;
    const LVM_DELETEALLITEMS: u32 = 0x1009;
    const LVM_GETNEXTITEM: u32 = 0x100C;
    const LVM_GETITEMRECT: u32 = 0x100E;
    const LVM_HITTEST: u32 = 0x1012;
    const LVM_ENSUREVISIBLE: u32 = 0x1013;
    const LVM_SCROLL: u32 = 0x1014;
    const LVM_REDRAWITEMS: u32 = 0x1015;
    const LVM_ARRANGE: u32 = 0x1016;
    const LVM_DELETECOLUMN: u32 = 0x101C;
    const LVM_GETCOLUMNWIDTH: u32 = 0x101D;
    const LVM_SETCOLUMNWIDTH: u32 = 0x101E;
    const LVM_GETHEADER: u32 = 0x101F;
    const LVM_SETTEXTCOLOR: u32 = 0x1024;
    const LVM_SETTEXTBKCOLOR: u32 = 0x1026;
    const LVM_GETTOPINDEX: u32 = 0x1027;
    const LVM_GETCOUNTPERPAGE: u32 = 0x1028;
    const LVM_UPDATE: u32 = 0x102A;
    const LVM_SETITEMSTATE: u32 = 0x102B;
    const LVM_GETITEMSTATE: u32 = 0x102C;
    const LVM_SETITEMCOUNT: u32 = 0x102F;
    const LVM_SORTITEMS: u32 = 0x1030;
    const LVM_GETSELECTEDCOUNT: u32 = 0x1032;
    const LVM_SETEXTENDEDLISTVIEWSTYLE: u32 = 0x1036;
    const LVM_GETEXTENDEDLISTVIEWSTYLE: u32 = 0x1037;
    const LVM_GETSELECTIONMARK: u32 = 0x1042;
    const LVM_SETSELECTIONMARK: u32 = 0x1043;
    const LVM_GETITEMW: u32 = 0x104B;
    const LVM_SETITEMW: u32 = 0x104C;
    const LVM_INSERTITEMW: u32 = 0x104D;
    const LVM_FINDITEMW: u32 = 0x1053;
    const LVM_GETSTRINGWIDTHW: u32 = 0x1057;
    const LVM_GETCOLUMNW: u32 = 0x105F;
    const LVM_SETCOLUMNW: u32 = 0x1060;
    const LVM_INSERTCOLUMNW: u32 = 0x1061;
    const LVM_GETITEMTEXTW: u32 = 0x1073;
    const LVM_SETITEMTEXTW: u32 = 0x1074;

    let child = ctx.kernel.controls.get(hwnd).cloned().unwrap();
    let (w, h, style) = (child.w, child.h, child.style);
    let fit = child.rows_fit();
    let r = match msg {
        LVM_GETIMAGELIST => {
            let l = list_of(ctx, hwnd);
            match wparam {
                1 => l.small_image_list,
                2 => l.state_image_list,
                _ => l.image_list,
            }
        }
        LVM_SETIMAGELIST => {
            let size = image_size(ctx, lparam);
            let l = list_of(ctx, hwnd);
            match wparam {
                1 => {
                    l.small_size = size;
                    std::mem::replace(&mut l.small_image_list, lparam)
                }
                2 => std::mem::replace(&mut l.state_image_list, lparam),
                _ => {
                    l.large_size = size;
                    std::mem::replace(&mut l.image_list, lparam)
                }
            }
        }
        LVM_GETITEMCOUNT => list_of(ctx, hwnd).items.len() as u32,
        LVM_DELETEITEM => u32::from(list_of(ctx, hwnd).delete(wparam as i32)),
        LVM_DELETEALLITEMS => {
            let l = list_of(ctx, hwnd);
            l.items.clear();
            l.top = 0;
            l.focus = -1;
            1
        }
        LVM_GETNEXTITEM => list_of(ctx, hwnd).next_item(wparam as i32, lparam) as u32,
        LVM_GETITEMRECT if lparam != 0 => match list_of(ctx, hwnd).item_rect(w, h, style, wparam as i32) {
            Some((l, t, r, b)) => {
                for (i, v) in [l, t, r, b].into_iter().enumerate() {
                    wr(ctx, lparam + 4 * i as u32, v as u32)?;
                }
                1
            }
            None => 0,
        },
        LVM_HITTEST if lparam != 0 => {
            let px = rd(ctx, lparam)? as i32;
            let py = rd(ctx, lparam + 4)? as i32;
            let (flags, item) = match list_of(ctx, hwnd).hit(0, 0, w, h, style, px, py) {
                pocket_kernel::itemview::ListHit::Item(i) => (0x0004u32, i),
                _ => (0x0001, -1),
            };
            wr(ctx, lparam + 8, flags)?;
            wr(ctx, lparam + 12, item as u32)?;
            item as u32
        }
        LVM_ENSUREVISIBLE => {
            list_of(ctx, hwnd).ensure_visible(wparam as i32, fit);
            1
        }
        LVM_SCROLL => {
            let l = list_of(ctx, hwnd);
            let rows = (lparam as i32) / 16;
            l.top = (l.top as i32 + rows).clamp(0, l.items.len().saturating_sub(1) as i32) as usize;
            1
        }
        LVM_SETBKCOLOR | LVM_SETTEXTCOLOR | LVM_SETTEXTBKCOLOR | LVM_REDRAWITEMS | LVM_ARRANGE
        | LVM_UPDATE | LVM_SETITEMCOUNT | LVM_SORTITEMS => 1,
        LVM_DELETECOLUMN => {
            let l = list_of(ctx, hwnd);
            if (wparam as usize) < l.columns.len() {
                l.columns.remove(wparam as usize);
                1
            } else {
                0
            }
        }
        LVM_GETCOLUMNWIDTH => list_of(ctx, hwnd)
            .columns
            .get(wparam as usize)
            .map(|c| c.width as u32)
            .unwrap_or(0),
        LVM_SETCOLUMNWIDTH => {
            let l = list_of(ctx, hwnd);
            let col = wparam as usize;
            let width = match lparam as i32 {
                // LVSCW_AUTOSIZE / LVSCW_AUTOSIZE_USEHEADER: fit the text.
                -1 | -2 => {
                    let head = l.columns.get(col).map(|c| pocket_kernel::font::str_width(&c.text)).unwrap_or(0);
                    let body = l
                        .items
                        .iter()
                        .map(|it| pocket_kernel::font::str_width(it.text(col)))
                        .max()
                        .unwrap_or(0);
                    head.max(body) + 12
                }
                n => n,
            };
            if l.columns.is_empty() && col == 0 {
                l.columns.push(ListColumn::default());
            }
            match l.columns.get_mut(col) {
                Some(c) => {
                    c.width = width;
                    1
                }
                None => 0,
            }
        }
        LVM_GETHEADER => 0,
        LVM_GETTOPINDEX => list_of(ctx, hwnd).top as u32,
        LVM_GETCOUNTPERPAGE => fit as u32,
        LVM_SETITEMSTATE if lparam != 0 => {
            let state = rd(ctx, lparam + 12)?;
            let mask = rd(ctx, lparam + 16)?;
            let l = list_of(ctx, hwnd);
            let events = l.set_state(wparam as i32, state, mask);
            if mask & LVIS_FOCUSED != 0 && state & LVIS_FOCUSED != 0 {
                l.focus = wparam as i32;
            }
            for ev in events {
                ctx.kernel.controls.notify(hwnd, ev);
            }
            1
        }
        LVM_GETITEMSTATE => list_of(ctx, hwnd)
            .items
            .get(wparam as usize)
            .map(|i| i.state & lparam)
            .unwrap_or(0),
        LVM_GETSELECTEDCOUNT => list_of(ctx, hwnd).selected_count() as u32,
        LVM_SETEXTENDEDLISTVIEWSTYLE => {
            let l = list_of(ctx, hwnd);
            let old = l.ex_style;
            let mask = if wparam == 0 { 0xFFFF_FFFF } else { wparam };
            l.ex_style = (old & !mask) | (lparam & mask);
            old
        }
        LVM_GETEXTENDEDLISTVIEWSTYLE => list_of(ctx, hwnd).ex_style,
        LVM_GETSELECTIONMARK => list_of(ctx, hwnd).focus as u32,
        LVM_SETSELECTIONMARK => {
            let l = list_of(ctx, hwnd);
            std::mem::replace(&mut l.focus, lparam as i32) as u32
        }
        LVM_INSERTITEMW if lparam != 0 => {
            let (index, item) = read_list_item(ctx, lparam)?;
            let at = list_of(ctx, hwnd).insert(index, item);
            at as u32
        }
        LVM_SETITEMW if lparam != 0 => {
            let mask = rd(ctx, lparam)?;
            let index = rd(ctx, lparam + 4)? as i32;
            let sub = rd(ctx, lparam + 8)? as usize;
            let state = rd(ctx, lparam + 12)?;
            let smask = rd(ctx, lparam + 16)?;
            let text = if mask & LVIF_TEXT != 0 {
                let p = rd(ctx, lparam + 20)?;
                read_text(ctx, p)
            } else {
                None
            };
            let image = rd(ctx, lparam + 28)? as i32;
            let param = rd(ctx, lparam + 32)?;
            let indent = rd(ctx, lparam + 36)? as i32;
            let l = list_of(ctx, hwnd);
            let Some(item) = usize::try_from(index).ok().and_then(|i| l.items.get_mut(i)) else {
                return Ok(0);
            };
            if let Some(text) = text {
                item.set_text(sub, text);
            }
            if sub == 0 {
                if mask & LVIF_IMAGE != 0 {
                    item.image = image;
                }
                if mask & LVIF_PARAM != 0 {
                    item.lparam = param;
                }
                if mask & LVIF_INDENT != 0 {
                    item.indent = indent;
                }
            }
            if mask & LVIF_STATE != 0 {
                let events = l.set_state(index, state, smask);
                for ev in events {
                    ctx.kernel.controls.notify(hwnd, ev);
                }
            }
            1
        }
        LVM_GETITEMW if lparam != 0 => {
            let mask = rd(ctx, lparam)?;
            let index = rd(ctx, lparam + 4)? as i32;
            let sub = rd(ctx, lparam + 8)? as usize;
            let Some(item) = usize::try_from(index)
                .ok()
                .and_then(|i| list_of(ctx, hwnd).items.get(i).cloned())
            else {
                return Ok(0);
            };
            if mask & LVIF_STATE != 0 {
                let smask = rd(ctx, lparam + 16)?;
                wr(ctx, lparam + 12, item.state & smask)?;
            }
            if mask & LVIF_TEXT != 0 {
                let p = rd(ctx, lparam + 20)?;
                let room = rd(ctx, lparam + 24)?;
                write_text(ctx, p, room, item.text(sub))?;
            }
            if mask & LVIF_IMAGE != 0 {
                wr(ctx, lparam + 28, item.image as u32)?;
            }
            if mask & LVIF_PARAM != 0 {
                wr(ctx, lparam + 32, item.lparam)?;
            }
            if mask & LVIF_INDENT != 0 {
                wr(ctx, lparam + 36, item.indent as u32)?;
            }
            1
        }
        LVM_GETITEMTEXTW if lparam != 0 => {
            let sub = rd(ctx, lparam + 8)? as usize;
            let p = rd(ctx, lparam + 20)?;
            let room = rd(ctx, lparam + 24)?;
            let text = list_of(ctx, hwnd)
                .items
                .get(wparam as usize)
                .map(|i| i.text(sub).to_string())
                .unwrap_or_default();
            write_text(ctx, p, room, &text)?
        }
        LVM_SETITEMTEXTW if lparam != 0 => {
            let sub = rd(ctx, lparam + 8)? as usize;
            let p = rd(ctx, lparam + 20)?;
            let text = read_text(ctx, p).unwrap_or_default();
            match list_of(ctx, hwnd).items.get_mut(wparam as usize) {
                Some(item) => {
                    item.set_text(sub, text);
                    1
                }
                None => 0,
            }
        }
        LVM_FINDITEMW if lparam != 0 => {
            const LVFI_PARAM: u32 = 0x0001;
            const LVFI_PARTIAL: u32 = 0x0008;
            let flags = rd(ctx, lparam)?;
            let p = rd(ctx, lparam + 4)?;
            let param = rd(ctx, lparam + 8)?;
            let needle = read_text(ctx, p).unwrap_or_default().to_lowercase();
            let start = (wparam as i32).saturating_add(1).max(0) as usize;
            let l = list_of(ctx, hwnd);
            (start..l.items.len())
                .find(|&i| {
                    let it = &l.items[i];
                    if flags & LVFI_PARAM != 0 {
                        it.lparam == param
                    } else if flags & LVFI_PARTIAL != 0 {
                        it.text(0).to_lowercase().starts_with(&needle)
                    } else {
                        it.text(0).to_lowercase() == needle
                    }
                })
                .map(|i| i as u32)
                .unwrap_or(u32::MAX)
        }
        LVM_GETSTRINGWIDTHW => {
            let text = read_text(ctx, lparam).unwrap_or_default();
            pocket_kernel::font::str_width(&text) as u32
        }
        LVM_INSERTCOLUMNW | LVM_SETCOLUMNW if lparam != 0 => {
            let mask = rd(ctx, lparam)?;
            let fmt = rd(ctx, lparam + 4)?;
            let cx = rd(ctx, lparam + 8)? as i32;
            let text = if mask & LVCF_TEXT != 0 {
                let p = rd(ctx, lparam + 12)?;
                read_text(ctx, p)
            } else {
                None
            };
            let l = list_of(ctx, hwnd);
            let at = if msg == LVM_INSERTCOLUMNW {
                let at = (wparam as usize).min(l.columns.len());
                l.columns.insert(at, ListColumn::default());
                at
            } else {
                wparam as usize
            };
            let Some(col) = l.columns.get_mut(at) else {
                return Ok(0);
            };
            if mask & LVCF_FMT != 0 {
                col.fmt = fmt;
            }
            if mask & LVCF_WIDTH != 0 {
                col.width = cx;
            }
            if let Some(text) = text {
                col.text = text;
            }
            if msg == LVM_INSERTCOLUMNW { at as u32 } else { 1 }
        }
        LVM_GETCOLUMNW if lparam != 0 => {
            let mask = rd(ctx, lparam)?;
            let Some(col) = list_of(ctx, hwnd).columns.get(wparam as usize).cloned() else {
                return Ok(0);
            };
            if mask & LVCF_FMT != 0 {
                wr(ctx, lparam + 4, col.fmt)?;
            }
            if mask & LVCF_WIDTH != 0 {
                wr(ctx, lparam + 8, col.width as u32)?;
            }
            if mask & LVCF_TEXT != 0 {
                let p = rd(ctx, lparam + 12)?;
                let room = rd(ctx, lparam + 16)?;
                write_text(ctx, p, room, &col.text)?;
            }
            1
        }
        _ => {
            log::debug!("list view 0x{hwnd:08x} ignoring message 0x{msg:04x}");
            0
        }
    };
    Ok(r)
}

/// Read an `LVITEMW` for `LVM_INSERTITEM`.
fn read_list_item(ctx: &mut CallCtx<'_>, p: u32) -> Result<(i32, ListItem), KernelError> {
    let mask = rd(ctx, p)?;
    let index = rd(ctx, p + 4)? as i32;
    let mut item = ListItem::default();
    if mask & LVIF_TEXT != 0 {
        let t = rd(ctx, p + 20)?;
        item.texts.push(read_text(ctx, t).unwrap_or_default());
    }
    if mask & LVIF_IMAGE != 0 {
        item.image = rd(ctx, p + 28)? as i32;
    }
    if mask & LVIF_PARAM != 0 {
        item.lparam = rd(ctx, p + 32)?;
    }
    if mask & LVIF_STATE != 0 {
        item.state = rd(ctx, p + 12)? & rd(ctx, p + 16)? & (LVIS_SELECTED | LVIS_FOCUSED);
    }
    if mask & LVIF_INDENT != 0 {
        item.indent = rd(ctx, p + 36)? as i32;
    }
    Ok((index, item))
}

thread_local! {
    /// Guest buffers holding the last few notification structures. A
    /// `WM_NOTIFY` is handled long before this many more are posted, so
    /// the oldest can be handed back to the heap.
    static NOTIFY_BUFFERS: RefCell<std::collections::VecDeque<u32>> = RefCell::new(Default::default());
}

/// Allocate a zeroed guest buffer for one notification.
fn notify_buffer(ctx: &mut CallCtx<'_>, size: u32) -> Option<u32> {
    let p = ctx.kernel.heap.alloc(size)?;
    let _ = ctx.cpu.write_mem(p, &vec![0u8; size as usize]);
    let old = NOTIFY_BUFFERS.with(|b| {
        let mut b = b.borrow_mut();
        b.push_back(p);
        (b.len() > 64).then(|| b.pop_front()).flatten()
    });
    if let Some(old) = old {
        ctx.kernel.heap.free(old);
    }
    Some(p)
}

/// Write a `TVITEMW` describing `handle` at `at`.
fn write_tvitem(ctx: &mut CallCtx<'_>, at: u32, hwnd: u32, handle: u32) -> Result<(), KernelError> {
    let item = ctx
        .kernel
        .controls
        .get(hwnd)
        .and_then(|c| c.tree.as_ref())
        .and_then(|t| t.get(handle).cloned());
    let Some(item) = item else { return Ok(()) };
    let mask = TVIF_HANDLE | TVIF_PARAM | TVIF_STATE | TVIF_IMAGE | TVIF_SELECTEDIMAGE | TVIF_CHILDREN;
    let children = if item.children.is_empty() { item.has_children } else { 1 };
    let fields = [
        mask,
        handle,
        item.state,
        0xFFFF,
        0,
        0,
        item.image as u32,
        item.selected_image as u32,
        children as u32,
        item.lparam,
    ];
    for (i, v) in fields.into_iter().enumerate() {
        wr(ctx, at + 4 * i as u32, v)?;
    }
    Ok(())
}

/// Post every queued view event to its parent as `WM_NOTIFY`.
pub(crate) fn flush_notifications(ctx: &mut CallCtx<'_>) {
    const NM_CLICK: i32 = -2;
    const NM_DBLCLK: i32 = -3;
    const TVN_SELCHANGEDW: i32 = -451;
    const TVN_ITEMEXPANDINGW: i32 = -454;
    const TVN_ITEMEXPANDEDW: i32 = -455;
    const LVN_ITEMCHANGED: i32 = -101;
    const LVN_COLUMNCLICK: i32 = -108;
    const LVN_ITEMACTIVATE: i32 = -114;
    const LVIF_STATE_CHANGED: u32 = 0x0008;

    let pending = std::mem::take(&mut ctx.kernel.controls.notifications);
    for (parent, id, hwnd, event) in pending {
        // (code, structure size, filler)
        let mut posts: Vec<(i32, u32)> = Vec::new();
        let mut tree_fill: Option<(u32, u32, u32)> = None; // action, old, new
        let mut list_fill: Option<(i32, i32, u32, u32, u32)> = None; // item, sub, new, old, changed
        match &event {
            ViewEvent::TreeExpand { item, action } => {
                posts.push((TVN_ITEMEXPANDINGW, 104));
                posts.push((TVN_ITEMEXPANDEDW, 104));
                tree_fill = Some((*action, 0, *item));
            }
            ViewEvent::TreeSelect { old, new, by } => {
                posts.push((TVN_SELCHANGEDW, 104));
                tree_fill = Some((*by, *old, *new));
            }
            ViewEvent::TreeClick => posts.push((NM_CLICK, 12)),
            ViewEvent::TreeActivate { .. } => posts.push((NM_DBLCLK, 12)),
            ViewEvent::ListChanged { item, new_state, old_state } => {
                posts.push((LVN_ITEMCHANGED, 48));
                list_fill = Some((*item, 0, *new_state, *old_state, LVIF_STATE_CHANGED));
            }
            ViewEvent::ListClick { item } => {
                posts.push((NM_CLICK, 48));
                list_fill = Some((*item, 0, 0, 0, 0));
            }
            ViewEvent::ListActivate { item } => {
                posts.push((NM_DBLCLK, 48));
                posts.push((LVN_ITEMACTIVATE, 48));
                list_fill = Some((*item, 0, 0, 0, 0));
            }
            ViewEvent::ColumnClick { column } => {
                posts.push((LVN_COLUMNCLICK, 48));
                list_fill = Some((-1, *column, 0, 0, 0));
            }
        }
        for (code, size) in posts {
            let Some(p) = notify_buffer(ctx, size) else { continue };
            let _ = wr(ctx, p, hwnd);
            let _ = wr(ctx, p + 4, id);
            let _ = wr(ctx, p + 8, code as u32);
            if let Some((action, old, new)) = tree_fill.filter(|_| size == 104) {
                let action = if code == TVN_SELCHANGEDW { action } else if action == TVE_COLLAPSE { 1 } else { 2 };
                let _ = wr(ctx, p + 12, action);
                let _ = write_tvitem(ctx, p + 16, hwnd, old);
                let _ = write_tvitem(ctx, p + 56, hwnd, new);
            }
            if let Some((item, sub, new, old, changed)) = list_fill.filter(|_| size == 48) {
                let lparam = ctx
                    .kernel
                    .controls
                    .get(hwnd)
                    .and_then(|c| c.list.as_ref())
                    .and_then(|l| usize::try_from(item).ok().and_then(|i| l.items.get(i)))
                    .map(|it| it.lparam)
                    .unwrap_or(0);
                for (off, v) in [(12, item as u32), (16, sub as u32), (20, new), (24, old), (28, changed), (40, lparam)] {
                    let _ = wr(ctx, p + off, v);
                }
            }
            if ctx.kernel.posted_messages.len() < 256 {
                ctx.kernel.posted_messages.push_back((parent, WM_NOTIFY, id, p));
            }
            log::debug!("WM_NOTIFY(code={code}) from 0x{hwnd:08x} to 0x{parent:08x}");
        }
    }
}

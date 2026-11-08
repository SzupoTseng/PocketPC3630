//! egui application: library screen, settings, per-game sheet.

use std::sync::mpsc::{self, Receiver, Sender};
use std::time::{Duration, Instant};

use eframe::egui::{self, Color32, Mesh, Pos2, Rect, RichText, ScrollArea, Sense, Vec2};
use egui_extras::image::load_image_bytes;

use pocket_core::kernel::{InputEvent, FB_HEIGHT, FB_WIDTH};
use pocket_library::{
    CpuBackendPref, GameEntry, GameSettings, GuestButton, LauncherConfig, Library, RotationPref,
    ScreenPref,
};

use crate::runner::{FrameSnapshot, InputCommand, RunOutcome, Runner};
use pocket_library::controls::{
    Binding, ButtonPosition, Controls as ControlConfig, Presentation, Profile,
};

// Virtual button layout for the Run screen — modelled after the
// j2me-loader gamepad: a D-pad on the left and three action buttons
// (A / B / Start) on the right. Pressing a button sends a
// `WM_KEYDOWN`/`WM_KEYUP` pair down to the guest.
//
// Which virtual-key code each button sends lives on
// [`pocket_library::GuestButton`], shared with the Android pad and with
// the keybinding editor, and is pinned against
// `pocket_core::kernel::gapi` — the table `gx.dll`'s `GXGetDefaultKeys`
// hands the guest — by `guest_button_vks_match_the_gapi_table` below.

/// Top-level egui app.
pub struct PocketLauncher {
    ui_theme: &'static pocket_library::theme::Theme,
    icon_cache: std::collections::HashMap<String, egui::TextureHandle>,
    library: Library,
    catalog_view: crate::catalog_ui::CatalogView,
    selected_game: Option<String>,
    screen: Screen,
    game_only_fullscreen: bool,
    fullscreen_before_game_only: Option<bool>,
    runner: Runner,
    events_rx: Receiver<UiEvent>,
    events_tx: Sender<UiEvent>,
    /// Live framebuffer updates streamed by [`Runner`] while a game
    /// is running. `Some` between [`Self::spawn_run`] and
    /// `UiEvent::RunFinished`; `None` otherwise.
    frame_rx: Option<Receiver<FrameSnapshot>>,
    /// Channel for sending [`InputCommand`]s (taps / D-pad / stop)
    /// to the running emulator. Mirrors `frame_rx`.
    input_tx: Option<Sender<InputCommand>>,
    /// Which guest buttons are held right now, and by which input
    /// source, so we can fire the matching `WM_KEYUP` on release.
    held: HeldButtons,
    host_keys: std::collections::HashMap<egui::Key, u16>,
    activation_releases: Vec<(u16, Instant)>,
    gamepad_connected: usize,
    gamepad_menu_held: bool,
    /// `Some` while a stylus drag is in progress — carries the last
    /// reported game-space coordinates so we don't spam the guest
    /// with redundant events.
    pointer_down_at: Option<(u16, u16)>,
    /// Game currently being launched, used as a status caption
    /// while the run is in progress.
    running_game: Option<String>,
    status: String,
    config_draft: Option<LauncherConfig>,
    game_settings_draft: Option<(String, GameSettings)>,
    last_frame_texture: Option<egui::TextureHandle>,
    last_frame_status: Option<String>,
    frame_stats: FrameStats,
    screenshot_path: Option<std::path::PathBuf>,
    /// How far the presented frame is turned. Initialised from the
    /// running game's persisted [`RotationPref`] and editable live on
    /// the Run screen, where a change is written straight back to the
    /// game's settings so the next launch starts turned the same way.
    game_rotation: RotationPref,
    /// Id of the game currently on the Run screen, so a rotation
    /// change made there knows which `game.json` to write.
    running_game_id: Option<String>,
    /// Guest button waiting for the user to press the host key that
    /// should drive it, while the keybinding editor is in "press a
    /// key" mode.
    binding_capture: Option<GuestButton>,
    controls: ControlConfig,
    controls_draft: Option<(String, ControlConfig, Presentation)>,
    controls_error: Option<String>,
    menu_open: bool,
    pause_requested: bool,
    library_query: String,
    recent_first: bool,
    guest_text: String,
}

fn control_label(vk: u16) -> String {
    match vk {
        38 => "Up".into(),
        40 => "Down".into(),
        37 => "Left".into(),
        39 => "Right".into(),
        13 => "Enter".into(),
        9 => "Tab".into(),
        27 => "Esc".into(),
        17 => "A/Ctrl".into(),
        32 => "B/Space".into(),
        16 => "C/Shift".into(),
        33 => "PgUp".into(),
        34 => "PgDn".into(),
        112 | 113 | 119 | 120 => format!("F{}", vk - 111),
        121 => "Rec/F10".into(),
        0x86 => "Action".into(),
        0xc1..=0xc6 => format!("App {}", vk - 0xc0),
        _ => format!("VK {vk}"),
    }
}

/// Use an installed Windows CJK font at runtime; never redistribute OS fonts.
fn configure_system_fonts(ctx: &egui::Context) {
    #[cfg(windows)]
    {
        if let Some(directory) = std::env::var_os("WINDIR") {
            let path = std::path::PathBuf::from(directory).join("Fonts/msyh.ttc");
            if let Ok(bytes) = std::fs::read(path) {
                let mut fonts = egui::FontDefinitions::default();
                fonts
                    .font_data
                    .insert("system-cjk".into(), egui::FontData::from_owned(bytes));
                for family in fonts.families.values_mut() {
                    family.push("system-cjk".into());
                }
                ctx.set_fonts(fonts);
            }
        }
    }
    #[cfg(not(windows))]
    let _ = ctx;
}

/// The emulator's interface colors (see `pocket_library::theme`): the glossy
/// Windows Mobile 5/6 look, red by default, the same palette as the web page.
fn apply_theme(ctx: &egui::Context, theme: &pocket_library::theme::Theme) {
    use pocket_library::theme::rgb;
    let color = |c| {
        let [r, g, b] = rgb(c);
        Color32::from_rgb(r, g, b)
    };
    let flat = color(theme.flat);
    let mut visuals = if theme.dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    visuals.panel_fill = color(theme.panel);
    visuals.window_fill = color(theme.panel);
    visuals.extreme_bg_color = color(theme.panel2);
    visuals.faint_bg_color = color(theme.panel2);
    visuals.override_text_color = Some(color(theme.ink));
    visuals.window_stroke = egui::Stroke::new(1.0_f32, color(theme.edge));
    visuals.selection.bg_fill = color(theme.select);
    visuals.selection.stroke = egui::Stroke::new(1.0_f32, color(theme.select_ink));
    visuals.hyperlink_color = flat;
    visuals.window_rounding = 8.0.into();
    visuals.menu_rounding = 8.0.into();
    for widgets in [
        &mut visuals.widgets.inactive,
        &mut visuals.widgets.hovered,
        &mut visuals.widgets.active,
        &mut visuals.widgets.noninteractive,
    ] {
        widgets.rounding = 6.0.into();
        widgets.bg_stroke = egui::Stroke::new(1.0_f32, color(theme.line));
    }
    visuals.widgets.inactive.weak_bg_fill = color(theme.panel2);
    visuals.widgets.inactive.bg_fill = color(theme.panel2);
    visuals.widgets.hovered.weak_bg_fill = color(theme.panel2).linear_multiply(0.92);
    visuals.widgets.hovered.bg_stroke = egui::Stroke::new(1.0_f32, flat);
    visuals.widgets.active.weak_bg_fill = color(theme.select);
    visuals.widgets.active.fg_stroke = egui::Stroke::new(1.0_f32, color(theme.select_ink));
    ctx.set_visuals(visuals);
    let mut style = (*ctx.style()).clone();
    style.spacing.item_spacing = Vec2::new(10.0, 8.0);
    style.spacing.button_padding = Vec2::new(12.0, 6.0);
    ctx.set_style(style);
}

/// A theme color as an egui color.
fn theme_color(color: pocket_library::theme::Rgb) -> Color32 {
    let [r, g, b] = pocket_library::theme::rgb(color);
    Color32::from_rgb(r, g, b)
}

/// The PocketPC3630 mark: a small handheld with a themed screen, drawn
/// directly so it follows the chosen theme.
fn brand_icon(ui: &mut egui::Ui, theme: &pocket_library::theme::Theme) {
    use pocket_library::theme::rgb;
    let [r, g, b] = rgb(theme.flat);
    let (rect, _) = ui.allocate_exact_size(Vec2::new(16.0, 24.0), Sense::hover());
    let painter = ui.painter();
    painter.rect_filled(rect, 4.0, Color32::WHITE);
    painter.rect_stroke(
        rect,
        4.0,
        egui::Stroke::new(1.0_f32, Color32::from_black_alpha(90)),
    );
    let screen = Rect::from_min_size(rect.min + Vec2::new(2.5, 3.0), Vec2::new(11.0, 12.0));
    painter.rect_filled(screen, 1.0, Color32::from_rgb(r, g, b));
    painter.circle_filled(
        Pos2::new(rect.center().x, rect.max.y - 4.0),
        2.0,
        Color32::from_rgb(r, g, b),
    );
}

/// Texture corners for a rotated presentation, in
/// `[left-top, right-top, left-bottom, right-bottom]` order.
///
/// Turning the *presented* frame rather than the guest's own geometry
/// is what a landscape game that ships as a 240x320 portrait build
/// needs: it keeps rendering into the panel it was written for and the
/// user still sees it the right way up. See
/// [`pocket_library::RotationPref`].
fn rotation_uv(rotation: RotationPref) -> [Pos2; 4] {
    match rotation {
        RotationPref::None => [
            egui::pos2(0.0, 0.0),
            egui::pos2(1.0, 0.0),
            egui::pos2(0.0, 1.0),
            egui::pos2(1.0, 1.0),
        ],
        RotationPref::Cw90 => [
            egui::pos2(0.0, 1.0),
            egui::pos2(0.0, 0.0),
            egui::pos2(1.0, 1.0),
            egui::pos2(1.0, 0.0),
        ],
        RotationPref::Half => [
            egui::pos2(1.0, 1.0),
            egui::pos2(0.0, 1.0),
            egui::pos2(1.0, 0.0),
            egui::pos2(0.0, 0.0),
        ],
        RotationPref::Ccw90 => [
            egui::pos2(1.0, 0.0),
            egui::pos2(1.0, 1.0),
            egui::pos2(0.0, 0.0),
            egui::pos2(0.0, 1.0),
        ],
    }
}

fn fullscreen_frame_rect(bounds: Rect, size: Vec2, rotation: RotationPref) -> Rect {
    let frame_size = if rotation.is_quarter_turn() {
        Vec2::new(size.y, size.x)
    } else {
        size
    };
    let scale = (bounds.width() / frame_size.x).min(bounds.height() / frame_size.y);
    Rect::from_center_size(bounds.center(), frame_size * scale)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Screen {
    Library,
    Catalog,
    Settings,
    GameSettings,
    Run,
}

/// Where a held guest button came from.
///
/// The on-screen pad and the physical keyboard can hold the same VK at
/// the same time, so [`HeldButtons`] keeps one set per source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum InputSource {
    Modifiers,
    Activation,
    Keyboard,
    Pointer,
}

/// Which guest buttons are down, tracked per input source.
///
/// Both sources used to share a single `HashSet<u16>`, and that quietly
/// broke holding a key: [`PocketLauncher::ui_virtual_pad`] runs every frame and
/// released whatever it found in the set whenever the pointer was not on
/// its own rect, so a keyboard arrow the user was still holding was
/// cancelled with a `WM_KEYUP` in the very next frame. JumpyBall steers
/// its ball for as long as the direction key is down, so on the desktop
/// the ball stopped moving the moment the user had touched the on-screen
/// D-pad once. Keeping the sources apart — and only telling the guest a
/// button came up once *no* source holds it — is what lets a hold on
/// either one last as long as the user holds it.
#[derive(Debug, Default)]
struct HeldButtons {
    modifiers: std::collections::HashSet<u16>,
    activation: std::collections::HashSet<u16>,
    keyboard: std::collections::HashSet<u16>,
    pointer: std::collections::HashSet<u16>,
    gamepad: std::collections::HashMap<(u8, &'static str), u16>,
}

impl HeldButtons {
    fn set(&mut self, source: InputSource) -> &mut std::collections::HashSet<u16> {
        match source {
            InputSource::Modifiers => &mut self.modifiers,
            InputSource::Activation => &mut self.activation,
            InputSource::Keyboard => &mut self.keyboard,
            InputSource::Pointer => &mut self.pointer,
        }
    }

    /// Record a press. Returns whether the guest should be told the
    /// button went down — only when no source was holding it yet, so a
    /// key pressed while the same button is held on the pad does not
    /// become a second `WM_KEYDOWN` (which a game like JumpyBall would
    /// act on as another menu step).
    fn press(&mut self, source: InputSource, vk: u16) -> bool {
        let was_held = self.is_held(vk);
        self.set(source).insert(vk);
        !was_held
    }

    /// Record a release. Returns whether the guest should be told the
    /// button came up — only once the other source has let go too.
    fn release(&mut self, source: InputSource, vk: u16) -> bool {
        let was_held = self.set(source).remove(&vk);
        was_held && !self.is_held(vk)
    }

    fn is_held(&self, vk: u16) -> bool {
        self.modifiers.contains(&vk)
            || self.activation.contains(&vk)
            || self.keyboard.contains(&vk)
            || self.pointer.contains(&vk)
            || self.gamepad.values().any(|value| *value == vk)
    }

    fn is_held_by(&self, source: InputSource, vk: u16) -> bool {
        match source {
            InputSource::Modifiers => self.modifiers.contains(&vk),
            InputSource::Activation => self.activation.contains(&vk),
            InputSource::Keyboard => self.keyboard.contains(&vk),
            InputSource::Pointer => self.pointer.contains(&vk),
        }
    }

    /// Every VK still held, forgetting all of them. Used when the window
    /// is closing so the guest is not left with a stuck key.
    fn drain_all(&mut self) -> Vec<u16> {
        self.keyboard
            .drain()
            .chain(self.modifiers.drain())
            .chain(self.activation.drain())
            .chain(self.pointer.drain())
            .chain(self.gamepad.drain().map(|(_, vk)| vk))
            // A VK held on both at once must only be released once.
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect()
    }
}

// Each event is the end of a background job, so every name says so.
#[allow(clippy::enum_variant_names)]
#[derive(Debug)]
pub enum UiEvent {
    ImportFinished(Result<String, String>),
    SaveFinished(Result<String, String>),
    RunFinished(RunOutcome),
}

#[derive(Debug, Clone)]
struct FrameStats {
    displayed_frames: u64,
    fps: f32,
    window_started_at: Option<Instant>,
    window_frames: u32,
    last_frame_at: Option<Instant>,
    last_frame_ms: Option<f32>,
}

impl Default for FrameStats {
    fn default() -> Self {
        Self {
            displayed_frames: 0,
            fps: 0.0,
            window_started_at: None,
            window_frames: 0,
            last_frame_at: None,
            last_frame_ms: None,
        }
    }
}

impl FrameStats {
    fn reset(&mut self) {
        *self = Self::default();
    }

    fn record_frame(&mut self) {
        let now = Instant::now();
        if let Some(previous) = self.last_frame_at {
            self.last_frame_ms = Some(now.duration_since(previous).as_secs_f32() * 1000.0);
        }
        self.last_frame_at = Some(now);
        self.displayed_frames = self.displayed_frames.saturating_add(1);
        self.window_frames = self.window_frames.saturating_add(1);

        let started_at = match self.window_started_at {
            Some(t) => t,
            None => {
                self.window_started_at = Some(now);
                now
            }
        };
        let elapsed = now.duration_since(started_at);
        if elapsed >= Duration::from_secs(1) {
            self.fps = self.window_frames as f32 / elapsed.as_secs_f32();
            self.window_frames = 0;
            self.window_started_at = Some(now);
        }
    }

    fn overlay_text(&self) -> String {
        let last_ms = self.last_frame_ms.unwrap_or(0.0);
        let since_ms = self
            .last_frame_at
            .map(|t| t.elapsed().as_secs_f32() * 1000.0)
            .unwrap_or(0.0);
        format!(
            "FPS {:.1}  Frames {}  Last {:.0}ms  Since {:.0}ms",
            self.fps, self.displayed_frames, last_ms, since_ms
        )
    }
}

impl PocketLauncher {
    pub fn new(cc: &eframe::CreationContext<'_>, library: Library) -> Self {
        let ui_theme = pocket_library::theme::load(library.root());
        apply_theme(&cc.egui_ctx, ui_theme);
        configure_system_fonts(&cc.egui_ctx);
        let (tx, rx) = mpsc::channel();
        Self {
            ui_theme,
            library,
            catalog_view: crate::catalog_ui::CatalogView::default(),
            icon_cache: std::collections::HashMap::new(),
            selected_game: None,
            screen: Screen::Library,
            game_only_fullscreen: false,
            fullscreen_before_game_only: None,
            runner: Runner::new(),
            events_rx: rx,
            events_tx: tx,
            frame_rx: None,
            input_tx: None,
            held: HeldButtons::default(),
            host_keys: std::collections::HashMap::new(),
            activation_releases: Vec::new(),
            gamepad_connected: 0,
            gamepad_menu_held: false,
            pointer_down_at: None,
            running_game: None,
            status: String::new(),
            config_draft: None,
            game_settings_draft: None,
            last_frame_texture: None,
            last_frame_status: None,
            frame_stats: FrameStats::default(),
            screenshot_path: None,
            game_rotation: RotationPref::None,
            running_game_id: None,
            binding_capture: None,
            controls: ControlConfig::default(),
            controls_draft: None,
            controls_error: None,
            menu_open: false,
            pause_requested: false,
            library_query: String::new(),
            recent_first: false,
            guest_text: String::new(),
        }
    }

    fn send_command(&self, command: InputCommand) {
        if let Some(tx) = &self.input_tx {
            let _ = tx.send(command);
        }
    }

    fn open_controls(&mut self, id: &str) {
        self.controls_error = None;
        let value = if id.is_empty() {
            self.library.controls_path(None).and_then(|p| {
                pocket_library::controls::read(&p)
                    .map(|v| v.unwrap_or_default())
                    .map_err(Into::into)
            })
        } else {
            self.library.effective_controls(id).map(|v| v.0)
        };
        match value {
            Ok(value) => {
                self.release_all_keys();
                self.controls_draft = Some((
                    id.to_string(),
                    value,
                    self.library.presentation(id).unwrap_or_default(),
                ));
            }
            Err(e) => {
                self.controls_error = Some(e.to_string());
                self.controls_draft = Some((
                    id.into(),
                    ControlConfig::default(),
                    self.library.presentation(id).unwrap_or_default(),
                ));
            }
        }
    }

    fn ui_controls_editor(&mut self, ctx: &egui::Context) {
        let Some((id, mut controls, mut meta)) = self.controls_draft.take() else {
            return;
        };
        let mut open = true;
        let mut save = false;
        let mut reset = false;
        egui::Window::new(if id.is_empty(){"Default controls".to_string()}else{format!("Controls: {}",self.library.get(&id).map(|g|g.display_name.as_str()).unwrap_or(&id))})
            .open(&mut open).vscroll(true).default_width(500.0).show(ctx,|ui| {
                if let Some(error)=&self.controls_error{ui.label(format!("Cannot read controls: {error}. Original file retained. Explicitly Restore inherited/default before editing."));}
                ui.label("Program controls are independent of saved progress. Save inside the program before exporting a progress backup.");
                let before=controls.profile;
                egui::ComboBox::from_id_source("control_profile").selected_text(controls.profile.label()).show_ui(ui,|ui| {
                    for p in Profile::ALL { ui.selectable_value(&mut controls.profile,p,p.label()); }
                });
                if controls.profile!=before && controls.profile!=Profile::Custom { let profile=controls.profile; controls.keys=profile.keys(); controls.keyboard=profile==Profile::Text; }
                if controls.profile==Profile::Ppc2003 { ui.label("PPC2003 reference keys. Shortcuts and Enter click once; directions and Rec hold. Power / Reset unavailable."); }
                if controls.profile==Profile::Device { ui.label("Generic CE App1–6 / Action. Device Emulator skin and shell behavior require separate reference testing."); }
                ui.checkbox(&mut controls.keyboard,"Text / keyboard input");
                ui.horizontal(|ui|{ui.label("Menu shortcut");ui.text_edit_singleline(&mut controls.menu_key);});
                ui.checkbox(&mut controls.auto_hide_gamepad,"Hide touch controls with a controller");
                ui.label("Windows: XInput controllers; click left stick for Menu / Resume unless rebound. Linux controller integration is unavailable.");
                ui.add(egui::Slider::new(&mut controls.opacity,0.1..=1.0).text("Button opacity"));
                ui.add(egui::Slider::new(&mut controls.volume,0.0..=1.0).text("Volume"));
                ui.add(egui::Slider::new(&mut controls.zoom,1.0..=4.0).text("Zoom"));
                ui.add(egui::Slider::new(&mut controls.zoom_x,0.0..=1.0).text("Guest focus X"));
                ui.add(egui::Slider::new(&mut controls.zoom_y,0.0..=1.0).text("Guest focus Y"));
                ui.label("Pointer uses absolute stylus coordinates. Relative mouse is pending.");
                ui.collapsing("Visible buttons",|ui| {
                    for vk in [38,40,37,39,13,9,27,17,32,16,33,34,0x86,0xc1,0xc2,0xc3,0xc4,0xc5,0xc6] {
                        let mut checked=controls.keys.contains(&vk);
                        if ui.checkbox(&mut checked,control_label(vk)).changed() { controls.profile=Profile::Custom; if checked {controls.keys.push(vk)} else {controls.keys.retain(|k|*k!=vk)} }
                    }
                });
                ui.collapsing("Keyboard / controller remapping",|ui| {
                    ui.label("Host key names: Up, Enter, A, PadA…; choose a separate menu shortcut above.");
                    let mut remove=None;
                    for (i,b) in controls.bindings.iter_mut().enumerate() {
                        ui.horizontal(|ui| { ui.text_edit_singleline(&mut b.input);
                            egui::ComboBox::from_id_source(("target",i)).selected_text(control_label(b.vk)).show_ui(ui,|ui| {for k in [38,40,37,39,13,9,27,17,32,16,33,34,0x86,0xc1,0xc2,0xc3,0xc4,0xc5,0xc6] {ui.selectable_value(&mut b.vk,k,control_label(k));}});
                            if ui.button("Remove").clicked(){remove=Some(i);}
                        });
                    }
                    if let Some(i)=remove {controls.bindings.remove(i);}
                    if ui.button("Add mapping").clicked(){controls.bindings.push(Binding{input:String::new(),vk:13});}
                });
                for (name,layout) in [("Portrait layout",&mut controls.portrait),("Landscape layout",&mut controls.landscape)] {
                    ui.collapsing(name,|ui| {
                        if ui.button("Create layout for visible buttons").clicked() {
                            *layout=controls.keys.iter().enumerate().map(|(i,vk)|ButtonPosition{vk:*vk,x:0.12+(i%5)as f32*0.19,y:0.16+(i/5)as f32*0.22,size:0.15}).collect();
                        }
                        for b in layout.iter_mut(){ui.horizontal(|ui|{ui.label(control_label(b.vk));ui.add(egui::DragValue::new(&mut b.x).speed(0.01).clamp_range(0.0..=1.0).prefix("x "));ui.add(egui::DragValue::new(&mut b.y).speed(0.01).clamp_range(0.0..=1.0).prefix("y "));ui.add(egui::DragValue::new(&mut b.size).speed(0.01).clamp_range(0.03..=0.4).prefix("size "));});}
                    });
                }
                if !id.is_empty(){ui.separator();ui.label("Display alias");ui.text_edit_singleline(&mut meta.alias);ui.checkbox(&mut meta.favorite,"Favorite");}
                ui.horizontal(|ui| {save=ui.add_enabled(self.controls_error.is_none(),egui::Button::new("Save controls")).clicked();reset=ui.button("Restore inherited / default").clicked();});
            });
        if save {
            let id_option = (!id.is_empty()).then_some(id.as_str());
            let result = self
                .library
                .write_controls(id_option, &controls)
                .and_then(|_| {
                    if id.is_empty() {
                        Ok(())
                    } else {
                        self.library.write_presentation(&id, &meta)
                    }
                });
            match result {
                Ok(()) => {
                    self.status = "Controls saved.".into();
                    if self.running_game_id.as_ref().is_some_and(|active| {
                        active == &id
                            || (id.is_empty()
                                && self
                                    .library
                                    .effective_controls(active)
                                    .is_ok_and(|(_, overridden)| !overridden))
                    }) {
                        self.controls = controls.clone();
                        self.send_command(InputCommand::SetVolume(
                            (controls.volume * 1000.0) as u16,
                        ));
                    }
                    open = false;
                }
                Err(e) => self.status = e.to_string(),
            }
        }
        if reset {
            let result = (|| -> Result<(), pocket_library::LibraryError> {
                let path = self
                    .library
                    .controls_path((!id.is_empty()).then_some(id.as_str()))?;
                if path.exists() {
                    std::fs::remove_file(&path)?;
                }
                if !id.is_empty() {
                    let legacy = path.with_file_name("web-controls.json");
                    if legacy.exists() {
                        std::fs::remove_file(legacy)?;
                    }
                }
                Ok(())
            })();
            match result {
                Ok(()) => {
                    if let Some(active) = self.running_game_id.as_deref() {
                        if let Ok((value, overridden)) = self.library.effective_controls(active) {
                            if active == id || (id.is_empty() && !overridden) {
                                self.controls = value;
                                self.send_command(InputCommand::SetVolume(
                                    (self.controls.volume * 1000.0) as u16,
                                ));
                            }
                        }
                    }
                    open = false;
                    self.status = "Inherited controls restored.".into();
                }
                Err(e) => self.status = e.to_string(),
            }
        }
        if open {
            self.controls_draft = Some((id, controls, meta));
        }
    }

    fn request_screenshot(&mut self, ctx: &egui::Context) {
        // F12 captures EQ now; a modal save picker would keep the guest
        // running until the user chose a path and capture the wrong moment.
        let path = std::env::var_os("POCKETHLE_SCREENSHOT_PATH")
            .map(std::path::PathBuf::from)
            .unwrap_or_else(|| {
                let stamp = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis();
                self.library
                    .root()
                    .join("screenshots")
                    .join(format!("pockethle-{stamp}.png"))
            });
        if let Some(parent) = path.parent() {
            if let Err(error) = std::fs::create_dir_all(parent) {
                self.status = format!("Could not save screenshot: {error}");
                return;
            }
        }
        self.screenshot_path = Some(path);
        ctx.send_viewport_cmd(egui::ViewportCommand::Screenshot);
    }

    /// Pick the interface theme: applies it at once and remembers it in the
    /// library folder. The same list the web page and Android offer.
    fn theme_picker(&mut self, ui: &mut egui::Ui) {
        let mut chosen = self.ui_theme.id;
        ui.horizontal(|ui| {
            ui.label("Interface theme");
            egui::ComboBox::from_id_source("ui_theme")
                .selected_text(format!("{} / {}", self.ui_theme.en, self.ui_theme.zh))
                .show_ui(ui, |ui| {
                    for theme in pocket_library::theme::THEMES {
                        ui.selectable_value(
                            &mut chosen,
                            theme.id,
                            format!("{} / {}", theme.en, theme.zh),
                        );
                    }
                });
        });
        if chosen != self.ui_theme.id {
            self.ui_theme = pocket_library::theme::by_id(chosen);
            apply_theme(ui.ctx(), self.ui_theme);
            if let Err(e) = pocket_library::theme::save(self.library.root(), chosen) {
                self.status = format!("Could not save the interface theme: {e}");
            }
        }
    }

    fn ui_game_menu(&mut self, ctx: &egui::Context) {
        if !self.menu_open || self.input_tx.is_none() {
            return;
        }
        let mut open = true;
        let before = self.controls.clone();
        egui::Window::new(if self.runner.is_paused() {
            "Paused"
        } else {
            "Pausing…"
        })
        .id(egui::Id::new("in_game_menu"))
        .anchor(egui::Align2::CENTER_CENTER, Vec2::ZERO)
        .default_width(440.0)
        .default_height(640.0)
        .max_height((ctx.screen_rect().height() - 80.0).max(100.0))
        .collapsible(false)
        .vscroll(true)
        .open(&mut open)
        .show(ctx, |ui| {
            ui.set_enabled(self.runner.is_paused());
            if ui.button("Resume").clicked() {
                self.menu_open = false;
            }
            if ui.button("Controls / program settings").clicked() {
                if let Some(id) = self.running_game_id.clone() {
                    self.open_controls(&id);
                }
            }
            ui.horizontal(|ui| {
                ui.label("Volume");
                if ui
                    .add(egui::Slider::new(&mut self.controls.volume, 0.0..=1.0))
                    .changed()
                {
                    self.send_command(InputCommand::SetVolume(
                        (self.controls.volume * 1000.0) as u16,
                    ));
                }
            });
            self.theme_picker(ui);
            ui.add(egui::Slider::new(&mut self.controls.zoom, 1.0..=4.0).text("Zoom"));
            ui.add(egui::Slider::new(&mut self.controls.zoom_x, 0.0..=1.0).text("Guest focus X"));
            ui.add(egui::Slider::new(&mut self.controls.zoom_y, 0.0..=1.0).text("Guest focus Y"));
            if ui.button("Save screenshot (F12)").clicked() {
                self.request_screenshot(ctx);
            }
            if ui.button("Rotate display").clicked() {
                self.game_rotation = match self.game_rotation {
                    RotationPref::None => RotationPref::Cw90,
                    RotationPref::Cw90 => RotationPref::Half,
                    RotationPref::Half => RotationPref::Ccw90,
                    RotationPref::Ccw90 => RotationPref::None,
                };
                self.persist_rotation(self.game_rotation);
            }
            ui.label("Committed text (separate from game shortcuts)");
            ui.text_edit_multiline(&mut self.guest_text);
            if ui.button("Queue text for Resume").clicked() {
                for unit in self.guest_text.encode_utf16() {
                    self.send_input(InputEvent::Text { unit });
                }
                self.guest_text.clear();
            }
            if ui.button("Queue guest Backspace").clicked() {
                self.send_input(InputEvent::Text { unit: 8 });
            }
            ui.label("Save inside the program before backing up; this is not a save-state.");
            if ui.button("Export written progress…").clicked() {
                if let Some(game) = self
                    .running_game_id
                    .as_ref()
                    .and_then(|id| self.library.get(id))
                    .cloned()
                {
                    self.spawn_save_action(&game, "export");
                }
            }
            if ui.button("Return to library").clicked() {
                self.send_command(InputCommand::Stop);
                self.release_all_keys();
                self.screen = Screen::Library;
                self.menu_open = false;
            }
        });
        if self.controls != before {
            if let Some(id) = self.running_game_id.as_deref() {
                if let Err(error) = self.library.write_controls(Some(id), &self.controls) {
                    self.status = format!("Could not save display/volume: {error}");
                }
            }
        }
        if !open {
            self.menu_open = false;
        }
    }

    fn drain_events(&mut self, ctx: &egui::Context) {
        while let Ok(ev) = self.events_rx.try_recv() {
            match ev {
                UiEvent::ImportFinished(Ok(message)) => {
                    self.status = message;
                    self.reload_library();
                }
                UiEvent::ImportFinished(Err(e)) => {
                    self.status = if e == "cancelled" { String::new() } else { e };
                }
                UiEvent::SaveFinished(Ok(message)) | UiEvent::SaveFinished(Err(message)) => {
                    self.status = message;
                }
                UiEvent::RunFinished(outcome) => {
                    self.last_frame_status = Some(outcome.summary.clone());
                    if self.game_only_fullscreen {
                        self.set_game_only_fullscreen(ctx, false);
                    }
                    if let Some(frame) = outcome.framebuffer {
                        self.upload_frame_texture(ctx, &frame);
                    }
                    self.status = outcome.ui_message;
                    self.menu_open = false;
                    self.frame_rx = None;
                    self.input_tx = None;
                    self.release_all_keys();
                    self.pointer_down_at = None;
                    self.running_game = None;
                }
            }
        }
        // Drain any live preview frames the background runner may
        // have produced since the last UI tick.
        let mut latest: Option<FrameSnapshot> = None;
        if let Some(rx) = self.frame_rx.as_ref() {
            while let Ok(frame) = rx.try_recv() {
                latest = Some(frame);
            }
        }
        if let Some(frame) = latest {
            self.upload_frame_texture(ctx, &frame);
        }
    }

    fn upload_frame_texture(&mut self, ctx: &egui::Context, frame: &FrameSnapshot) {
        if self.input_tx.is_some() && self.status == "Starting emulator..." {
            self.status = format!(
                "Playing {}",
                self.running_game.as_deref().unwrap_or("program")
            );
        }
        let size = [frame.width as usize, frame.height as usize];
        let img = egui::ColorImage::from_rgba_unmultiplied(size, &frame.rgba);
        if let Some(tex) = self.last_frame_texture.as_mut() {
            tex.set(img, egui::TextureOptions::NEAREST);
        } else {
            let tex = ctx.load_texture("pockethle-fb", img, egui::TextureOptions::NEAREST);
            self.last_frame_texture = Some(tex);
        }
        self.frame_stats.record_frame();
    }

    fn reload_library(&mut self) {
        match Library::open(self.library.root()) {
            Ok(lib) => self.library = lib,
            Err(e) => self.status = format!("Could not reload library: {e}"),
        }
    }

    fn ui_top_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            brand_icon(ui, self.ui_theme);
            ui.heading(RichText::new("PocketPC3630").strong());
            ui.add_space(16.0);
            if ui
                .selectable_label(self.screen == Screen::Library, "Library")
                .clicked()
            {
                if self.input_tx.is_some() {
                    self.release_all_keys();
                    self.send_command(InputCommand::Stop);
                }
                self.screen = Screen::Library;
            }
            if self.input_tx.is_some()
                && ui
                    .selectable_label(self.screen == Screen::Run, "Playing")
                    .clicked()
            {
                self.screen = Screen::Run;
            }
            if ui
                .selectable_label(self.screen == Screen::Settings, "Settings")
                .clicked()
            {
                self.config_draft = Some(self.library.config().clone());
                self.screen = Screen::Settings;
            }
            if ui
                .selectable_label(self.screen == Screen::Catalog, "Catalog")
                .clicked()
            {
                self.screen = Screen::Catalog;
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button(RichText::new("Import game").strong()).clicked() {
                    self.spawn_import_dialog();
                }
            });
        });
        ui.add_space(6.0);
    }

    fn ui_library(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Search");
            ui.text_edit_singleline(&mut self.library_query);
            ui.checkbox(&mut self.recent_first, "Recently played");
            if ui.button("Default controls").clicked() {
                self.open_controls("");
            }
        });
        let query = self.library_query.to_lowercase();
        let mut games: Vec<GameEntry> = self
            .library
            .games()
            .iter()
            .filter(|g| {
                let meta = self.library.presentation(&g.id).unwrap_or_default();
                format!("{} {}", g.display_name, meta.alias)
                    .to_lowercase()
                    .contains(&query)
            })
            .cloned()
            .collect();
        if self.recent_first {
            games.sort_by_key(|g| {
                std::cmp::Reverse(
                    self.library
                        .presentation(&g.id)
                        .unwrap_or_default()
                        .last_played,
                )
            });
        }
        if games.is_empty() {
            ui.add_space(80.0);
            ui.vertical_centered(|ui| {
                ui.label(
                    RichText::new(if self.library.games().is_empty() {
                        "No games yet"
                    } else {
                        "No matching programs"
                    })
                    .heading()
                    .color(ui.visuals().weak_text_color()),
                );
                ui.add_space(8.0);
                ui.label(
                    RichText::new(
                        "Import a game you own: a .zip, .cab or the download as it came.",
                    )
                    .color(ui.visuals().weak_text_color()),
                );
                ui.add_space(20.0);
                if ui.button(RichText::new("Import game").strong()).clicked() {
                    self.spawn_import_dialog();
                }
            });
            return;
        }
        ScrollArea::vertical().show(ui, |ui| {
            let gap = 16.0;
            let card_size = Vec2::new(224.0, 304.0);
            let columns = ((ui.available_width() + gap) / (card_size.x + gap))
                .floor()
                .max(1.0) as usize;
            egui::Grid::new("library_grid")
                .num_columns(columns)
                .min_col_width(card_size.x)
                .max_col_width(card_size.x)
                .min_row_height(card_size.y)
                .spacing(Vec2::splat(gap))
                .show(ui, |ui| {
                    for (index, game) in games.iter().enumerate() {
                        self.ui_game_card(ui, game, card_size);
                        if (index + 1) % columns == 0 {
                            ui.end_row();
                        }
                    }
                    if !games.len().is_multiple_of(columns) {
                        ui.end_row();
                    }
                });
        });
    }

    fn ui_game_card(&mut self, ui: &mut egui::Ui, game: &GameEntry, size: Vec2) {
        let (card_rect, card_response) = ui.allocate_exact_size(size, Sense::click());
        let frame = egui::Frame::none()
            .fill(theme_color(self.ui_theme.panel2))
            .stroke(egui::Stroke::new(1.0_f32, theme_color(self.ui_theme.line)))
            .rounding(14.0)
            .inner_margin(0.0);
        ui.painter().add(frame.paint(card_rect));

        let menu_rect = Rect::from_min_size(
            Pos2::new(card_rect.right() - 52.0, card_rect.top() + 4.0),
            Vec2::splat(44.0),
        );
        let mut menu_ui = ui.child_ui(menu_rect, egui::Layout::right_to_left(egui::Align::Center));
        menu_ui.spacing_mut().interact_size = Vec2::splat(44.0);
        let menu_response = menu_ui.menu_button(RichText::new("···").size(20.0), |ui| {
            for (label, action) in [
                ("Back up written progress ZIP…", "export"),
                ("Restore progress…", "import"),
                ("Reset save…", "reset"),
            ] {
                if ui.button(label).clicked() {
                    self.spawn_save_action(game, action);
                    ui.close_menu();
                }
            }
            ui.separator();
            if ui.button("About / controls").clicked() {
                self.open_controls(&game.id);
                ui.close_menu();
            }
            if ui.button("Settings").clicked() {
                self.selected_game = Some(game.id.clone());
                self.game_settings_draft = Some((game.id.clone(), game.settings.clone()));
                self.screen = Screen::GameSettings;
                ui.close_menu();
            }
            if ui.button("Remove").clicked() {
                if let Err(e) = self.library.remove(&game.id) {
                    self.status = format!("Remove failed: {e}");
                } else {
                    self.status = format!("Removed {}", game.display_name);
                }
                ui.close_menu();
            }
        });

        let icon_rect = Rect::from_min_size(
            Pos2::new(card_rect.left(), card_rect.top() + 4.0),
            Vec2::new(size.x, 142.0),
        );
        let icon_size = (size.x - 24.0).clamp(56.0, 112.0);
        let mut icon_ui = ui.child_ui(
            icon_rect,
            egui::Layout::centered_and_justified(egui::Direction::TopDown),
        );
        let mut drew_icon = false;
        if let Some(path) = game.icon_path(self.library.root()) {
            if let Ok(bytes) = std::fs::read(path) {
                if let Ok(image) = load_image_bytes(&bytes) {
                    let texture = self.icon_cache.entry(game.id.clone()).or_insert_with(|| {
                        icon_ui.ctx().load_texture(
                            format!("icon-{}", game.id),
                            image,
                            egui::TextureOptions::LINEAR,
                        )
                    });
                    icon_ui.add(
                        egui::Image::from_texture(&*texture)
                            .fit_to_exact_size(Vec2::splat(icon_size)),
                    );
                    drew_icon = true;
                }
            }
        }
        if !drew_icon {
            icon_ui.label(RichText::new("CE").size(48.0));
        }

        let label_rect = Rect::from_min_size(
            Pos2::new(card_rect.left(), card_rect.bottom() - 140.0),
            Vec2::new(size.x, 140.0),
        );
        ui.painter().rect_filled(
            label_rect,
            egui::Rounding {
                nw: 0.0,
                ne: 0.0,
                sw: 14.0,
                se: 14.0,
            },
            theme_color(self.ui_theme.panel),
        );
        let text_rect = Rect::from_min_size(
            Pos2::new(label_rect.left() + 12.0, label_rect.top() + 8.0),
            Vec2::new(size.x - 24.0, 124.0),
        );
        let mut text_ui = ui.child_ui(text_rect, egui::Layout::top_down(egui::Align::Min));
        text_ui.set_width(size.x - 24.0);
        text_ui.set_height(124.0);
        let publisher = game
            .provider
            .as_deref()
            .filter(|value| !value.trim().is_empty())
            .unwrap_or("Unknown");
        text_ui.add(
            egui::Label::new(
                RichText::new({
                    let m = self.library.presentation(&game.id).unwrap_or_default();
                    if m.alias.is_empty() {
                        game.display_name.clone()
                    } else {
                        m.alias
                    }
                })
                .strong()
                .size(17.0),
            )
            .wrap(true),
        );
        text_ui.add(
            egui::Label::new(
                RichText::new(publisher)
                    .size(13.0)
                    .color(ui.visuals().weak_text_color()),
            )
            .truncate(true),
        );

        match self.library.effective_controls(&game.id) {
            Ok((controls, overridden)) => {
                text_ui.label(
                    RichText::new(format!(
                        "{} · {}",
                        controls.profile.label(),
                        if overridden {
                            "Program override"
                        } else {
                            "Inherited defaults"
                        }
                    ))
                    .size(12.0),
                );
            }
            Err(error) => {
                text_ui.label(RichText::new(format!("Controls error: {error}")).size(12.0));
            }
        }
        if card_response.clicked() && !menu_response.response.clicked() {
            if let Some(id) = ui.ctx().memory(|memory| memory.focused()) {
                ui.ctx().memory_mut(|memory| memory.surrender_focus(id));
            }
            self.spawn_run(game);
        }
    }

    fn ui_settings(&mut self, ui: &mut egui::Ui) {
        let Some(mut draft) = self.config_draft.take() else {
            return;
        };
        let mut save_clicked = false;
        let mut cancel_clicked = false;
        ui.heading("Launcher settings");
        ui.add_space(8.0);
        self.theme_picker(ui);
        let library_root = self.library.root().display().to_string();
        if ui
            .button("Default controls / touch layout / volume")
            .clicked()
        {
            self.open_controls("");
        }
        ui.collapsing("Advanced diagnostics and window preferences", |ui| {
            egui::Grid::new("settings_grid")
                .num_columns(2)
                .spacing(Vec2::new(12.0, 8.0))
                .show(ui, |ui| {
                    ui.label("Default CPU backend");
                    egui::ComboBox::from_id_source("cpu_backend")
                        .selected_text(draft.default_cpu_backend.label())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut draft.default_cpu_backend,
                                CpuBackendPref::Stub,
                                CpuBackendPref::Stub.label(),
                            );
                            if cfg!(feature = "unicorn") {
                                ui.selectable_value(
                                    &mut draft.default_cpu_backend,
                                    CpuBackendPref::Unicorn,
                                    CpuBackendPref::Unicorn.label(),
                                );
                            }
                        });
                    ui.end_row();

                    ui.label("Verbosity (0..3)");
                    ui.add(egui::Slider::new(&mut draft.verbosity, 0..=3));
                    ui.end_row();

                    ui.label("Apply diagnostic run limits");
                    ui.checkbox(&mut draft.apply_run_limits, "").on_hover_text("Apply each program’s slice limit and stop on unimplemented calls. Disabled for normal play; takes effect on next launch.");
                    ui.end_row();

                    ui.label("Show FPS overlay");
                    ui.checkbox(&mut draft.show_fps, "");
                    ui.end_row();

                    ui.label("Show backend status");
                    ui.checkbox(&mut draft.show_backend_log, "");
                    ui.end_row();
                    ui.label("Borderless fullscreen");
                    if ui.checkbox(&mut draft.fullscreen, "").changed() {
                        ui.ctx()
                            .send_viewport_cmd(egui::ViewportCommand::Fullscreen(draft.fullscreen));
                    }
                    ui.end_row();

                    ui.label("Library root");
                    ui.label(library_root);
                    ui.end_row();
                });
        });

        ui.add_space(12.0);
        ui.collapsing("Legacy keyboard defaults", |ui| {
            self.ui_keybindings(ui, &mut draft)
        });

        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                save_clicked = true;
            }
            if ui.button("Cancel").clicked() {
                cancel_clicked = true;
            }
        });
        if save_clicked {
            *self.library.config_mut() = draft;
            if let Err(e) = self.library.save() {
                self.status = format!("Could not save settings: {e}");
            } else {
                self.status = "Settings saved.".to_string();
            }
            self.screen = Screen::Library;
        } else if cancel_clicked {
            self.screen = Screen::Library;
        } else {
            self.config_draft = Some(draft);
        }
    }

    /// Keyboard editor: one row per guest button, listing the host keys
    /// bound to it with an ✕ to drop each and an "Add key" that waits
    /// for the next key press.
    ///
    /// Edits land in `draft`, so they are only written to `config.json`
    /// when the user saves the settings screen — and once written they
    /// are what [`Self::handle_physical_keyboard`] reads on every key
    /// event, which is what makes a rebind survive a restart.
    fn ui_keybindings(&mut self, ui: &mut egui::Ui, draft: &mut LauncherConfig) {
        ui.label(RichText::new("Keyboard").strong());
        ui.label(
            RichText::new(
                "Host keys for the Windows Mobile buttons. A key only ever drives one \
                 button, so binding it somewhere new takes it away from where it was.",
            )
            .small()
            .color(ui.visuals().weak_text_color()),
        );
        ui.add_space(6.0);

        // Copied out so the closures below can mutate the capture state
        // without holding a second borrow of `self`.
        let mut capture = self.binding_capture;
        let mut reset_clicked = false;

        // A capture is armed: the next key press the window sees becomes
        // the binding. Read the events directly rather than waiting for
        // a widget to be focused, because the key the user wants may be
        // one egui would otherwise treat as navigation (Tab, arrows).
        if let Some(button) = capture {
            let captured = ui.ctx().input(|input| {
                input.events.iter().find_map(|event| match event {
                    egui::Event::Key {
                        key,
                        pressed: true,
                        repeat: false,
                        ..
                    } => Some(*key),
                    _ => None,
                })
            });
            if let Some(key) = captured {
                draft.keybindings.bind(button, key.name());
                capture = None;
            }
        }

        egui::Grid::new("keybindings_grid")
            .num_columns(3)
            .spacing(Vec2::new(12.0, 6.0))
            .show(ui, |ui| {
                for button in GuestButton::ALL {
                    ui.label(button.label());
                    let keys: Vec<String> = draft.keybindings.keys_for(button).to_vec();
                    ui.horizontal(|ui| {
                        if keys.is_empty() {
                            ui.label(
                                RichText::new("unbound")
                                    .small()
                                    .color(ui.visuals().weak_text_color()),
                            );
                        }
                        for key in &keys {
                            ui.label(RichText::new(key).monospace());
                            if ui.small_button("✕").clicked() {
                                draft.keybindings.unbind(button, key);
                            }
                        }
                    });
                    if capture == Some(button) {
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new("press a key…")
                                    .small()
                                    .color(Color32::LIGHT_BLUE),
                            );
                            if ui.small_button("Cancel").clicked() {
                                capture = None;
                            }
                        });
                    } else if ui.button("Add key").clicked() {
                        capture = Some(button);
                    }
                    ui.end_row();
                }
            });

        ui.add_space(6.0);
        if ui.button("Reset keys to defaults").clicked() {
            reset_clicked = true;
        }
        if reset_clicked {
            draft.keybindings = pocket_library::KeyBindings::default();
            capture = None;
        }
        self.binding_capture = capture;
    }

    fn ui_game_settings(&mut self, ui: &mut egui::Ui) {
        let Some((id, mut draft)) = self.game_settings_draft.take() else {
            return;
        };
        let display_name = self
            .library
            .get(&id)
            .map(|g| g.display_name.clone())
            .unwrap_or_else(|| id.clone());
        let mut save_clicked = false;
        let mut cancel_clicked = false;
        ui.heading(format!("Settings: {display_name}"));
        ui.add_space(8.0);
        egui::Grid::new("game_settings_grid")
            .num_columns(2)
            .spacing(Vec2::new(12.0, 8.0))
            .show(ui, |ui| {
                ui.label("CPU backend");
                egui::ComboBox::from_id_source("game_cpu_backend")
                    .selected_text(draft.cpu_backend.label())
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut draft.cpu_backend,
                            CpuBackendPref::Stub,
                            CpuBackendPref::Stub.label(),
                        );
                        if cfg!(feature = "unicorn") {
                            ui.selectable_value(
                                &mut draft.cpu_backend,
                                CpuBackendPref::Unicorn,
                                CpuBackendPref::Unicorn.label(),
                            );
                        }
                    });
                ui.end_row();

                ui.label("Max slices");
                ui.add(egui::DragValue::new(&mut draft.max_slices).clamp_range(1..=u64::MAX));
                ui.end_row();

                ui.label("Instructions / slice");
                ui.add(
                    egui::DragValue::new(&mut draft.instructions_per_slice)
                        .clamp_range(1..=u64::MAX),
                );
                ui.end_row();

                ui.label("Screen");
                egui::ComboBox::from_id_source("game_screen")
                    .selected_text(draft.screen.label())
                    .show_ui(ui, |ui| {
                        for pref in [
                            ScreenPref::Portrait,
                            ScreenPref::Landscape,
                            ScreenPref::SmallPortrait,
                            ScreenPref::Wvga,
                            ScreenPref::Hpc,
                        ] {
                            ui.selectable_value(&mut draft.screen, pref, pref.label());
                        }
                    });
                ui.end_row();

                // Presentation-only: the guest keeps rendering into
                // whatever `Screen` says. This is the knob for a
                // landscape game that shipped as a 240x320 portrait
                // build — forcing `Screen` to 320x240 makes some of
                // them lay out against geometry they never shipped
                // against, turning the frame instead always works.
                ui.label("Rotate display");
                egui::ComboBox::from_id_source("game_rotation_pref")
                    .selected_text(draft.rotation.label())
                    .show_ui(ui, |ui| {
                        for choice in RotationPref::ALL {
                            ui.selectable_value(&mut draft.rotation, choice, choice.label());
                        }
                    });
                ui.end_row();

                ui.label("Halt on unimplemented API");
                ui.checkbox(&mut draft.halt_on_unimplemented, "");
                ui.end_row();
            });

        // Satellite libraries are not a setting — they are a fact about
        // what got imported. Showing them here is how a user confirms
        // that picking `solitare.exe` also brought `pegcards.dll` in,
        // without having to go digging in the library directory.
        let companions: Vec<String> = self
            .library
            .get(&id)
            .map(|g| {
                g.companions
                    .iter()
                    .filter_map(|p| p.file_name())
                    .map(|n| n.to_string_lossy().to_string())
                    .collect()
            })
            .unwrap_or_default();
        if !companions.is_empty() {
            ui.add_space(8.0);
            ui.label(format!("Support libraries: {}", companions.join(", ")));
        }

        ui.add_space(12.0);
        ui.horizontal(|ui| {
            if ui.button("Save").clicked() {
                save_clicked = true;
            }
            if ui.button("Cancel").clicked() {
                cancel_clicked = true;
            }
        });
        if save_clicked {
            if let Err(e) = self.library.update_settings(&id, draft) {
                self.status = format!("Could not save game settings: {e}");
            } else {
                self.status = "Game settings saved.".to_string();
            }
            self.screen = Screen::Library;
        } else if cancel_clicked {
            self.screen = Screen::Library;
        } else {
            self.game_settings_draft = Some((id, draft));
        }
    }

    fn ui_run(&mut self, ui: &mut egui::Ui) {
        ui.heading(if self.input_tx.is_some() {
            "Playing"
        } else {
            "Program ended"
        });
        if ui
            .add_enabled(
                self.input_tx.is_some(),
                egui::Button::new(format!("Menu / Pause ({})", self.controls.menu_key)),
            )
            .clicked()
        {
            self.menu_open = true;
            self.release_all_keys();
        }
        ui.add_space(8.0);
        if let Some(name) = self.running_game.as_ref() {
            ui.label(format!("Running {name}…"));
        }
        ui.add_space(8.0);
        ui.horizontal(|ui| {
            ui.label("Rotation");
            let mut rotation = self.game_rotation;
            egui::ComboBox::from_id_source("game_rotation")
                .selected_text(rotation.label())
                .show_ui(ui, |ui| {
                    for choice in RotationPref::ALL {
                        ui.selectable_value(&mut rotation, choice, choice.label());
                    }
                });
            if rotation != self.game_rotation {
                self.game_rotation = rotation;
                self.persist_rotation(rotation);
            }
            if ui.button("Fullscreen (F11)").clicked() {
                let fullscreen = ui
                    .ctx()
                    .input(|input| input.viewport().fullscreen.unwrap_or(false));
                ui.ctx()
                    .send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fullscreen));
            }
            if ui
                .button("Game-only fullscreen")
                .on_hover_text("Show only the centered game. Press F11 to return.")
                .clicked()
            {
                self.set_game_only_fullscreen(ui.ctx(), true);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if ui.button("Return to library").clicked() {
                    // Best-effort — if the runner thread already
                    // exited the channel will just drop the message.
                    if let Some(tx) = self.input_tx.as_ref() {
                        let _ = tx.send(InputCommand::Stop);
                    }
                    self.release_all_keys();
                    self.screen = Screen::Library;
                }
            });
        });
        ui.add_space(6.0);
        ui.horizontal_top(|ui| {
            self.ui_run_screen(ui);
            ui.add_space(12.0);
            self.ui_virtual_pad(ui);
        });
        ui.add_space(8.0);
        if let Some(s) = self.last_frame_status.as_ref() {
            ui.collapsing("Diagnostic details", |ui| {
                egui::ScrollArea::vertical()
                    .max_height(140.0)
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(s)
                                .small()
                                .color(ui.visuals().weak_text_color()),
                        );
                    });
            });
        }
    }

    /// Render the live framebuffer (or placeholder) and forward any
    /// pointer presses on it as `WM_LBUTTONDOWN` / `WM_LBUTTONUP`
    /// events, with stylus coordinates in whatever resolution the game
    /// is actually running at.
    fn ui_run_screen(&mut self, ui: &mut egui::Ui) {
        let Some(tex) = self.last_frame_texture.clone() else {
            ui.allocate_ui(
                Vec2::new(FB_WIDTH as f32 * 2.0, FB_HEIGHT as f32 * 2.0),
                |ui| {
                    ui.label(if self.frame_rx.is_some() {
                        "Loading program display..."
                    } else {
                        "No captured frame. Open the library to try again."
                    });
                },
            );
            return;
        };
        let size = tex.size_vec2();
        // Keep the normal Run-screen preview at 2x unless it would
        // exceed the space shared with the virtual control pad.
        let available = ui.available_size();
        let rotated_size = if self.game_rotation.is_quarter_turn() {
            Vec2::new(size.y, size.x)
        } else {
            size
        };
        let scale = 2.0_f32
            .min(available.x / rotated_size.x)
            .min(available.y / rotated_size.y)
            .max(0.1);
        let display_size = rotated_size * scale;
        let (rect, _response) = ui.allocate_exact_size(display_size, Sense::click_and_drag());
        let painter = ui.painter();
        let ctx = ui.ctx().clone();
        self.paint_game_texture(painter, &ctx, &tex, rect, size, true);
    }

    fn ui_game_only(&mut self, ctx: &egui::Context) {
        // Keep EQ's framebuffer geometry anchored to a root panel, including
        // when the guest is paused and stops producing new frames.
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::BLACK))
            .show(ctx, |ui| {
                let Some(tex) = self.last_frame_texture.clone() else {
                    return;
                };
                let size = tex.size_vec2();
                let rect = fullscreen_frame_rect(ui.max_rect(), size, self.game_rotation);
                self.paint_game_texture(ui.painter(), ctx, &tex, rect, size, false);
            });
    }

    fn paint_game_texture(
        &mut self,
        painter: &egui::Painter,
        ctx: &egui::Context,
        tex: &egui::TextureHandle,
        rect: Rect,
        size: Vec2,
        show_fps: bool,
    ) {
        let mut uv = rotation_uv(self.game_rotation);
        let (cx, cy) = self.controls.zoom_center();
        for p in &mut uv {
            p.x = cx + (p.x - 0.5) / self.controls.zoom;
            p.y = cy + (p.y - 0.5) / self.controls.zoom;
        }
        let mut mesh = Mesh::with_texture(tex.id());
        let idx = mesh.vertices.len() as u32;
        for (pos, uv) in [
            (rect.left_top(), uv[0]),
            (rect.right_top(), uv[1]),
            (rect.left_bottom(), uv[2]),
            (rect.right_bottom(), uv[3]),
        ] {
            mesh.vertices.push(egui::epaint::Vertex {
                pos,
                uv,
                color: Color32::WHITE,
            });
        }
        mesh.indices
            .extend_from_slice(&[idx, idx + 1, idx + 2, idx + 2, idx + 1, idx + 3]);
        painter.add(egui::Shape::mesh(mesh));
        if show_fps && self.library.config().show_fps {
            let overlay_rect =
                Rect::from_min_size(rect.min + Vec2::new(6.0, 6.0), Vec2::new(390.0, 24.0));
            painter.rect_filled(overlay_rect, 4.0, Color32::from_black_alpha(190));
            painter.text(
                overlay_rect.min + Vec2::new(6.0, 4.0),
                egui::Align2::LEFT_TOP,
                self.frame_stats.overlay_text(),
                egui::FontId::monospace(13.0),
                Color32::LIGHT_GREEN,
            );
        }
        self.handle_pointer(ctx, &rect, size);
    }

    /// `size` is the guest framebuffer's own dimensions in pixels — the
    /// coordinate space the game expects to receive stylus events in.
    ///
    /// Press state comes from [`pointer_held_in`] rather than from
    /// egui's click/drag bookkeeping so that a stylus held on the same
    /// spot stays down indefinitely — a tap-and-hold is how a Pocket PC
    /// game is told "keep going", and egui's click timeout would have
    /// let go for the user.
    fn handle_pointer(&mut self, ctx: &egui::Context, rect: &Rect, size: Vec2) {
        let host_menu_press = self.game_only_fullscreen
            && ctx.input(|input| {
                input
                    .pointer
                    .press_origin()
                    .is_some_and(|point| fullscreen_menu_rect().contains(point))
            });
        let held = !host_menu_press
            && !self.menu_open
            && self.controls_draft.is_none()
            && pointer_held_in(ctx, rect);
        let pos = ctx.input(|input| input.pointer.latest_pos());
        let coords = pos.map(|pos| {
            // Map UI-space coords to guest framebuffer pixels. The panel
            // scales whatever geometry the game actually runs at, so both
            // factors have to come from the live texture rather than the
            // compile-time constants — otherwise a 480×320 game would see
            // taps land on the wrong pixels.
            let (x, y) =
                rotated_pointer_to_game(pos - rect.min, rect.size(), size, self.game_rotation);
            let (cx, cy) = self.controls.zoom_center();
            (
                ((x as f32 - size.x / 2.0) / self.controls.zoom + size.x * cx)
                    .clamp(0.0, size.x - 1.0) as u16,
                ((y as f32 - size.y / 2.0) / self.controls.zoom + size.y * cy)
                    .clamp(0.0, size.y - 1.0) as u16,
            )
        });
        match (self.pointer_down_at, held) {
            (None, true) => {
                if let Some(id) = ctx.memory(|memory| memory.focused()) {
                    ctx.memory_mut(|memory| memory.surrender_focus(id));
                }
                if let Some((x, y)) = coords {
                    self.send_input(InputEvent::PointerDown { x, y });
                    self.pointer_down_at = Some((x, y));
                }
            }
            (Some(last), true) => {
                // Only report movement that actually changed a guest
                // pixel; a still stylus should not flood the pump.
                if let Some((x, y)) = coords {
                    if (x, y) != last {
                        self.send_input(InputEvent::PointerMove { x, y });
                        self.pointer_down_at = Some((x, y));
                    }
                }
            }
            (Some(last), false) => {
                let (x, y) = coords.unwrap_or(last);
                self.send_input(InputEvent::PointerUp { x, y });
                self.pointer_down_at = None;
            }
            (None, false) => {}
        }
    }

    /// j2me-loader-inspired virtual gamepad: a D-pad on the left and
    /// three action buttons (A / B / Start) plus two soft keys on
    /// the right. Each button drives a `WM_KEYDOWN`/`WM_KEYUP` pair
    /// while held.
    fn ui_virtual_pad(&mut self, ui: &mut egui::Ui) {
        if (self.controls.auto_hide_gamepad && self.gamepad_connected > 0)
            || (self.controls.keys.is_empty() && !self.controls.keyboard)
        {
            return;
        }
        ui.vertical(|ui| {
            let panel_mode = self.controls.profile == Profile::Ppc2003;
            if panel_mode {
                ui.colored_label(theme_color(self.ui_theme.flat), "POCKET PC 2003");
                ui.separator();
            } else {
                ui.label(self.controls.profile.label());
            }
            let keys = self.controls.keys.clone();
            let area = ui
                .allocate_exact_size(Vec2::new(240.0, 260.0), Sense::hover())
                .0;
            let layout = if self.game_rotation.is_quarter_turn() {
                self.controls.landscape.clone()
            } else {
                self.controls.portrait.clone()
            };
            for (i, vk) in keys.iter().enumerate() {
                let panel_button = self.controls.panel_button(*vk);
                let pulse = panel_button.is_some_and(|b| b.mode == "click");
                let default_position = panel_button
                    .map(|b| ButtonPosition {
                        vk: *vk,
                        x: b.x,
                        y: b.y,
                        size: b.size,
                    })
                    .unwrap_or(ButtonPosition {
                        vk: *vk,
                        x: 0.1 + (i % 5) as f32 * 0.2,
                        y: 0.14 + (i / 5) as f32 * 0.22,
                        size: 0.15,
                    });
                let position = layout
                    .iter()
                    .find(|p| p.vk == *vk)
                    .cloned()
                    .unwrap_or(default_position);
                let side = (position.size * area.width().min(area.height())).max(44.0);
                let center = Pos2::new(
                    (area.left() + position.x * area.width())
                        .clamp(area.left() + side / 2.0, area.right() - side / 2.0),
                    (area.top() + position.y * area.height())
                        .clamp(area.top() + side / 2.0, area.bottom() - side / 2.0),
                );
                let rect = Rect::from_center_size(center, Vec2::splat(side));
                let response = ui
                    .scope(|ui| {
                        ui.spacing_mut().button_padding = Vec2::splat(2.0);
                        ui.put(
                            rect,
                            egui::Button::new(
                                RichText::new(
                                    panel_button
                                        .map(|b| b.label.clone())
                                        .unwrap_or_else(|| control_label(*vk)),
                                )
                                .size(13.0),
                            )
                            .wrap(false)
                            .fill(Color32::from_rgba_unmultiplied(
                                60,
                                75,
                                95,
                                (self.controls.opacity * 255.0) as u8,
                            )),
                        )
                    })
                    .inner;
                if pulse {
                    if response.clicked() && !self.menu_open && self.controls_draft.is_none() {
                        // The PPC2003 skin emits one down/up for Enter/F keys,
                        // even after a long hold. Keep other input sources held.
                        if self.held.press(InputSource::Activation, *vk) {
                            self.send_input(InputEvent::KeyDown { vk: *vk });
                        }
                        if self.held.release(InputSource::Activation, *vk) {
                            self.send_input(InputEvent::KeyUp { vk: *vk });
                        }
                    }
                    continue;
                }
                if response.clicked()
                    && !response.clicked_by(egui::PointerButton::Primary)
                    && !self.menu_open
                    && self.controls_draft.is_none()
                {
                    if self.held.press(InputSource::Activation, *vk) {
                        self.send_input(InputEvent::KeyDown { vk: *vk });
                    }
                    self.activation_releases.retain(|(key, _)| key != vk);
                    self.activation_releases
                        .push((*vk, Instant::now() + Duration::from_millis(64)));
                }
                let held = !self.menu_open
                    && self.controls_draft.is_none()
                    && pointer_held_in(ui.ctx(), &response.rect);
                let old = self.held.is_held_by(InputSource::Pointer, *vk);
                if held && !old && self.held.press(InputSource::Pointer, *vk) {
                    self.send_input(InputEvent::KeyDown { vk: *vk });
                }
                if !held && old && self.held.release(InputSource::Pointer, *vk) {
                    self.send_input(InputEvent::KeyUp { vk: *vk });
                }
            }
            if self.controls.keyboard {
                ui.text_edit_multiline(&mut self.guest_text);
                if ui.button("Send text").clicked() {
                    for unit in self.guest_text.encode_utf16() {
                        self.send_input(InputEvent::Text { unit });
                    }
                    self.guest_text.clear();
                }
            }
        });
    }

    /// Write a rotation picked on the Run screen back to the game's
    /// `game.json`, so the choice survives the run it was made in.
    fn persist_rotation(&mut self, rotation: RotationPref) {
        let Some(id) = self.running_game_id.clone() else {
            return;
        };
        let Some(mut settings) = self.library.get(&id).map(|g| g.settings.clone()) else {
            return;
        };
        settings.rotation = rotation;
        if let Err(e) = self.library.update_settings(&id, settings) {
            self.status = format!("Could not save rotation: {e}");
        }
    }

    fn send_input(&self, ev: InputEvent) {
        if let Some(tx) = self.input_tx.as_ref() {
            let _ = tx.send(InputCommand::Input(ev));
        }
    }

    fn release_all_keys(&mut self) {
        self.activation_releases.clear();
        self.host_keys.clear();
        for vk in self.held.drain_all() {
            self.send_input(InputEvent::KeyUp { vk });
        }
    }

    fn set_game_only_fullscreen(&mut self, ctx: &egui::Context, enabled: bool) {
        if self.game_only_fullscreen == enabled {
            return;
        }
        self.game_only_fullscreen = enabled;
        if enabled {
            let fullscreen = ctx.input(|input| input.viewport().fullscreen.unwrap_or(false));
            self.fullscreen_before_game_only = Some(fullscreen);
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(true));
        } else {
            let fullscreen = self.fullscreen_before_game_only.take().unwrap_or(false);
            ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(fullscreen));
        }
    }

    fn handle_gamepads(&mut self, ctx: &egui::Context) {
        if self.input_tx.is_none() {
            return;
        }
        let pads = crate::gamepad::poll();
        self.gamepad_connected = pads.len();
        let menu_held = pads.iter().any(|p| p.pressed.contains(&"PadL3"))
            && self.controls.mapped_key("PadL3").is_none();
        if menu_held && !self.gamepad_menu_held && self.screen == Screen::Run {
            self.menu_open = !self.menu_open;
            self.release_all_keys();
        }
        self.gamepad_menu_held = menu_held;
        let mut next = std::collections::HashMap::new();
        if self.screen == Screen::Run
            && !self.menu_open
            && self.controls_draft.is_none()
            && ctx.input(|i| i.viewport().focused.unwrap_or(true))
        {
            for pad in pads {
                for name in pad.pressed {
                    if name == "PadL3" && self.controls.mapped_key(name).is_none() {
                        continue;
                    }
                    if let Some(vk) = self
                        .controls
                        .mapped_key(name)
                        .or_else(|| pocket_library::controls::gamepad_default(name))
                    {
                        if vk != 0 {
                            next.insert((pad.index, name), vk);
                        }
                    }
                }
            }
        }
        let old: std::collections::HashSet<_> = self.held.gamepad.values().copied().collect();
        self.held.gamepad = next;
        let new: std::collections::HashSet<_> = self.held.gamepad.values().copied().collect();
        for vk in old.difference(&new) {
            if !self.held.is_held(*vk) {
                self.send_input(InputEvent::KeyUp { vk: *vk });
            }
        }
        for vk in new.difference(&old) {
            if !self.held.keyboard.contains(vk)
                && !self.held.pointer.contains(vk)
                && !self.held.modifiers.contains(vk)
                && !self.held.activation.contains(vk)
            {
                self.send_input(InputEvent::KeyDown { vk: *vk });
            }
        }
    }

    fn release_host_keyboard(&mut self) {
        self.host_keys.clear();
        let keys: std::collections::HashSet<_> = self
            .held
            .keyboard
            .union(&self.held.modifiers)
            .copied()
            .collect();
        self.held.keyboard.clear();
        self.held.modifiers.clear();
        for vk in keys {
            if !self.held.is_held(vk) {
                self.send_input(InputEvent::KeyUp { vk });
            }
        }
    }

    fn route_raw_keyboard(&mut self, ctx: &egui::Context, raw: &mut egui::RawInput) {
        if self.screen != Screen::Run || self.input_tx.is_none() {
            return;
        }
        let function_shortcut = self
            .controls
            .menu_key
            .strip_prefix('F')
            .is_some_and(|s| s.parse::<u8>().is_ok_and(|n| (1..=12).contains(&n)));
        if (function_shortcut || !ctx.wants_keyboard_input()) && raw.events.iter().any(|event|matches!(event,egui::Event::Key{key,pressed:true,repeat:false,..} if pocket_library::keybindings::keys_match(key.name(), &self.controls.menu_key))) {
            self.menu_open = !self.menu_open;
            log::debug!("Host menu shortcut: open={}, game_only={}", self.menu_open, self.game_only_fullscreen);
            self.release_all_keys();
            raw.events.retain(|event|!matches!(event,egui::Event::Key{key,..} if pocket_library::keybindings::keys_match(key.name(), &self.controls.menu_key)));
            return;
        }
        if raw.events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Key {
                    key: egui::Key::F12,
                    pressed: true,
                    repeat: false,
                    ..
                }
            )
        }) {
            self.request_screenshot(ctx);
            raw.events.retain(|event| {
                !matches!(
                    event,
                    egui::Event::Key {
                        key: egui::Key::F12,
                        ..
                    }
                )
            });
            return;
        }
        // The host fullscreen exit must work while a pause menu or text
        // field owns focus as well as while EQ owns the keyboard.
        if raw.events.iter().any(|event| {
            matches!(
                event,
                egui::Event::Key {
                    key: egui::Key::F11,
                    pressed: true,
                    repeat: false,
                    ..
                }
            )
        }) {
            if self.game_only_fullscreen {
                self.set_game_only_fullscreen(ctx, false);
            } else {
                let fullscreen = ctx.input(|input| input.viewport().fullscreen.unwrap_or(false));
                ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fullscreen));
            }
            raw.events.retain(|event| {
                !matches!(
                    event,
                    egui::Event::Key {
                        key: egui::Key::F11,
                        ..
                    }
                )
            });
            return;
        }
        if self.menu_open
            || self.controls_draft.is_some()
            || !raw.focused
            || ctx.wants_keyboard_input()
        {
            self.release_host_keyboard();
            return;
        }
        self.handle_physical_keyboard(ctx, raw.events.clone(), raw.modifiers);
        // Guest Tab/Escape/Space must not also navigate or activate host UI.
        // A focused host field/button takes the other branch before filtering.
        raw.events.retain(|event| {
            !matches!(
                event,
                egui::Event::Key { .. } | egui::Event::Text(_) | egui::Event::Paste(_)
            )
        });
    }

    fn handle_physical_keyboard(
        &mut self,
        ctx: &egui::Context,
        events: Vec<egui::Event>,
        modifiers: egui::Modifiers,
    ) {
        let mut targets = std::collections::HashSet::new();
        for (name, pressed, fallback) in
            [("Ctrl", modifiers.ctrl, 17), ("Shift", modifiers.shift, 16)]
        {
            if pressed && self.controls.profile != Profile::Text {
                let vk = self
                    .controls
                    .mapped_key(name)
                    .or_else(|| self.library.config().keybindings.vk_for_key(name))
                    .unwrap_or(fallback);
                if vk != 0 {
                    targets.insert(vk);
                }
            }
        }
        let old = self.held.modifiers.clone();
        for vk in old.difference(&targets) {
            if self.held.release(InputSource::Modifiers, *vk) {
                self.send_input(InputEvent::KeyUp { vk: *vk });
            }
        }
        for vk in targets.difference(&old) {
            if self.held.press(InputSource::Modifiers, *vk) {
                self.send_input(InputEvent::KeyDown { vk: *vk });
            }
        }
        for event in events {
            if self.controls.profile == Profile::Text {
                if let egui::Event::Text(ref text) | egui::Event::Paste(ref text) = event {
                    for unit in text.encode_utf16() {
                        self.send_input(InputEvent::Text { unit });
                    }
                    continue;
                }
            }
            let egui::Event::Key {
                key,
                pressed,
                repeat,
                ..
            } = event
            else {
                continue;
            };
            if key == egui::Key::F11 && pressed && !repeat {
                if self.game_only_fullscreen {
                    self.set_game_only_fullscreen(ctx, false);
                } else {
                    let fullscreen =
                        ctx.input(|input| input.viewport().fullscreen.unwrap_or(false));
                    ctx.send_viewport_cmd(egui::ViewportCommand::Fullscreen(!fullscreen));
                }
                continue;
            }
            let mapped = self.controls.mapped_key(key.name());
            let fallback = if self.controls.profile == Profile::Text {
                match key {
                    egui::Key::Backspace => Some(8),
                    egui::Key::Tab => Some(9),
                    egui::Key::Enter => Some(13),
                    egui::Key::ArrowLeft => Some(37),
                    egui::Key::ArrowRight => Some(39),
                    _ => None,
                }
            } else {
                self.library.config().keybindings.vk_for_key(key.name())
            };
            let Some(vk) = mapped.or(fallback) else {
                continue;
            };
            if vk == 0 {
                continue;
            }
            if pressed {
                if !repeat
                    && self.host_keys.insert(key, vk).is_none()
                    && self
                        .host_keys
                        .values()
                        .filter(|value| **value == vk)
                        .count()
                        == 1
                    && self.held.press(InputSource::Keyboard, vk)
                {
                    self.send_input(InputEvent::KeyDown { vk });
                }
            } else if let Some(released) = self.host_keys.remove(&key) {
                if !self.host_keys.values().any(|value| *value == released)
                    && self.held.release(InputSource::Keyboard, released)
                {
                    self.send_input(InputEvent::KeyUp { vk: released });
                }
            }
        }
        if ctx.input(|input| input.viewport().close_requested()) {
            self.release_all_keys();
        }
    }

    fn spawn_import_dialog(&mut self) {
        let library_root = self.library.root().to_path_buf();
        let last_dir = self.library.config().last_import_dir.clone();
        let tx = self.events_tx.clone();
        std::thread::spawn(move || {
            let mut dialog = rfd::FileDialog::new()
                .set_title("Import Pocket PC game")
                // A single "Pocket PC game" filter that matches every
                // shape we know how to import keeps the dialog UX
                // simple — the user picks a file and we pick the
                // right loader from the extension below. `.dll` is
                // included so a user who mistakenly picks a support
                // library gets a clear error directing them to import
                // the `.exe` instead, rather than seeing nothing in
                // the picker at all.
                .add_filter(
                    "Pocket PC game or download (.zip / .cab / .exe / .rar)",
                    &[
                        "rar", "RAR", "cab", "CAB", "zip", "ZIP", "exe", "EXE", "dll", "DLL",
                    ],
                )
                .add_filter("Cabinet archive", &["cab", "CAB"])
                .add_filter("Zip archive", &["zip", "ZIP"])
                .add_filter("ARM PE executable", &["exe", "EXE"]);
            if let Some(d) = last_dir {
                dialog = dialog.set_directory(d);
            }
            // Several files at once (a game plus the DLLs it needs) are put
            // into one package first, as the browser version does.
            let Some(picked) = dialog.pick_files().filter(|p| !p.is_empty()) else {
                let _ = tx.send(UiEvent::ImportFinished(Err("cancelled".into())));
                return;
            };
            let path = if picked.len() == 1 {
                picked[0].clone()
            } else {
                match pocket_library::bundle::bundle_files(&picked, &library_root.join(".bundles"))
                {
                    Ok(path) => path,
                    Err(e) => {
                        let _ = tx.send(UiEvent::ImportFinished(Err(format!(
                            "Could not put the files together: {e}"
                        ))));
                        return;
                    }
                }
            };
            let result = (|| -> Result<String, String> {
                let mut lib = Library::open(&library_root).map_err(|e| e.to_string())?;
                let parent = picked[0]
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or(library_root.clone());
                lib.config_mut().last_import_dir = Some(parent);
                lib.save().map_err(|e| e.to_string())?;
                // The same importer as Android and the browser: a download that
                // is not a package is looked into through the knowledge base,
                // and a failure comes back explained.
                let imported = lib.import_any(&path);
                if picked.len() > 1 {
                    let _ = std::fs::remove_file(&path);
                }
                let name = match imported {
                    Ok(entry) => entry.display_name.clone(),
                    Err(e) => {
                        let problem = pocket_library::import_kb::KnowledgeBase::load(&library_root)
                            .explain_import(&e);
                        return Err(problem.message(pocket_library::import_problem::Lang::En));
                    }
                };
                Ok(match lib.last_import_note() {
                    Some(note) => format!("Imported {name}. Found inside the download: {note}."),
                    None => format!("Imported {name}."),
                })
            })();
            let _ = tx.send(UiEvent::ImportFinished(result));
        });
        self.status = "Importing...".to_string();
    }

    /// Export, import or reset a game's progress as the save zip shared with the
    /// Android and browser versions. Dialogs run off the UI thread.
    fn spawn_save_action(&mut self, game: &GameEntry, action: &'static str) {
        use pocket_library::save_archive;
        let game_dir = self.library.root().join("games").join(&game.id);
        let id = game.id.clone();
        let name = game.display_name.clone();
        let tx = self.events_tx.clone();
        std::thread::spawn(move || {
            let result = match action {
                "export" => rfd::FileDialog::new()
                    .set_title("Export save")
                    .set_file_name(format!("{id}-save.zip"))
                    .add_filter("Save zip", &["zip"])
                    .save_file()
                    .map(|path| {
                        std::fs::File::create(&path)
                            .and_then(|file| save_archive::export(&game_dir, file))
                            .map(|count| match count {
                                0 => format!("{name} has not saved anything yet."),
                                n => format!("Exported {n} file(s) of {name}."),
                            })
                            .map_err(|e| format!("Export failed: {e}"))
                    }),
                "import" => rfd::FileDialog::new()
                    .set_title("Import save")
                    .add_filter("Save zip", &["zip"])
                    .pick_file()
                    .map(|path| {
                        std::fs::File::open(&path)
                            .and_then(|file| save_archive::import(&game_dir, file))
                            .map(|n| format!("Restored {n} file(s) of {name}."))
                            .map_err(|e| format!("Import failed: {e}"))
                    }),
                _ => {
                    let sure = rfd::MessageDialog::new()
                        .set_title("Reset progress")
                        .set_description(format!(
                            "Delete all progress of {name}? Export a save first to keep it."
                        ))
                        .set_buttons(rfd::MessageButtons::OkCancel)
                        .show();
                    matches!(sure, rfd::MessageDialogResult::Ok).then(|| {
                        save_archive::reset(&game_dir)
                            .map(|_| format!("Cleared the progress of {name}."))
                            .map_err(|e| format!("Reset failed: {e}"))
                    })
                }
            };
            if let Some(result) = result {
                let _ = tx.send(UiEvent::SaveFinished(result));
            }
        });
    }

    fn spawn_run(&mut self, game: &GameEntry) {
        if self.frame_rx.is_some() {
            self.status = "The previous program is still stopping.".into();
            return;
        }
        self.controls = match self.library.effective_controls(&game.id) {
            Ok((value, _)) => value,
            Err(e) => {
                self.status = e.to_string();
                return;
            }
        };
        self.menu_open = false;
        self.pause_requested = false;
        let mut presentation = self.library.presentation(&game.id).unwrap_or_default();
        presentation.last_played = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|v| v.as_secs())
            .unwrap_or(0);
        if let Err(e) = self.library.write_presentation(&game.id, &presentation) {
            self.status = e.to_string();
            return;
        }
        self.last_frame_texture = None;
        self.last_frame_status = None;
        self.frame_stats.reset();
        // Start turned the way this game was saved: a portrait-rendered
        // landscape title is unplayable until it is turned, and the user
        // should not have to re-pick that on every launch.
        self.game_rotation = game.settings.rotation;
        self.screen = Screen::Run;
        self.running_game = Some(game.display_name.clone());
        self.running_game_id = Some(game.id.clone());
        let (frame_tx, frame_rx) = mpsc::channel();
        let (input_tx, input_rx) = mpsc::channel();
        self.frame_rx = Some(frame_rx);
        let _ = input_tx.send(InputCommand::SetVolume(
            (self.controls.volume * 1000.0) as u16,
        ));
        self.input_tx = Some(input_tx);
        self.held = HeldButtons::default();
        self.host_keys.clear();
        self.pointer_down_at = None;
        let library_root = self.library.root().to_path_buf();
        let game = game.clone();
        let tx = self.events_tx.clone();
        let runner = self.runner.clone();
        std::thread::spawn(move || {
            let outcome = runner.run_game(library_root, game, Some(frame_tx), Some(input_rx));
            let _ = tx.send(UiEvent::RunFinished(outcome));
        });
        self.status = "Starting emulator...".to_string();
    }
}

/// The visible host target and the stylus exclusion must have identical bounds.
fn fullscreen_menu_rect() -> Rect {
    Rect::from_min_size(Pos2::new(8.0, 8.0), Vec2::new(168.0, 44.0))
}

/// Is the primary pointer button held down, with the press having
/// started inside `rect`?
///
/// `Response::is_pointer_button_down_on` cannot answer this for a
/// widget that senses clicks: egui gives up on a press being a click
/// after `MAX_CLICK_DURATION` (0.8 s in 0.27) and clears the
/// interaction's `potential_click_id`, at which point the response
/// reports "not down" even though the user never let go. That is the
/// bug where holding a virtual button with the mouse released itself
/// after about a second while the Android pad — which tracks
/// ACTION_DOWN/UP itself — held fine. Reading the pointer state has no
/// such timeout, and keying off `press_origin` keeps the button held
/// even if the pointer drifts off it, which is what a user dragging
/// their thumb across a D-pad expects.
fn pointer_held_in(ctx: &egui::Context, rect: &Rect) -> bool {
    ctx.input(|input| {
        input.pointer.primary_down()
            && input
                .pointer
                .press_origin()
                .is_some_and(|origin| rect.contains(origin))
    })
}

fn rotated_pointer_to_game(
    local: Vec2,
    display_size: Vec2,
    game_size: Vec2,
    rotation: RotationPref,
) -> (u16, u16) {
    let u = (local.x / display_size.x).clamp(0.0, 1.0);
    let v = (local.y / display_size.y).clamp(0.0, 1.0);
    // Un-rotate: the guest only ever knows about its own unturned
    // panel, so a tap has to be mapped back through the same quarter
    // turn the presentation applied.
    let (x, y) = match rotation {
        RotationPref::None => (u, v),
        RotationPref::Cw90 => (v, 1.0 - u),
        RotationPref::Half => (1.0 - u, 1.0 - v),
        RotationPref::Ccw90 => (1.0 - v, u),
    };
    let max_x = game_size.x.max(1.0) as u32 - 1;
    let max_y = game_size.y.max(1.0) as u32 - 1;
    (
        ((x * game_size.x).floor() as u32).min(max_x) as u16,
        ((y * game_size.y).floor() as u32).min(max_y) as u16,
    )
}

impl eframe::App for PocketLauncher {
    fn raw_input_hook(&mut self, ctx: &egui::Context, input: &mut egui::RawInput) {
        self.route_raw_keyboard(ctx, input);
    }
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(captured) = ctx.input(|input| {
            input.events.iter().find_map(|event| {
                if let egui::Event::Screenshot { image, .. } = event {
                    Some(image.clone())
                } else {
                    None
                }
            })
        }) {
            if let Some(path) = self.screenshot_path.take() {
                let rgba: Vec<u8> = captured
                    .pixels
                    .iter()
                    .flat_map(|pixel| pixel.to_array())
                    .collect();
                self.status = match image::save_buffer(
                    &path,
                    &rgba,
                    captured.size[0] as u32,
                    captured.size[1] as u32,
                    image::ColorType::Rgba8,
                ) {
                    Ok(()) => format!("Screenshot saved: {}", path.display()),
                    Err(error) => format!("Could not save screenshot: {error}"),
                };
            }
        }
        let now = Instant::now();
        let expired: Vec<_> = self
            .activation_releases
            .iter()
            .filter(|(_, deadline)| *deadline <= now)
            .map(|(vk, _)| *vk)
            .collect();
        self.activation_releases
            .retain(|(_, deadline)| *deadline > now);
        for vk in expired {
            if self.held.release(InputSource::Activation, vk) {
                self.send_input(InputEvent::KeyUp { vk });
            }
        }
        self.drain_events(ctx);
        self.handle_gamepads(ctx);
        if self.input_tx.is_some() {
            let paused = self.screen != Screen::Run
                || self.menu_open
                || self.controls_draft.is_some()
                || !ctx.input(|i| i.viewport().focused.unwrap_or(true));
            if paused != self.pause_requested {
                self.release_all_keys();
                self.pause_requested = paused;
                self.send_command(InputCommand::SetPaused(paused));
            }
        }
        if self.game_only_fullscreen && self.screen == Screen::Run {
            self.ui_game_only(ctx);
            egui::Area::new("fullscreen_menu".into())
                .order(egui::Order::Foreground)
                .fixed_pos(Pos2::new(8.0, 8.0))
                .show(ctx, |ui| {
                    if ui
                        .add_sized(
                            fullscreen_menu_rect().size(),
                            egui::Button::new(format!("Menu ({})", self.controls.menu_key)),
                        )
                        .clicked()
                    {
                        self.menu_open = true;
                        log::debug!("Fullscreen host menu button opened");
                        self.release_all_keys();
                    }
                });
        } else {
            egui::TopBottomPanel::top("top").show(ctx, |ui| self.ui_top_bar(ui));
            egui::TopBottomPanel::bottom("status").show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new(&self.status).small());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.label(
                            RichText::new(format!("v{}", env!("CARGO_PKG_VERSION")))
                                .small()
                                .color(ui.visuals().weak_text_color()),
                        );
                    });
                });
            });
            egui::CentralPanel::default().show(ctx, |ui| match self.screen {
                Screen::Library => self.ui_library(ui),
                Screen::Catalog => {
                    if let Some((id, profile)) = self.catalog_view.ui(ui, &self.library) {
                        self.open_controls(&id);
                        if let Some((_, controls, _)) = self.controls_draft.as_mut() {
                            controls.profile = profile;
                            controls.keys = profile.keys();
                            controls.keyboard = profile == Profile::Text;
                        }
                    }
                }
                Screen::Settings => self.ui_settings(ui),
                Screen::GameSettings => self.ui_game_settings(ui),
                Screen::Run => self.ui_run(ui),
            });
        }
        self.ui_game_menu(ctx);
        self.ui_controls_editor(ctx);
        // While a game is running we want to drain the live frame
        // channel as fast as the runner produces frames; the original
        // 250 ms cadence capped the launcher at 4 fps, which is most
        // of what users perceived as "lag" in JumpyBall on the
        // desktop. ~16 ms (60 fps) matches the runner's per-slice
        // hook and keeps idle-frame redraws cheap because egui only
        // actually re-uploads textures when something changed.
        let repaint_after = if matches!(self.screen, Screen::Run) || self.frame_rx.is_some() {
            Duration::from_millis(16)
        } else {
            Duration::from_millis(250)
        };
        ctx.request_repaint_after(repaint_after);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pocket_core::kernel::gapi;
    use pocket_library::KeyBindings;

    /// Drawing the on-screen pad must not cancel a key the user is
    /// holding on the keyboard. `ui_virtual_pad` runs every frame and releases
    /// its button as soon as the pointer is not on it, which — while
    /// both sources shared one set — sent a `WM_KEYUP` for a keyboard
    /// arrow that was still down, so JumpyBall's ball stopped steering
    /// on the desktop after the user had touched the pad once.
    #[test]
    fn the_on_screen_pad_does_not_release_a_key_held_on_the_keyboard() {
        let mut held = HeldButtons::default();
        let vk = GuestButton::DpadRight.vk();

        assert!(held.press(InputSource::Keyboard, vk));
        // The pad's per-frame "pointer is not on me" branch.
        assert!(
            !held.release(InputSource::Pointer, vk),
            "the pad never held this button, so it cannot release it"
        );
        assert!(held.is_held(vk), "the keyboard is still holding it");

        // Only letting go of the key itself reaches the guest.
        assert!(held.release(InputSource::Keyboard, vk));
        assert!(!held.is_held(vk));
    }

    /// The mirror case: while a button is held on the pad *and* on the
    /// keyboard, letting go of one of them must not tell the guest the
    /// button came up — otherwise a user resting a finger on the pad
    /// loses their keyboard hold and vice versa.
    #[test]
    fn a_button_stays_down_until_every_source_releases_it() {
        let mut held = HeldButtons::default();
        let vk = GuestButton::DpadLeft.vk();

        assert!(held.press(InputSource::Pointer, vk));
        assert!(
            !held.press(InputSource::Keyboard, vk),
            "the guest already saw WM_KEYDOWN for this button"
        );
        assert!(!held.release(InputSource::Pointer, vk));
        assert!(held.is_held(vk));
        assert!(held.release(InputSource::Keyboard, vk));
        assert!(!held.is_held(vk));
    }

    /// Closing the window has to release everything, from either source,
    /// or the guest is left with a stuck direction.
    #[test]
    fn draining_reports_every_held_button_once() {
        let mut held = HeldButtons::default();
        let up = GuestButton::DpadUp.vk();
        let action = GuestButton::Action.vk();
        held.press(InputSource::Keyboard, up);
        held.press(InputSource::Pointer, up);
        held.press(InputSource::Pointer, action);

        let mut drained = held.drain_all();
        drained.sort_unstable();
        let mut expected = vec![up, action];
        expected.sort_unstable();
        assert_eq!(drained, expected);
        assert!(!held.is_held(up));
        assert!(!held.is_held(action));
    }

    /// `pocket-library` cannot depend on the emulator (the Android
    /// launcher links it without one), so it repeats the GAPI virtual-key
    /// codes. A `gx.dll` title only reacts to the keys
    /// `GXGetDefaultKeys` named, so a drift between the two tables would
    /// silently stop the D-pad working.
    #[test]
    fn guest_button_vks_match_the_gapi_table() {
        assert_eq!(GuestButton::DpadUp.vk(), gapi::VK_UP);
        assert_eq!(GuestButton::DpadDown.vk(), gapi::VK_DOWN);
        assert_eq!(GuestButton::DpadLeft.vk(), gapi::VK_LEFT);
        assert_eq!(GuestButton::DpadRight.vk(), gapi::VK_RIGHT);
        assert_eq!(GuestButton::Action.vk(), gapi::VK_RETURN);
    }

    /// The keyboard used to be a hard-coded `match` on `egui::Key`.
    /// Users who never open the keybinding editor must keep exactly the
    /// keys they had, so the defaults reproduce that table key for key.
    #[test]
    fn default_bindings_reproduce_the_old_hard_coded_keyboard() {
        let bindings = KeyBindings::default();
        for (key, expected) in [
            (egui::Key::ArrowUp, GuestButton::DpadUp),
            (egui::Key::ArrowDown, GuestButton::DpadDown),
            (egui::Key::ArrowLeft, GuestButton::DpadLeft),
            (egui::Key::ArrowRight, GuestButton::DpadRight),
            (egui::Key::Enter, GuestButton::Action),
            (egui::Key::A, GuestButton::ButtonA),
            (egui::Key::B, GuestButton::ButtonB),
            (egui::Key::Space, GuestButton::ButtonB),
            (egui::Key::C, GuestButton::ButtonC),
            (egui::Key::Tab, GuestButton::Soft1),
            (egui::Key::Num1, GuestButton::Soft1),
            (egui::Key::Escape, GuestButton::Soft2),
            (egui::Key::Num2, GuestButton::Soft2),
            (egui::Key::S, GuestButton::Turbo),
            (egui::Key::F3, GuestButton::Turbo),
        ] {
            assert_eq!(
                bindings.vk_for_key(key.name()),
                Some(expected.vk()),
                "{} should still drive {:?}",
                key.name(),
                expected
            );
        }
        // A key that was never bound must stay unbound.
        assert_eq!(bindings.vk_for_key(egui::Key::F12.name()), None);
    }

    /// A quarter turn has to be undone on the way back in, or a tap
    /// lands somewhere the user did not touch.
    #[test]
    fn a_tap_is_unrotated_before_the_guest_sees_it() {
        let display = Vec2::new(320.0, 240.0);
        let game = Vec2::new(240.0, 320.0);
        // Top-left of a 90°-clockwise presentation is the bottom-left
        // corner of the guest's own portrait panel.
        let (x, y) =
            rotated_pointer_to_game(Vec2::new(0.0, 0.0), display, game, RotationPref::Cw90);
        assert_eq!((x, y), (0, 319));
        let (x, y) =
            rotated_pointer_to_game(Vec2::new(0.0, 0.0), display, game, RotationPref::None);
        assert_eq!((x, y), (0, 0));
    }

    #[test]
    fn game_only_fullscreen_centers_and_fits_the_rotated_game_frame() {
        let bounds = Rect::from_min_size(Pos2::new(100.0, 50.0), Vec2::new(1920.0, 1080.0));
        let portrait = fullscreen_frame_rect(bounds, Vec2::new(240.0, 320.0), RotationPref::None);
        let landscape = fullscreen_frame_rect(bounds, Vec2::new(240.0, 320.0), RotationPref::Cw90);

        assert_eq!(portrait.center(), bounds.center());
        assert_eq!(portrait.size(), Vec2::new(810.0, 1080.0));
        assert_eq!(landscape.center(), bounds.center());
        assert_eq!(landscape.size(), Vec2::new(1440.0, 1080.0));
    }
    #[test]
    fn semantic_activation_and_modifiers_cannot_release_another_source() {
        let mut held = HeldButtons::default();
        let vk = 17;
        assert!(held.press(InputSource::Modifiers, vk));
        assert!(!held.press(InputSource::Activation, vk));
        assert!(!held.press(InputSource::Keyboard, vk));
        held.gamepad.insert((0, "PadY"), vk);
        held.gamepad.insert((1, "PadY"), vk);
        assert!(!held.release(InputSource::Activation, vk));
        assert!(!held.release(InputSource::Modifiers, vk));
        assert!(!held.release(InputSource::Keyboard, vk));
        held.gamepad.remove(&(0, "PadY"));
        assert!(held.is_held(vk));
        assert_eq!(held.drain_all(), vec![vk]);
        assert!(!held.is_held(vk));
    }
}

// Pixel loops step through fixed-size chunks; as the other crates do.
#![allow(clippy::chunks_exact_to_as_chunks)]
//! Browser frontend: PocketHLE as a set of C functions a page calls.
//!
//! Modelled on FluffOS's WebAssembly driver, whose `fluffos_boot` / `tick` /
//! `input` exports let the page pump everything. A tab cannot block, so
//! `pocket_tick` runs until the guest sleeps and returns how long to wait. The
//! ARM code runs on the portable interpreter, since Unicorn generates host
//! machine code that WebAssembly cannot execute.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::path::{Path, PathBuf};

use pocket_core::kernel::{AudioTap, FrameAction, InputEvent, KernelState};
use pocket_core::Emulator;
use pocket_library::import_problem::Lang;
use pocket_library::{GameEntry, Library};

const LIBRARY_ROOT: &str = "/library";

struct Session {
    emu: Emulator,
    audio: Option<AudioTap>,
    /// RGBA pixels of the last presented frame.
    frame: Vec<u8>,
    width: u32,
    height: u32,
    frame_serial: u32,
    seen_counter: u64,
    /// `complete_frames` when a frame was last shown.
    seen_complete: u64,
    /// Ticks since a frame was last shown.
    ticks_since_shown: u32,
    input: VecDeque<InputEvent>,
    last_error: String,
}

thread_local! {
    static SESSION: RefCell<Option<Session>> = const { RefCell::new(None) };
    static LAST_ERROR: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static GAME_NAME: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    static WARNING: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Warnings go to the console, where the page's developer tools and the Node
/// harness see which guest calls are not implemented yet.
struct ConsoleLogger;

impl log::Log for ConsoleLogger {
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::Level::Warn
    }
    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            eprintln!("{}: {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}

static LOGGER: ConsoleLogger = ConsoleLogger;

fn init_logging() {
    if log::set_logger(&LOGGER).is_ok() {
        log::set_max_level(log::LevelFilter::Warn);
    }
}

fn set_error(message: String) {
    eprintln!("pockethle: {message}");
    LAST_ERROR.with(|slot| {
        let mut bytes = message.into_bytes();
        bytes.push(0);
        *slot.borrow_mut() = bytes;
    });
}

thread_local! {
    /// The page's language: explanations come in Chinese for the Chinese
    /// pages and in English for every other language.
    static ENGLISH: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

fn lang() -> Lang {
    if ENGLISH.with(|e| e.get()) {
        Lang::En
    } else {
        Lang::ZhHant
    }
}

/// Choose the language of explanations: `0` Chinese, anything else English.
#[no_mangle]
pub extern "C" fn pocket_set_lang(english: i32) {
    ENGLISH.with(|e| e.set(english != 0));
}

fn knowledge() -> pocket_library::import_kb::KnowledgeBase {
    pocket_library::import_kb::KnowledgeBase::load(Path::new(LIBRARY_ROOT))
}

/// A failure explained for a player, with the technical message kept.
fn explain(message: &str, missing: &[String]) -> String {
    knowledge().explain(message, missing).message(lang())
}

/// After a successful import, how the game was found when the download was not
/// a package (`download -> file taken`); empty otherwise.
#[no_mangle]
pub extern "C" fn pocket_import_note() -> *const u8 {
    pocket_boot_warning()
}

/// A warning about the game just booted, such as a DLL nothing provides; empty
/// when there is none. The game still starts, since some never call it.
#[no_mangle]
pub extern "C" fn pocket_boot_warning() -> *const u8 {
    WARNING.with(|slot| {
        if slot.borrow().is_empty() {
            *slot.borrow_mut() = vec![0];
        }
        slot.borrow().as_ptr()
    })
}

/// Text of the last failure, for the page to show.
#[no_mangle]
pub extern "C" fn pocket_last_error() -> *const u8 {
    LAST_ERROR.with(|slot| slot.borrow().as_ptr())
}

fn c_path(path: *const c_char) -> Option<PathBuf> {
    if path.is_null() {
        return None;
    }
    // SAFETY: the page passes a NUL-terminated string that outlives this call.
    let text = unsafe { CStr::from_ptr(path) }.to_str().ok()?;
    Some(PathBuf::from(text))
}

/// Import a game package from the virtual filesystem. Returns 0 on success.
#[no_mangle]
pub extern "C" fn pocket_import(path: *const c_char) -> i32 {
    init_logging();
    let Some(path) = c_path(path) else {
        return -1;
    };
    let mut library = match Library::open(LIBRARY_ROOT) {
        Ok(library) => library,
        Err(error) => {
            set_error(explain(&error.to_string(), &[]));
            return -2;
        }
    };
    match library.import_any(&path) {
        Ok(_) => {
            let note = library.last_import_note().unwrap_or_default().to_string();
            WARNING.with(|slot| {
                let mut bytes = note.into_bytes();
                bytes.push(0);
                *slot.borrow_mut() = bytes;
            });
            0
        }
        Err(error) => {
            let problem = knowledge().explain_import(&error);
            set_error(problem.message(lang()));
            -2
        }
    }
}

/// How many games the library holds.
/// Delete game number `index`: its files and its entry in the library index.
/// Removing only the files left the name behind with no icon after a reload.
/// Returns 0 on success, -1 on failure.
#[no_mangle]
pub extern "C" fn pocket_delete_game(index: i32) -> i32 {
    let Ok(mut library) = Library::open(LIBRARY_ROOT) else {
        return -1;
    };
    let Some(id) = library.games().get(index as usize).map(|g| g.id.clone()) else {
        return -1;
    };
    match library.remove(&id) {
        Ok(()) => 0,
        Err(error) => {
            set_error(error.to_string());
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn pocket_game_count() -> i32 {
    Library::open(LIBRARY_ROOT).map_or(0, |library| library.games().len() as i32)
}

/// Directory name of game number `index`, which is where its files and saves
/// live under `/library/games`. Valid until the next call.
#[no_mangle]
pub extern "C" fn pocket_game_id(index: i32) -> *const u8 {
    let id = Library::open(LIBRARY_ROOT)
        .ok()
        .and_then(|library| Some(library.games().get(index as usize)?.id.clone()))
        .unwrap_or_default();
    GAME_NAME.with(|slot| {
        let mut bytes = id.into_bytes();
        bytes.push(0);
        *slot.borrow_mut() = bytes;
        slot.borrow().as_ptr()
    })
}

fn game_dir(index: i32) -> Option<PathBuf> {
    let library = Library::open(LIBRARY_ROOT).ok()?;
    let id = library.games().get(index as usize)?.id.clone();
    Some(Path::new(LIBRARY_ROOT).join("games").join(id))
}

/// Write game number `index`'s progress as a save zip at `path`. Returns how many
/// files it holds, or -1 on failure. The format is shared with the desktop and
/// Android versions.
#[no_mangle]
pub extern "C" fn pocket_save_export(index: i32, path: *const c_char) -> i32 {
    let (Some(dir), Some(path)) = (game_dir(index), c_path(path)) else {
        return -1;
    };
    let result = std::fs::File::create(&path)
        .and_then(|file| pocket_library::save_archive::export(&dir, file));
    report_count(result, "cannot export the save")
}

/// Replace game number `index`'s progress with the save zip at `path`.
#[no_mangle]
pub extern "C" fn pocket_save_import(index: i32, path: *const c_char) -> i32 {
    let (Some(dir), Some(path)) = (game_dir(index), c_path(path)) else {
        return -1;
    };
    let result = std::fs::File::open(&path)
        .and_then(|file| pocket_library::save_archive::import(&dir, file));
    report_count(result, "cannot import the save")
}

/// Delete game number `index`'s progress.
#[no_mangle]
pub extern "C" fn pocket_save_reset(index: i32) -> i32 {
    let Some(dir) = game_dir(index) else {
        return -1;
    };
    report_count(
        pocket_library::save_archive::reset(&dir),
        "cannot reset the save",
    )
}

fn report_count(result: std::io::Result<usize>, what: &str) -> i32 {
    match result {
        Ok(count) => count as i32,
        Err(error) => {
            set_error(format!("{what}: {error}"));
            -1
        }
    }
}

/// Name of game number `index` for the page's list, valid until the next call.
#[no_mangle]
pub extern "C" fn pocket_game_name(index: i32) -> *const u8 {
    let name = Library::open(LIBRARY_ROOT)
        .ok()
        .and_then(|library| Some(library.games().get(index as usize)?.display_name.clone()))
        .unwrap_or_default();
    GAME_NAME.with(|slot| {
        let mut bytes = name.into_bytes();
        bytes.push(0);
        *slot.borrow_mut() = bytes;
        slot.borrow().as_ptr()
    })
}

fn prepare(entry: &GameEntry, root: &Path) -> anyhow::Result<Emulator> {
    // ARM, or MIPS for the Pocket PC 2000 builds made for the Casio Cassiopeia.
    let arch = Emulator::arch_of_executable(entry.launch_path(root))
        .unwrap_or(pocket_core::cpu::Arch::Arm);
    let mut emu = Emulator::with_interp_cpu_for_arch(arch);
    emu.max_slices = 0;
    emu.instruction_budget_per_slice = entry.settings.instructions_per_slice;
    emu.load_pe(entry.launch_path(root))?;
    for value in &entry.registry {
        if let Some(text) = value.string.as_deref() {
            emu.set_registry_value(
                &value.key,
                &value.name,
                pocket_core::kernel::registry::RegistryValue::Sz(text.to_string()),
            );
        }
    }
    let (width, height) = entry.settings.screen.size();
    emu.set_screen_size(width, height);
    let extracted = entry.extracted_dir(root);
    emu.mount_read_only_dir("\\Application\\", &extracted);
    emu.mount_read_only_dir("\\Program Files\\", &extracted);
    for prefix in &entry.install_dirs {
        emu.mount_read_only_dir(prefix, &extracted);
    }
    if let Some(prefix) = entry.guest_install_prefix() {
        emu.mount_read_only_dir(&prefix, &extracted);
        if let Some(guest_exe) = entry.guest_exe_path() {
            emu.set_module_path(&guest_exe);
            emu.set_default_dir(&prefix);
        }
        if let Some(save_prefix) = entry.guest_save_prefix() {
            std::fs::create_dir_all(entry.save_dir(root))?;
            emu.mount_save_dir(&save_prefix, entry.save_dir(root));
        }
    }
    if entry.guest_install_prefix().is_none() {
        mount_loose_archive(&mut emu, entry, root, &extracted)?;
    }
    emu.set_synthetic_message_budget(0);
    if let Some(process) = emu.process_mut() {
        process.state.cooperative_host = true;
    }
    Ok(emu)
}

/// A plain ZIP says nothing about where it was installed, so its folders do: the
/// executable reports `\\Program Files\\<folders>\\`, and those folders also appear
/// at the root of the device with writes going to the save folder. Pocket Quake
/// insists on `\\quake\\id1`, where it also keeps its saved games.
fn mount_loose_archive(
    emu: &mut Emulator,
    entry: &GameEntry,
    root: &Path,
    extracted: &Path,
) -> anyhow::Result<()> {
    let launch = entry.launch_path(root);
    let folder = launch.parent().unwrap_or(extracted);
    let relative = folder.strip_prefix(extracted).unwrap_or(Path::new(""));
    let guest: Vec<String> = relative
        .components()
        .map(|part| part.as_os_str().to_string_lossy().into_owned())
        .collect();
    let joined = guest.join("\\");
    let installed = if joined.is_empty() {
        "\\Program Files\\Game\\".to_string()
    } else {
        format!("\\Program Files\\{joined}\\")
    };
    let saves = entry.save_dir(root);
    std::fs::create_dir_all(&saves)?;
    emu.mount_read_only_dir(&installed, folder);
    emu.mount_save_dir(&installed, &saves);
    if !joined.is_empty() {
        let at_root = format!("\\{joined}\\");
        emu.mount_read_only_dir(&at_root, folder);
        emu.mount_save_dir(&at_root, &saves);
    }
    if let Some(name) = launch.file_name() {
        emu.set_module_path(format!("{installed}{}", name.to_string_lossy()));
    }
    emu.set_default_dir(&installed);
    Ok(())
}

/// Prepare game number `index` to run. Returns 0 on success.
#[no_mangle]
pub extern "C" fn pocket_boot(index: i32) -> i32 {
    init_logging();
    let root = Path::new(LIBRARY_ROOT);
    let result = Library::open(root)
        .map_err(anyhow::Error::from)
        .and_then(|library| {
            let entry = library
                .games()
                .get(index as usize)
                .ok_or_else(|| anyhow::anyhow!("no game number {index}"))?;
            let emu = prepare(entry, root)?;
            Ok((emu, entry.settings.screen.size()))
        });
    match result {
        Ok((emu, (width, height))) => {
            let missing = emu.missing_libraries();
            WARNING.with(|slot| {
                let mut text = if missing.is_empty() {
                    String::new()
                } else {
                    knowledge().explain("", &missing).hint(lang())
                };
                text.push('\0');
                *slot.borrow_mut() = text.into_bytes();
            });
            let audio = emu.audio_tap();
            SESSION.with(|slot| {
                *slot.borrow_mut() = Some(Session {
                    emu,
                    audio,
                    frame: vec![0; width as usize * height as usize * 4],
                    width,
                    height,
                    frame_serial: 0,
                    seen_counter: 0,
                    seen_complete: 0,
                    ticks_since_shown: 0,
                    input: VecDeque::new(),
                    last_error: String::new(),
                });
            });
            0
        }
        Err(error) => {
            set_error(explain(&format!("{error:#}"), &[]));
            -1
        }
    }
}

/// Run until the guest sleeps. Returns milliseconds to wait before the next
/// call, or -1 once the game has ended or failed.
#[no_mangle]
pub extern "C" fn pocket_tick() -> i32 {
    SESSION.with(|slot| {
        let mut guard = slot.borrow_mut();
        let Some(session) = guard.as_mut() else {
            return -1;
        };
        let Session {
            emu,
            frame,
            frame_serial,
            seen_counter,
            seen_complete,
            ticks_since_shown,
            input,
            last_error,
            ..
        } = session;
        // Handed over before the tick, not in the present hook: a modal dialog
        // spins without presenting, and Pocket Quake opens with one that only a
        // tap can dismiss.
        if let Some(process) = emu.process_mut() {
            process.state.pending_input.extend(input.drain(..));
        }
        // The run loop presents many times per guest frame, so the conversion
        // waits until the tick ends.
        let mut hook = |_: &mut KernelState| -> FrameAction { FrameAction::Continue };
        if let Err(error) = emu.run_with_hook(&mut hook) {
            *last_error = format!("{error:#}");
            set_error(explain(last_error, &emu.missing_libraries()));
            return -1;
        }
        let Some(process) = emu.process_mut() else {
            return -1;
        };
        if process.resume_pc.is_none() {
            return -1;
        }
        let state = &process.state;
        // Show a frame only at a point where the guest's screen is whole:
        // when it waits (Sleep, an idle message loop, a dialog), when it
        // finishes a frame, or at the latest every few ticks. A tick that
        // simply ran out of time can end halfway through a redraw, and
        // showing that made EverQuest's character slots flash black.
        *ticks_since_shown += 1;
        let whole = state.host_yield_ms.is_some()
            || state.complete_frames != *seen_complete
            || *ticks_since_shown >= 6;
        if whole && state.framebuffer.frame_counter != *seen_counter {
            *ticks_since_shown = 0;
            *seen_complete = state.complete_frames;
            *seen_counter = state.framebuffer.frame_counter;
            *frame_serial = frame_serial.wrapping_add(1);
            for (rgba, pixel) in frame
                .chunks_exact_mut(4)
                .zip(state.framebuffer.pixels.chunks_exact(2))
            {
                let value = u16::from_le_bytes([pixel[0], pixel[1]]);
                let (r, g, b) = (value >> 11 & 31, value >> 5 & 63, value & 31);
                rgba[0] = (r << 3 | r >> 2) as u8;
                rgba[1] = (g << 2 | g >> 4) as u8;
                rgba[2] = (b << 3 | b >> 2) as u8;
                rgba[3] = 255;
            }
        }
        process.state.host_yield_ms.take().map_or(0, |ms| ms as i32)
    })
}

/// Address of the RGBA frame.
#[no_mangle]
pub extern "C" fn pocket_frame_ptr() -> *const u8 {
    SESSION.with(|slot| {
        slot.borrow()
            .as_ref()
            .map_or(std::ptr::null(), |s| s.frame.as_ptr())
    })
}

/// Changes whenever the frame does.
#[no_mangle]
pub extern "C" fn pocket_frame_serial() -> u32 {
    SESSION.with(|slot| slot.borrow().as_ref().map_or(0, |s| s.frame_serial))
}

#[no_mangle]
pub extern "C" fn pocket_width() -> u32 {
    SESSION.with(|slot| slot.borrow().as_ref().map_or(0, |s| s.width))
}

#[no_mangle]
pub extern "C" fn pocket_height() -> u32 {
    SESSION.with(|slot| slot.borrow().as_ref().map_or(0, |s| s.height))
}

/// Stylus input: `kind` 0 = move, 1 = down, 2 = up.
#[no_mangle]
pub extern "C" fn pocket_pointer(kind: i32, x: i32, y: i32) {
    let (x, y) = (x.clamp(0, 65535) as u16, y.clamp(0, 65535) as u16);
    let event = match kind {
        1 => InputEvent::PointerDown { x, y },
        2 => InputEvent::PointerUp { x, y },
        _ => InputEvent::PointerMove { x, y },
    };
    SESSION.with(|slot| {
        if let Some(session) = slot.borrow_mut().as_mut() {
            session.input.push_back(event);
        }
    });
}

/// Keyboard input with a Windows virtual-key code; `down` is 1 or 0.
#[no_mangle]
pub extern "C" fn pocket_key(down: i32, vk: i32) {
    let vk = vk as u16;
    let event = if down != 0 {
        InputEvent::KeyDown { vk }
    } else {
        InputEvent::KeyUp { vk }
    };
    SESSION.with(|slot| {
        if let Some(session) = slot.borrow_mut().as_mut() {
            session.input.push_back(event);
        }
    });
}

/// Fill `frames` stereo float frames at `rate` Hz from the mix.
// Called only by the page's audio worklet, which owns the buffer; the
// pointer is checked for null and the length comes from the same caller.
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn pocket_audio(output: *mut f32, frames: u32, rate: u32) {
    if output.is_null() {
        return;
    }
    // SAFETY: the page allocated `frames * 2` floats at `output`.
    let buffer = unsafe { std::slice::from_raw_parts_mut(output, frames as usize * 2) };
    SESSION.with(
        |slot| match slot.borrow().as_ref().and_then(|s| s.audio.as_ref()) {
            Some(tap) => tap.render_mix(buffer, rate, 2),
            None => buffer.fill(0.0),
        },
    );
}

/// Guest instructions executed so far, in millions.
#[no_mangle]
pub extern "C" fn pocket_million_instructions() -> f64 {
    pocket_core::cpu::interp::RETIRED.load(std::sync::atomic::Ordering::Relaxed) as f64 / 1e6
}

fn main() {}

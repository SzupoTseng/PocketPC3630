//! A Windows CE window test program.
//!
//! It creates windows in the ways MFC programs do and writes, to
//! `\window-test.txt`, every message each window receives, in order,
//! together with what `GetParent`, `GetWindow`, `GetFocus` and
//! `GetActiveWindow` answer between the steps. The same program runs on
//! Microsoft's Device Emulator and in PocketPC3630; comparing the two
//! logs shows where our window semantics differ from a device.
//!
//! Built without the standard library for ARMv5 (what Pocket PC devices
//! and Device Emulator run): see `build.bat`. Imports are called through
//! one table the PE converter turns into the import address table.

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut, read_volatile};

type Hwnd = usize;
type WndProc = extern "C" fn(Hwnd, u32, usize, isize) -> isize;

// ---- Imports ----------------------------------------------------------
//
// One slot per coredll export, in the order of `IMPORTS` in `elf2pe.py`,
// then a zero that ends the table.
// The loader writes each function's address into its slot.

#[no_mangle]
#[link_section = ".iat"]
static mut IAT: [usize; 26] = [0; 26];

macro_rules! import {
    ($slot:expr, fn($($arg:ty),*) -> $ret:ty) => {{
        #[allow(unused_unsafe)]
        let f: usize = unsafe { read_volatile(addr_of!(IAT[$slot])) };
        #[allow(unused_unsafe)]
        let f = unsafe { core::mem::transmute::<usize, extern "C" fn($($arg),*) -> $ret>(f) };
        f
    }};
}

const REGISTER_CLASS_W: usize = 0;
const CREATE_WINDOW_EX_W: usize = 1;
const DESTROY_WINDOW: usize = 2;
const DEF_WINDOW_PROC_W: usize = 3;
const GET_PARENT: usize = 4;
const GET_WINDOW: usize = 5;
const GET_ACTIVE_WINDOW: usize = 6;
const GET_FOCUS: usize = 7;
const SET_FOCUS: usize = 8;
const GET_FOREGROUND_WINDOW: usize = 9;
const IS_WINDOW: usize = 10;
const POST_THREAD_MESSAGE_W: usize = 11;
const PEEK_MESSAGE_W: usize = 12;
const GET_WINDOW_THREAD_PROCESS_ID: usize = 13;
const CREATE_FILE_W: usize = 14;
const WRITE_FILE: usize = 15;
const CLOSE_HANDLE: usize = 16;
const SHOW_WINDOW: usize = 17;
const SET_ACTIVE_WINDOW: usize = 18;
const POST_QUIT_MESSAGE: usize = 19;
const DISPATCH_MESSAGE_W: usize = 20;
const POST_MESSAGE_W: usize = 21;
const SEND_MESSAGE_W: usize = 22;
const SET_FOREGROUND_WINDOW: usize = 23;
const ENABLE_WINDOW: usize = 24;

// ---- Constants -----------------------------------------------------------

const WS_POPUP: u32 = 0x8000_0000;
const WS_CHILD: u32 = 0x4000_0000;
const WS_VISIBLE: u32 = 0x1000_0000;
const WS_CAPTION: u32 = 0x00C0_0000;
const GW_OWNER: u32 = 4;
const GW_CHILD: u32 = 5;
const PM_REMOVE: u32 = 1;
const WM_USER: u32 = 0x400;
const GENERIC_WRITE: u32 = 0x4000_0000;
const CREATE_ALWAYS: u32 = 2;

#[repr(C)]
struct WndClass {
    style: u32,
    proc_: WndProc,
    cls_extra: i32,
    wnd_extra: i32,
    instance: usize,
    icon: usize,
    cursor: usize,
    background: usize,
    menu_name: *const u16,
    class_name: *const u16,
}

#[repr(C)]
struct Msg {
    hwnd: Hwnd,
    message: u32,
    wparam: usize,
    lparam: isize,
    time: u32,
    x: i32,
    y: i32,
}

// ---- The log -------------------------------------------------------------

static mut FILE: usize = 0;
static mut LINE: [u8; 160] = [0; 160];
static mut LEN: usize = 0;
/// Windows known by name, so the logs compare across machines whose
/// handle values differ.
static mut NAMES: [(Hwnd, u8); 8] = [(0, 0); 8];

fn put(bytes: &[u8]) {
    unsafe {
        let line = &mut *addr_of_mut!(LINE);
        let mut i = 0;
        while i < bytes.len() && LEN < line.len() {
            line[LEN] = bytes[i];
            LEN += 1;
            i += 1;
        }
    }
}

fn put_hex(value: usize) {
    let digits = b"0123456789abcdef";
    let mut shift = 28i32;
    let mut started = false;
    while shift >= 0 {
        let d = (value >> shift) & 15;
        if d != 0 || started || shift == 0 {
            started = true;
            put(&[digits[d]]);
        }
        shift -= 4;
    }
}

/// A window by its name, `0` for none, or `?` for one not created here.
fn put_window(hwnd: Hwnd) {
    if hwnd == 0 {
        put(b"0");
        return;
    }
    let names = unsafe { &*addr_of!(NAMES) };
    let mut i = 0;
    while i < names.len() {
        if names[i].0 == hwnd {
            put(&[names[i].1]);
            return;
        }
        i += 1;
    }
    put(b"?");
}

/// A message parameter: a known window by name, a small number in hex,
/// anything else (a pointer) as `*`.
fn put_param(value: usize) {
    let names = unsafe { &*addr_of!(NAMES) };
    let mut i = 0;
    while i < names.len() {
        if value != 0 && names[i].0 == value {
            put(&[names[i].1]);
            return;
        }
        i += 1;
    }
    if value <= 0xffff {
        put_hex(value);
    } else {
        put(b"*");
    }
}

fn end_line() {
    put(b"\r\n");
    unsafe {
        let mut written = 0u32;
        let write = import!(WRITE_FILE, fn(usize, *const u8, u32, *mut u32, usize) -> i32);
        write(FILE, addr_of!(LINE) as *const u8, LEN as u32, &mut written, 0);
        LEN = 0;
    }
}

fn say(text: &[u8]) {
    put(text);
    end_line();
}

fn name(hwnd: Hwnd, letter: u8) {
    let names = unsafe { &mut *addr_of_mut!(NAMES) };
    let mut i = 0;
    while i < names.len() {
        if names[i].0 == 0 {
            names[i] = (hwnd, letter);
            return;
        }
        i += 1;
    }
}

// ---- Window procedures ---------------------------------------------------

/// The window being created gets its name as soon as its first message
/// arrives, before `CreateWindowExW` returns its handle.
static mut CREATING: u8 = 0;

extern "C" fn window_proc(hwnd: Hwnd, msg: u32, wparam: usize, lparam: isize) -> isize {
    unsafe {
        if CREATING != 0 {
            let known = {
                let names = &*addr_of!(NAMES);
                let mut found = false;
                let mut i = 0;
                while i < names.len() {
                    if names[i].0 == hwnd {
                        found = true;
                    }
                    i += 1;
                }
                found
            };
            if !known {
                name(hwnd, CREATING);
            }
        }
    }
    put(b"  msg ");
    put_window(hwnd);
    put(b" ");
    put_hex(msg as usize);
    put(b" w=");
    put_param(wparam);
    put(b" l=");
    put_param(lparam as usize);
    end_line();
    let def = import!(DEF_WINDOW_PROC_W, fn(Hwnd, u32, usize, isize) -> isize);
    def(hwnd, msg, wparam, lparam)
}

// ---- Steps ----------------------------------------------------------------

fn wide(text: &[u8], out: &mut [u16]) -> *const u16 {
    let mut i = 0;
    while i < text.len() && i + 1 < out.len() {
        out[i] = text[i] as u16;
        i += 1;
    }
    out[i] = 0;
    out.as_ptr()
}

static mut CLASS: [u16; 16] = [0; 16];
static mut TITLE: [u16; 16] = [0; 16];

fn create(letter: u8, ex_style: u32, style: u32, parent: Hwnd, instance: usize) -> Hwnd {
    put(b"create ");
    put(&[letter]);
    end_line();
    let create = import!(
        CREATE_WINDOW_EX_W,
        fn(u32, *const u16, *const u16, u32, i32, i32, i32, i32, Hwnd, usize, usize, usize) -> Hwnd
    );
    unsafe {
        CREATING = letter;
        let title = wide(&[letter], &mut *addr_of_mut!(TITLE));
        let hwnd = create(
            ex_style,
            addr_of!(CLASS) as *const u16,
            title,
            style,
            0,
            0,
            200,
            200,
            parent,
            0,
            instance,
            0,
        );
        CREATING = 0;
        let names = &*addr_of!(NAMES);
        let mut known = false;
        let mut i = 0;
        while i < names.len() {
            if names[i].0 == hwnd {
                known = true;
            }
            i += 1;
        }
        if !known && hwnd != 0 {
            name(hwnd, letter);
        }
        put(b"created ");
        put(&[letter]);
        put(b" -> ");
        put_window(hwnd);
        end_line();
        hwnd
    }
}

fn state(label: &[u8], windows: &[Hwnd]) {
    let get_parent = import!(GET_PARENT, fn(Hwnd) -> Hwnd);
    let get_window = import!(GET_WINDOW, fn(Hwnd, u32) -> Hwnd);
    let is_window = import!(IS_WINDOW, fn(Hwnd) -> i32);
    let get_focus = import!(GET_FOCUS, fn() -> Hwnd);
    let get_active = import!(GET_ACTIVE_WINDOW, fn() -> Hwnd);
    let get_foreground = import!(GET_FOREGROUND_WINDOW, fn() -> Hwnd);
    put(b"state ");
    put(label);
    put(b": focus=");
    put_window(get_focus());
    put(b" active=");
    put_window(get_active());
    put(b" foreground=");
    put_window(get_foreground());
    end_line();
    let mut i = 0;
    while i < windows.len() {
        let hwnd = windows[i];
        put(b"  window ");
        put_window(hwnd);
        put(b": alive=");
        put_hex(is_window(hwnd) as usize);
        put(b" parent=");
        put_window(get_parent(hwnd));
        put(b" owner=");
        put_window(get_window(hwnd, GW_OWNER));
        put(b" child=");
        put_window(get_window(hwnd, GW_CHILD));
        end_line();
        i += 1;
    }
}

/// Deliver whatever is queued, logging thread messages (no window).
fn pump(label: &[u8]) {
    put(b"pump ");
    put(label);
    end_line();
    let peek = import!(PEEK_MESSAGE_W, fn(*mut Msg, Hwnd, u32, u32, u32) -> i32);
    let dispatch = import!(DISPATCH_MESSAGE_W, fn(*const Msg) -> isize);
    let mut msg = Msg {
        hwnd: 0,
        message: 0,
        wparam: 0,
        lparam: 0,
        time: 0,
        x: 0,
        y: 0,
    };
    let mut rounds = 0;
    while rounds < 200 && peek(&mut msg, 0, 0, 0, PM_REMOVE) != 0 {
        if msg.hwnd == 0 {
            put(b"  thread msg ");
            put_hex(msg.message as usize);
            put(b" w=");
            put_param(msg.wparam);
            end_line();
        } else {
            dispatch(&msg);
        }
        rounds += 1;
    }
}

#[no_mangle]
pub extern "C" fn entry(instance: usize, _previous: usize, _command: usize, _show: i32) -> i32 {
    let create_file = import!(CREATE_FILE_W, fn(*const u16, u32, u32, usize, u32, u32, usize) -> usize);
    // On a device or Device Emulator the log goes to the storage card (the
    // emulator's shared folder) so the host can read it; where there is no
    // card, to the root.
    let mut path = [0u16; 48];
    unsafe {
        FILE = create_file(
            wide(b"\\Storage Card\\window-test.txt", &mut path),
            GENERIC_WRITE,
            0,
            0,
            CREATE_ALWAYS,
            0x80,
            0,
        );
        if FILE == usize::MAX {
            FILE = create_file(wide(b"\\window-test.txt", &mut path), GENERIC_WRITE, 0, 0, CREATE_ALWAYS, 0x80, 0);
        }
    }
    say(b"window-test 1");

    let register = import!(REGISTER_CLASS_W, fn(*const WndClass) -> u16);
    let class = WndClass {
        style: 0,
        proc_: window_proc,
        cls_extra: 0,
        wnd_extra: 0,
        instance,
        icon: 0,
        cursor: 0,
        background: 0,
        menu_name: core::ptr::null(),
        class_name: unsafe { wide(b"WinTest", &mut *addr_of_mut!(CLASS)) },
    };
    let atom = register(&class);
    put(b"register ");
    put_hex((atom != 0) as usize);
    end_line();

    let show = import!(SHOW_WINDOW, fn(Hwnd, i32) -> i32);
    let set_focus = import!(SET_FOCUS, fn(Hwnd) -> Hwnd);
    let set_active = import!(SET_ACTIVE_WINDOW, fn(Hwnd) -> Hwnd);
    let set_foreground = import!(SET_FOREGROUND_WINDOW, fn(Hwnd) -> i32);
    let destroy = import!(DESTROY_WINDOW, fn(Hwnd) -> i32);
    let post_thread = import!(POST_THREAD_MESSAGE_W, fn(u32, u32, usize, isize) -> i32);
    let post = import!(POST_MESSAGE_W, fn(Hwnd, u32, usize, isize) -> i32);
    let send = import!(SEND_MESSAGE_W, fn(Hwnd, u32, usize, isize) -> isize);
    let thread_of = import!(GET_WINDOW_THREAD_PROCESS_ID, fn(Hwnd, *mut u32) -> u32);
    let enable = import!(ENABLE_WINDOW, fn(Hwnd, i32) -> i32);

    // 1. A frame (A), a child of it (B), a window it owns (C).
    let a = create(b'A', 0, WS_CAPTION, 0, instance);
    let b = create(b'B', 0, WS_CHILD | WS_VISIBLE, a, instance);
    let c = create(b'C', 0, WS_POPUP | WS_CAPTION, a, instance);
    state(b"after create", &[a, b, c]);
    pump(b"after create");

    // 2. Showing, activating and focusing.
    say(b"show A");
    show(a, 5);
    pump(b"after show A");
    state(b"after show A", &[a, b, c]);
    say(b"show C");
    show(c, 5);
    pump(b"after show C");
    state(b"after show C", &[a, b, c]);
    say(b"set active A");
    set_active(a);
    pump(b"after set active A");
    state(b"after set active A", &[a, b, c]);
    say(b"set foreground C");
    set_foreground(c);
    pump(b"after set foreground C");
    state(b"after set foreground C", &[a, b, c]);
    say(b"set focus B");
    set_focus(b);
    pump(b"after set focus B");
    state(b"after set focus B", &[a, b, c]);

    // 3. A second frame (D) and a disabled owner.
    let d = create(b'D', 0, WS_CAPTION | WS_VISIBLE, 0, instance);
    pump(b"after create D");
    state(b"after create D", &[a, b, c, d]);
    say(b"disable A");
    enable(a, 0);
    pump(b"after disable A");
    state(b"after disable A", &[a, b, c, d]);
    enable(a, 1);

    // 4. Messages: sent, posted to a window, posted to the thread.
    say(b"send A user+1");
    send(a, WM_USER + 1, 1, 2);
    say(b"post A user+2 and thread user+3");
    post(a, WM_USER + 2, 3, 4);
    let thread = thread_of(a, core::ptr::null_mut());
    post_thread(thread, WM_USER + 3, 5, 6);
    pump(b"after posts");

    // 5. Destroying the frame takes its child and owned window with it.
    say(b"destroy A");
    destroy(a);
    pump(b"after destroy A");
    state(b"after destroy A", &[a, b, c, d]);
    say(b"destroy D");
    destroy(d);
    pump(b"after destroy D");
    state(b"after destroy D", &[a, b, c, d]);

    say(b"end");
    let close = import!(CLOSE_HANDLE, fn(usize) -> i32);
    unsafe {
        close(FILE);
    }
    let quit = import!(POST_QUIT_MESSAGE, fn(i32) -> usize);
    quit(0);
    0
}

// ---- What the compiler expects of a freestanding program ------------------

#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub unsafe extern "C" fn memset(dst: *mut u8, value: i32, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        core::ptr::write_volatile(dst.add(i), value as u8);
        i += 1;
    }
    dst
}

#[no_mangle]
pub unsafe extern "C" fn memcpy(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    let mut i = 0;
    while i < n {
        core::ptr::write_volatile(dst.add(i), core::ptr::read_volatile(src.add(i)));
        i += 1;
    }
    dst
}

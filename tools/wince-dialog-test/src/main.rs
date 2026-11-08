//! Compare dialog geometry, initial focus and colors on a real WinCE user layer.
//! No game files, registration codes, standard library or CE compiler required.
#![no_std]
#![no_main]

use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut, read_volatile};

// The order is the contract with imports.json, plus a terminating zero.
#[no_mangle]
#[link_section = ".iat"]
static mut IAT: [usize; 19] = [0; 19];
macro_rules! api {
    ($slot:expr, fn($($arg:ty),*) -> $ret:ty) => {{
        let pointer = unsafe { read_volatile(addr_of!(IAT[$slot])) };
        unsafe { core::mem::transmute::<usize, extern "C" fn($($arg),*) -> $ret>(pointer) }
    }};
}

static mut FILE: usize = 0;
static mut LINE: [u8; 256] = [0; 256];
static mut LEN: usize = 0;

fn put(text: &[u8]) {
    unsafe {
        for &byte in text {
            if LEN < 256 {
                (addr_of_mut!(LINE) as *mut u8).add(LEN).write(byte);
                LEN += 1;
            }
        }
    }
}
fn hex(value: usize) {
    let digits = b"0123456789abcdef";
    let mut shift = 28;
    let mut started = false;
    loop {
        let digit = (value >> shift) & 15;
        if started || digit != 0 || shift == 0 {
            put(&[digits[digit]]);
            started = true;
        }
        if shift == 0 {
            break;
        }
        shift -= 4;
    }
}
fn end() {
    put(b"\r\n");
    let mut written = 0;
    let write = api!(8, fn(usize, *const u8, u32, *mut u32, usize) -> i32);
    unsafe {
        write(
            FILE,
            addr_of!(LINE) as *const u8,
            LEN as u32,
            &mut written,
            0,
        );
        LEN = 0;
    }
}
fn say(text: &[u8]) {
    put(text);
    end();
}

#[repr(C)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}
fn rect(hwnd: usize, client: bool) {
    let mut r = Rect {
        left: 0,
        top: 0,
        right: 0,
        bottom: 0,
    };
    let result = if client {
        api!(3, fn(usize, *mut Rect) -> i32)(hwnd, &mut r)
    } else {
        api!(2, fn(usize, *mut Rect) -> i32)(hwnd, &mut r)
    };
    hex(result as usize);
    put(b":");
    for n in [r.left, r.top, r.right, r.bottom] {
        hex(n as usize);
        put(b",");
    }
}
fn snapshot(label: &[u8], hwnd: usize) {
    put(label);
    put(b" window=");
    rect(hwnd, false);
    put(b" client=");
    rect(hwnd, true);
    put(b" style=");
    hex(api!(4, fn(usize, i32) -> u32)(hwnd, -16) as usize);
    let edit = api!(6, fn(usize, i32) -> usize)(hwnd, 1000);
    put(b" edit=");
    rect(edit, false);
    put(b" focus_is_edit=");
    hex((api!(5, fn() -> usize)() == edit && edit != 0) as usize);
    end();
    let dc = api!(14, fn(usize) -> usize)(edit);
    let mut metrics = [0u32; 16];
    let ok = api!(15, fn(usize, *mut u32) -> i32)(dc, metrics.as_mut_ptr());
    put(b"metrics ok=");
    hex(ok as usize);
    put(b" height=");
    hex(metrics[0] as usize);
    put(b" ascent=");
    hex(metrics[1] as usize);
    put(b" descent=");
    hex(metrics[2] as usize);
    put(b" average_width=");
    hex(metrics[5] as usize);
    end();
    api!(16, fn(usize, usize) -> i32)(edit, dc);
}

extern "C" fn dialog_proc(hwnd: usize, message: u32, wparam: usize, _: isize) -> isize {
    if message == 0x110 {
        let edit = api!(6, fn(usize, i32) -> usize)(hwnd, 1000);
        put(b"init suggested_is_edit=");
        hex((edit != 0 && wparam == edit) as usize);
        end();
        snapshot(b"init", hwnd);
        return 1; // Ask the dialog manager to assign initial focus.
    }
    0
}

#[repr(C, align(4))]
struct Template([u16; 256]);
struct Builder {
    template: Template,
    pos: usize,
}
impl Builder {
    fn new() -> Self {
        Self {
            template: Template([0; 256]),
            pos: 0,
        }
    }
    fn word(&mut self, value: u16) {
        if self.pos < 256 {
            unsafe {
                self.template.0.as_mut_ptr().add(self.pos).write(value);
            }
            self.pos += 1;
        }
    }
    fn dword(&mut self, value: u32) {
        self.word(value as u16);
        self.word((value >> 16) as u16);
    }
    fn text(&mut self, text: &[u8]) {
        for &ch in text {
            self.word(ch as u16);
        }
        self.word(0);
    }
}

#[repr(C)]
struct Msg {
    hwnd: usize,
    message: u32,
    wparam: usize,
    lparam: isize,
    time: u32,
    x: i32,
    y: i32,
}
fn pump() {
    let peek = api!(11, fn(*mut Msg, usize, u32, u32, u32) -> i32);
    let dispatch = api!(12, fn(*const Msg) -> isize);
    let sleep = api!(17, fn(u32) -> ());
    let mut msg = Msg {
        hwnd: 0,
        message: 0,
        wparam: 0,
        lparam: 0,
        time: 0,
        x: 0,
        y: 0,
    };
    for _ in 0..20 {
        let mut rounds = 0;
        while rounds < 100 && peek(&mut msg, 0, 0, 0, 1) != 0 {
            dispatch(&msg);
            rounds += 1;
        }
        sleep(10);
    }
}

fn create(instance: usize, parent: usize, style: u32, face: &[u8], label: &[u8]) -> usize {
    say(label);
    let mut b = Builder::new();
    b.dword(style);
    b.dword(0);
    b.word(1); // style, exstyle, one item
    for value in [0, 0, 135, 92] {
        b.word(value);
    }
    b.word(0);
    b.word(0);
    b.text(b"Dialog probe"); // menu, class, caption
    b.word(8);
    b.text(face); // DS_SETFONT
    if b.pos & 1 != 0 {
        b.word(0);
    }
    b.dword(0x5081_0080);
    b.dword(0); // child, visible, tabstop, border, autohscroll
    for value in [7, 37, 121, 14, 1000] {
        b.word(value);
    }
    b.word(0xffff);
    b.word(0x81);
    b.word(0);
    b.word(0); // EDIT class, text, extra
    let hwnd = api!(0, fn(usize, *const u16, usize, usize, isize) -> usize)(
        instance,
        b.template.0.as_ptr(),
        parent,
        dialog_proc as *const () as usize,
        0,
    );
    put(b"created=");
    hex((hwnd != 0) as usize);
    end();
    snapshot(b"after create", hwnd);
    api!(10, fn(usize, i32) -> i32)(hwnd, 5);
    snapshot(b"after show", hwnd);
    api!(10, fn(usize, i32) -> i32)(hwnd, 1);
    pump();
    snapshot(b"after normal pump", hwnd);
    hwnd
}

fn wide(text: &[u8], out: &mut [u16; 48]) -> *const u16 {
    let mut i = 0;
    while i < text.len() && i < 47 {
        out[i] = text[i] as u16;
        i += 1;
    }
    out[i] = 0;
    out.as_ptr()
}

#[no_mangle]
pub extern "C" fn entry(instance: usize, _: usize, _: usize, _: i32) -> i32 {
    let mut path = [0u16; 48];
    let open = api!(7, fn(*const u16, u32, u32, usize, u32, u32, usize) -> usize);
    unsafe {
        FILE = open(
            wide(b"\\Storage Card\\dialog-test.txt", &mut path),
            0x4000_0000,
            0,
            0,
            2,
            0x80,
            0,
        );
        if FILE == usize::MAX {
            FILE = open(
                wide(b"\\dialog-test.txt", &mut path),
                0x4000_0000,
                0,
                0,
                2,
                0x80,
                0,
            );
        }
    }
    say(b"dialog-test 2 (numbers hexadecimal)");
    for index in [5, 15, 18, 0x4000_0005, 0x4000_000f, 0x4000_0012] {
        put(b"color ");
        hex(index);
        put(b"=");
        hex(api!(13, fn(i32) -> u32)(index as i32) as usize);
        end();
    }
    let destroy = api!(1, fn(usize) -> i32);
    let root = create(
        instance,
        0,
        0x80c0_0040,
        b"System",
        b"case system captioned popup",
    );
    let child = create(instance, root, 0x5000_0040, b"System", b"case system child");
    destroy(child);
    destroy(root);
    let modal_frame = create(
        instance,
        0,
        0x80c0_00c0,
        b"System",
        b"case system modal-frame popup",
    );
    destroy(modal_frame);
    let bare = create(
        instance,
        0,
        0x8000_0040,
        b"System",
        b"case system bare popup",
    );
    destroy(bare);
    let tahoma = create(
        instance,
        0,
        0x80c0_0040,
        b"Tahoma",
        b"case tahoma captioned popup",
    );
    destroy(tahoma);
    say(b"end");
    let close = api!(9, fn(usize) -> i32);
    unsafe {
        close(FILE);
    }
    0
}

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

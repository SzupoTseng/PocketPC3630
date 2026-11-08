//! Hardware-key and short-touch observations on CE and PocketHLE.
//! Contains no game code or registration data. The log stays beside the EXE.
#![no_std]
#![no_main]
use core::panic::PanicInfo;
use core::ptr::{addr_of, addr_of_mut, read_volatile};

#[no_mangle]
#[link_section = ".iat"]
static mut IAT: [usize; 28] = [0; 28];
macro_rules! api {
    ($slot:expr, fn($($arg:ty),*) -> $ret:ty) => {{
        #[allow(unused_unsafe)]
        let p = unsafe { read_volatile(addr_of!(IAT[$slot])) };
        #[allow(unused_unsafe)]
        let call = unsafe { core::mem::transmute::<usize, extern "C" fn($($arg),*) -> $ret>(p) };
        call
    }};
}
static mut FILE: usize = 0;
static mut LINE: [u8; 256] = [0; 256];
static mut LEN: usize = 0;
static mut MAIN: usize = 0;
static mut TARGET: usize = 0;
static mut LABEL: usize = 0;
static mut ASSOC: usize = 0;
static mut MODE: usize = 0;
static mut CLICKS: usize = 0;
static mut KEYS: usize = 0;
static mut SEQ: usize = 0;
fn put(text: &[u8]) {
    unsafe {
        for &b in text {
            if LEN < 254 { (addr_of_mut!(LINE) as *mut u8).add(LEN).write(b); LEN += 1; }
        }
    }
}
fn hex(n: usize) {
    let mut shift = 28; let mut started = false;
    loop { let d = (n >> shift) & 15;
        if started || d != 0 || shift == 0 { put(&[b"0123456789abcdef"[d]]); started = true; }
        if shift == 0 { break; } shift -= 4;
    }
}
fn end() {
    put(b"\r\n"); let mut written = 0;
    unsafe {
        if FILE != 0 && FILE != usize::MAX {
            api!(14, fn(usize,*const u8,u32,*mut u32,usize)->i32)(FILE,addr_of!(LINE) as *const u8,LEN as u32,&mut written,0);
            api!(16, fn(usize)->i32)(FILE);
        }
        LEN = 0;
    }
}
fn wide(text: &[u8], out: &mut [u16]) -> *const u16 {
    if out.is_empty() { return out.as_ptr(); }
    let mut i = 0;
    while i < text.len() && i + 1 < out.len() {
        unsafe { out.as_mut_ptr().add(i).write(text.as_ptr().add(i).read() as u16); }
        i += 1;
    }
    unsafe { out.as_mut_ptr().add(i).write(0); }
    out.as_ptr()
}
fn now() -> u32 { api!(17, fn()->u32)() }
fn field(label: &[u8], n: usize) { put(label); hex(n); }
fn status() {
    let mut text = [0u16; 96]; let mut bytes = [0u8; 96];
    // Only hexadecimal counters, matching the event log.
    let saved = unsafe { LEN }; unsafe { LEN = 0; }
    unsafe { field(b"mode=",MODE); field(b" taps=",CLICKS); field(b" keys=",KEYS); }
    let len = unsafe { LEN }.min(bytes.len());
    unsafe { core::ptr::copy_nonoverlapping(addr_of!(LINE) as *const u8,bytes.as_mut_ptr(),len); LEN=saved; }
    let p=wide(unsafe { core::slice::from_raw_parts(bytes.as_ptr(),len) },&mut text);
    unsafe { if LABEL != 0 { api!(22,fn(usize,*const u16)->i32)(LABEL,p); } }
}
#[repr(C)]
struct WndClass { style:u32, proc_:extern "C" fn(usize,u32,usize,isize)->isize,
    cls_extra:i32,wnd_extra:i32,instance:usize,icon:usize,cursor:usize,background:usize,
    menu_name:*const u16,class_name:*const u16 }
#[repr(C)]
struct Msg { hwnd:usize,msg:u32,wp:usize,lp:isize,time:u32,x:i32,y:i32 }
#[repr(C)]
struct Version { size:u32,major:u32,minor:u32,build:u32,platform:u32,description:[u16;128] }
#[repr(C)]
#[derive(Clone,Copy)]
struct Key { vk:i16,pad:i16,x:i32,y:i32 }
#[repr(C)]
struct GxKeys { keys:[Key;8] }
fn dynamic(module:&[u8], name:&[u8]) -> usize {
    let mut library=[0u16;64];let mut symbol=[0u16;128];
    let handle=api!(20,fn(*const u16)->usize)(wide(module,&mut library));
    if handle==0 { return 0; }
    api!(21,fn(usize,*const u16)->usize)(handle,wide(name,&mut symbol))
}
fn associate(target:usize,mode:usize) {
    unsafe { MODE=mode; }
    for key in 0xc1..=0xc6 {
        let result=unsafe {
            if ASSOC==0 { -1 } else {
                let call=core::mem::transmute::<usize,extern "C" fn(u32,usize)->i32>(ASSOC);
                call(key,target)
            }
        };
        field(b"associate mode=",mode);field(b" vk=",key as usize);field(b" result=",result as usize);
        field(b" error=",api!(26,fn()->u32)() as usize);end();
    }
    status();
}
fn query_gapi() {
    let mut p=dynamic(b"gx.dll",b"?GXGetDefaultKeys@@YA?AUGXKeyList@@H@Z");
    if p==0 { p=dynamic(b"gx.dll",b"GXGetDefaultKeys"); }
    field(b"gapi_available=",usize::from(p!=0));end();
    if p!=0 {
        let call=unsafe { core::mem::transmute::<usize,extern "C" fn(i32)->GxKeys>(p) };
        let keys=call(0);
        for (i,key) in keys.keys.iter().enumerate() {
            field(b"gapi entry=",i);field(b" vk=",key.vk as u16 as usize);
            field(b" x=",key.x as usize);field(b" y=",key.y as usize);end();
        }
    }
}
extern "C" fn proc_(hwnd:usize,msg:u32,wp:usize,lp:isize)->isize {
    if matches!(msg,0x7|0x8|0x100..=0x105|0x201..=0x203|0x312) {
        unsafe { SEQ+=1; if msg==0x100||msg==0x104 { KEYS+=1; } field(b"event seq=",SEQ); }
        field(b" tick=",now() as usize);field(b" mode=",unsafe {MODE});
        put(if hwnd==unsafe {TARGET} {b" target=B"} else {b" target=A"});
        field(b" msg=",msg as usize);field(b" w=",wp);field(b" l=",lp as usize);
        if (0x100..=0x105).contains(&msg) {
            field(b" sync=",api!(23,fn(u32)->i16)(wp as u32) as u16 as usize);
            field(b" async=",api!(24,fn(u32)->i16)(wp as u32) as u16 as usize);
        }
        field(b" focus=",api!(8,fn()->usize)());end();status();
    }
    if msg==0x111 {
        let id=wp&0xffff;field(b"command id=",id);end();
        unsafe {
            match id {
                101=>{CLICKS+=1;field(b"click count=",CLICKS);end();status();},
                102=>associate(MAIN,1),103=>associate(TARGET,2),104=>associate(0,0),
                106=>query_gapi(),107=>{api!(3,fn(usize)->i32)(MAIN);return 0;},_=>{}
            }
            api!(7,fn(usize)->usize)(MAIN);
        }
    }
    if msg==0x202 && hwnd==unsafe {TARGET} { api!(7,fn(usize)->usize)(hwnd); }
    if msg==2 && hwnd==unsafe {MAIN} { api!(12,fn(i32)->usize)(0); }
    api!(2,fn(usize,u32,usize,isize)->isize)(hwnd,msg,wp,lp)
}
fn create(instance:usize,class:*const u16,text:&[u8],style:u32,x:i32,y:i32,w:i32,h:i32,parent:usize,id:usize)->usize {
    let mut title=[0u16;96];
    api!(1,fn(u32,*const u16,*const u16,u32,i32,i32,i32,i32,usize,usize,usize,usize)->usize)
        (0,class,wide(text,&mut title),style,x,y,w,h,parent,id,instance,0)
}
#[no_mangle]
pub extern "C" fn entry(instance:usize,_previous:usize,_command:usize,_show:i32)->i32 {
    let mut path=[0u16;260];let length=api!(19,fn(usize,*mut u16,u32)->u32)(0,path.as_mut_ptr(),260) as usize;
    let mut last=0;for (i,&ch) in path.iter().take(length.min(259)).enumerate() { if ch==92||ch==47 {last=i+1;} }
    if last==0||last>240 { wide(b"\\Storage Card\\input-test.txt",&mut path); }
    else { for (i,&ch) in b"input-test.txt\0".iter().enumerate() {unsafe { path.as_mut_ptr().add(last+i).write(ch as u16); }} }
    unsafe { FILE=api!(13,fn(*const u16,u32,u32,usize,u32,u32,usize)->usize)(path.as_ptr(),0x4000_0000,3,0,2,0x80,0); }
    put(b"input-test v1; hex numbers; focus recovery after 1000ms; no user text");end();
    let mut version=Version{size:core::mem::size_of::<Version>() as u32,major:0,minor:0,build:0,platform:0,description:[0;128]};
    field(b"version ok=",api!(25,fn(*mut Version)->i32)(&mut version) as usize);
    field(b" major=",version.major as usize);field(b" minor=",version.minor as usize);field(b" build=",version.build as usize);end();
    unsafe { ASSOC=dynamic(b"aygshell.dll",b"SHSetAppKeyWndAssoc"); field(b"association_available=",usize::from(ASSOC!=0));end(); }
    let mut class=[0u16;32];let class_name=wide(b"PocketInputProbe",&mut class);
    let wc=WndClass{style:0,proc_,cls_extra:0,wnd_extra:0,instance,icon:0,cursor:0,background:6,menu_name:core::ptr::null(),class_name};
    api!(0,fn(*const WndClass)->u16)(&wc);
    unsafe { MAIN=create(instance,class_name,b"Input probe A",0x90c0_0000,0,26,240,294,0,0); }
    let mut button_class=[0u16;16];let button=wide(b"BUTTON",&mut button_class);
    for (id,label,y) in [(101,b"Test short taps".as_slice(),28),(102,b"Associate App1-6 to A".as_slice(),59),(103,b"Associate App1-6 to B".as_slice(),90),(104,b"Release associations".as_slice(),121),(106,b"Query GAPI key list".as_slice(),152)] {
        create(instance,button,label,0x5000_0000,8,y,224,28,unsafe {MAIN},id);
    }
    let mut static_class=[0u16;16];let static_name=wide(b"STATIC",&mut static_class);
    unsafe {
        LABEL=create(instance,static_name,b"Ready; tap test",0x5000_0000,8,0,224,24,MAIN,105);
        TARGET=create(instance,class_name,b"B",0x5080_0000,8,185,224,48,MAIN,108);
        create(instance,static_name,b"B: tap here to focus",0x5000_0000,5,5,210,26,TARGET,109);
        create(instance,static_name,b"Log beside EXE",0x5000_0000,8,240,160,24,MAIN,110);
        create(instance,button,b"Exit",0x5000_0000,180,240,52,24,MAIN,107);
        field(b"windows main=",MAIN);field(b" target=",TARGET);field(b" separate=",usize::from(MAIN!=TARGET));end();
        api!(4,fn(usize,i32)->i32)(MAIN,5);api!(5,fn(usize)->i32)(MAIN);api!(7,fn(usize)->usize)(MAIN);
    }
    let mut message=Msg{hwnd:0,msg:0,wp:0,lp:0,time:0,x:0,y:0};
    let mut last_foreground=0;let mut lost_at=0;let mut next_focus=0;
    loop {
        if api!(9,fn(*mut Msg,usize,u32,u32,u32)->i32)(&mut message,0,0,0,1)!=0 {
            if message.msg==0x12 {break;}
            api!(10,fn(*const Msg)->i32)(&message);api!(11,fn(*const Msg)->isize)(&message);
        } else {api!(18,fn(u32)->usize)(5);}
        let tick=now();
        if tick.wrapping_sub(next_focus)<0x8000_0000 {
            next_focus=tick.wrapping_add(100);let foreground=api!(6,fn()->usize)();
            if foreground!=last_foreground {field(b"foreground tick=",tick as usize);field(b" hwnd=",foreground);end();last_foreground=foreground;lost_at=tick;}
            if foreground!=0 && foreground!=unsafe {MAIN} && foreground!=unsafe {TARGET} && tick.wrapping_sub(lost_at)>=1000 {
                put(b"probe focus recovery");end();unsafe {api!(5,fn(usize)->i32)(MAIN);api!(7,fn(usize)->usize)(MAIN);}
            }
        }
    }
    put(b"end");end();unsafe {api!(15,fn(usize)->i32)(FILE);}0
}
#[panic_handler]
fn panic(_: &PanicInfo)->! {loop {}}
#[no_mangle]
pub unsafe extern "C" fn memset(dst:*mut u8,value:i32,n:usize)->*mut u8 {let mut i=0;while i<n {core::ptr::write_volatile(dst.add(i),value as u8);i+=1;}dst}
#[no_mangle]
pub unsafe extern "C" fn memcpy(dst:*mut u8,src:*const u8,n:usize)->*mut u8 {let mut i=0;while i<n {core::ptr::write_volatile(dst.add(i),core::ptr::read_volatile(src.add(i)));i+=1;}dst}

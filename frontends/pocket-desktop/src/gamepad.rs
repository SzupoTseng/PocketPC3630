//! Gamepad polling, independent of guest CPU / GAPI bindings: Windows XInput here,
//! Linux through the kernel joystick interface (gamepad_linux.rs).
//! ABI: https://learn.microsoft.com/en-us/windows/win32/api/xinput/ns-xinput-xinput_gamepad
//! XInput9_1_0 ships with desktop Windows; no bundled redistributable is needed.
#[derive(Debug)]
pub struct Pad {
    pub index: u8,
    pub pressed: Vec<&'static str>,
}
#[cfg(windows)]
#[repr(C)]
#[derive(Default)]
struct Gamepad {
    buttons: u16,
    left_trigger: u8,
    right_trigger: u8,
    lx: i16,
    ly: i16,
    rx: i16,
    ry: i16,
}
#[cfg(windows)]
#[repr(C)]
#[derive(Default)]
struct State {
    packet: u32,
    pad: Gamepad,
}
#[cfg(windows)]
#[link(name = "xinput9_1_0")]
extern "system" {
    fn XInputGetState(index: u32, state: *mut State) -> u32;
}
#[cfg(windows)]
pub fn poll() -> Vec<Pad> {
    let mut pads = Vec::new();
    for index in 0..4u8 {
        let mut state = State::default();
        // `State` and nested `Gamepad` match XINPUT_STATE / XINPUT_GAMEPAD;
        // only this initialized, correctly sized output buffer is written.
        if unsafe { XInputGetState(index as u32, &mut state) } != 0 {
            continue;
        }
        let p = state.pad;
        let mut pressed = Vec::new();
        for (mask, name) in [
            (1, "PadUp"),
            (2, "PadDown"),
            (4, "PadLeft"),
            (8, "PadRight"),
            (16, "PadStart"),
            (32, "PadSelect"),
            (64, "PadL3"),
            (128, "PadR3"),
            (256, "PadL1"),
            (512, "PadR1"),
            (4096, "PadA"),
            (8192, "PadB"),
            (16384, "PadX"),
            (32768, "PadY"),
        ] {
            if p.buttons & mask != 0 {
                pressed.push(name);
            }
        }
        if p.lx < -16384 && !pressed.contains(&"PadLeft") {
            pressed.push("PadLeft");
        }
        if p.lx > 16384 && !pressed.contains(&"PadRight") {
            pressed.push("PadRight");
        }
        if p.ly < -16384 && !pressed.contains(&"PadDown") {
            pressed.push("PadDown");
        }
        if p.ly > 16384 && !pressed.contains(&"PadUp") {
            pressed.push("PadUp");
        }
        if p.left_trigger > 30 {
            pressed.push("PadL2");
        }
        if p.right_trigger > 30 {
            pressed.push("PadR2");
        }
        pads.push(Pad { index, pressed });
    }
    pads
}
#[cfg(target_os = "linux")]
#[path = "gamepad_linux.rs"]
mod linux;
#[cfg(target_os = "linux")]
pub fn poll() -> Vec<Pad> {
    linux::poll()
}
#[cfg(not(any(windows, target_os = "linux")))]
pub fn poll() -> Vec<Pad> {
    Vec::new()
}
#[cfg(all(test, windows))]
mod tests {
    use super::*;
    #[test]
    fn xinput_abi_sizes_match_the_sdk() {
        assert_eq!(std::mem::size_of::<Gamepad>(), 12);
        assert_eq!(std::mem::size_of::<State>(), 16);
    }
}

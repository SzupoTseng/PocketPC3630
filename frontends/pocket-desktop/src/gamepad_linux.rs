//! Gamepads on Linux through the kernel joystick interface (`/dev/input/js0`..`js3`),
//! with no extra libraries. Buttons and axes follow the layout the `xpad` driver
//! gives Xbox-style pads (the common case), mapped to the same names as the
//! Windows XInput path so one set of bindings works on both.
//! Event format: <linux/joystick.h>, `struct js_event` (8 bytes).

use std::fs::File;
use std::io::{ErrorKind, Read};
use std::os::unix::fs::OpenOptionsExt;
use std::sync::Mutex;
use std::time::{Duration, Instant};

use super::Pad;

const JS_EVENT_BUTTON: u8 = 0x01;
const JS_EVENT_AXIS: u8 = 0x02;
const JS_EVENT_INIT: u8 = 0x80;
/// `O_NONBLOCK` on Linux (x86, x86_64, ARM, AArch64).
const O_NONBLOCK: i32 = 0o4000;
const DEVICES: usize = 4;
/// Missing devices are looked for again this often, so a pad plugged in later is found.
const RESCAN: Duration = Duration::from_secs(2);

/// Button and axis state of one pad.
#[derive(Default)]
pub(crate) struct Js {
    buttons: [bool; 16],
    axes: [i16; 8],
}

impl Js {
    /// Apply one 8-byte `js_event`: time (u32), value (i16), type (u8), number (u8).
    pub(crate) fn apply(&mut self, event: [u8; 8]) {
        let value = i16::from_le_bytes([event[4], event[5]]);
        let number = event[7] as usize;
        match event[6] & !JS_EVENT_INIT {
            JS_EVENT_BUTTON if number < self.buttons.len() => self.buttons[number] = value != 0,
            JS_EVENT_AXIS if number < self.axes.len() => self.axes[number] = value,
            _ => {}
        }
    }

    /// Names of what is held, as on the XInput path.
    pub(crate) fn pressed(&self) -> Vec<&'static str> {
        // xpad: A B X Y LB RB Back Start Guide L3 R3.
        const BUTTONS: [(usize, &str); 10] = [
            (0, "PadA"),
            (1, "PadB"),
            (2, "PadX"),
            (3, "PadY"),
            (4, "PadL1"),
            (5, "PadR1"),
            (6, "PadSelect"),
            (7, "PadStart"),
            (9, "PadL3"),
            (10, "PadR3"),
        ];
        let mut pressed: Vec<&'static str> = BUTTONS
            .iter()
            .filter(|(n, _)| self.buttons[*n])
            .map(|(_, name)| *name)
            .collect();
        let [lx, ly, lt, _rx, _ry, rt, hat_x, hat_y] = self.axes;
        // Stick or D-pad hat; on Linux "up" is negative.
        let mut add = |held: bool, name: &'static str| {
            if held && !pressed.contains(&name) {
                pressed.push(name);
            }
        };
        add(lx < -16384 || hat_x < 0, "PadLeft");
        add(lx > 16384 || hat_x > 0, "PadRight");
        add(ly < -16384 || hat_y < 0, "PadUp");
        add(ly > 16384 || hat_y > 0, "PadDown");
        // Triggers rest at -32767 and reach 32767 fully pressed: past halfway counts.
        add(lt > 0, "PadL2");
        add(rt > 0, "PadR2");
        pressed
    }
}

struct Device {
    file: File,
    js: Js,
}

struct Pads {
    devices: Vec<Option<Device>>,
    last_scan: Option<Instant>,
}

static PADS: Mutex<Pads> = Mutex::new(Pads {
    devices: Vec::new(),
    last_scan: None,
});

pub(crate) fn poll() -> Vec<Pad> {
    let mut pads = PADS.lock().unwrap_or_else(|e| e.into_inner());
    if pads.devices.is_empty() {
        pads.devices = (0..DEVICES).map(|_| None).collect();
    }
    if pads.last_scan.is_none_or(|at| at.elapsed() >= RESCAN) {
        pads.last_scan = Some(Instant::now());
        for (index, slot) in pads.devices.iter_mut().enumerate() {
            if slot.is_none() {
                *slot = std::fs::OpenOptions::new()
                    .read(true)
                    .custom_flags(O_NONBLOCK)
                    .open(format!("/dev/input/js{index}"))
                    .ok()
                    .map(|file| Device {
                        file,
                        js: Js::default(),
                    });
            }
        }
    }
    let mut out = Vec::new();
    for (index, slot) in pads.devices.iter_mut().enumerate() {
        let Some(device) = slot else { continue };
        let mut event = [0u8; 8];
        let unplugged = loop {
            match device.file.read(&mut event) {
                Ok(8) => device.js.apply(event),
                Ok(_) => break false,
                Err(e) if e.kind() == ErrorKind::WouldBlock => break false,
                Err(e) if e.kind() == ErrorKind::Interrupted => continue,
                Err(_) => break true,
            }
        };
        if unplugged {
            *slot = None;
            continue;
        }
        out.push(Pad {
            index: index as u8,
            pressed: device.js.pressed(),
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event(kind: u8, number: u8, value: i16) -> [u8; 8] {
        let v = value.to_le_bytes();
        [0, 0, 0, 0, v[0], v[1], kind, number]
    }

    #[test]
    fn buttons_press_and_release() {
        let mut js = Js::default();
        js.apply(event(JS_EVENT_BUTTON, 0, 1));
        js.apply(event(JS_EVENT_BUTTON, 7, 1));
        assert_eq!(js.pressed(), vec!["PadA", "PadStart"]);
        js.apply(event(JS_EVENT_BUTTON, 0, 0));
        assert_eq!(js.pressed(), vec!["PadStart"]);
    }

    #[test]
    fn initial_state_events_count_like_live_ones() {
        let mut js = Js::default();
        js.apply(event(JS_EVENT_BUTTON | JS_EVENT_INIT, 1, 1));
        assert_eq!(js.pressed(), vec!["PadB"]);
    }

    #[test]
    fn stick_and_hat_give_directions_with_up_negative() {
        let mut js = Js::default();
        js.apply(event(JS_EVENT_AXIS, 1, -32767));
        assert_eq!(js.pressed(), vec!["PadUp"]);
        js.apply(event(JS_EVENT_AXIS, 1, 0));
        js.apply(event(JS_EVENT_AXIS, 6, 32767));
        assert_eq!(js.pressed(), vec!["PadRight"]);
        js.apply(event(JS_EVENT_AXIS, 1, -32767));
        assert_eq!(js.pressed(), vec!["PadRight", "PadUp"]);
    }

    #[test]
    fn triggers_count_past_halfway() {
        let mut js = Js::default();
        js.apply(event(JS_EVENT_AXIS, 2, -32767));
        js.apply(event(JS_EVENT_AXIS, 5, -32767));
        assert!(js.pressed().is_empty());
        js.apply(event(JS_EVENT_AXIS, 5, 20000));
        assert_eq!(js.pressed(), vec!["PadR2"]);
    }

    #[test]
    fn out_of_range_numbers_are_ignored() {
        let mut js = Js::default();
        js.apply(event(JS_EVENT_BUTTON, 40, 1));
        js.apply(event(JS_EVENT_AXIS, 30, 32767));
        assert!(js.pressed().is_empty());
    }
}

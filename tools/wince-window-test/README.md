# Window test program

A small ARM Windows CE program that creates windows the way MFC programs do
(a frame, a child, an owned pop-up, a second frame), shows, activates,
focuses, disables and destroys them, and sends and posts messages. Every
message each window receives, and what `GetParent`, `GetWindow`, `GetFocus`,
`GetActiveWindow` and `GetForegroundWindow` answer between steps, goes to
`window-test.txt` (on the storage card when there is one, else the root).

`reference/device-emulator-ppc2003.txt` is its log from Microsoft Device
Emulator running the Pocket PC 2003 image. Running it in PocketPC3630 and
comparing the two shows where our window behaviour differs from a device:

```
build.bat                                     # out\window-test.exe
pockethle run --cpu interp --rom-dir out\root --rom-prefix \ out\window-test.exe
python compare.py out\root\window-test.txt
```

Set `POCKETHLE_SEPARATE_WINDOWS=1` to give it a handle per window, as MFC
programs get; without it every window shares the main window's handle.

## Building

No Windows CE compiler is needed. The program is Rust without the standard
library, compiled for ARMv5 (`armv5te-unknown-linux-gnueabi`, the
instruction set of XScale devices and of Device Emulator), linked with the
`rust-lld` that comes with Rust at the addresses the PE image will use, and
turned into a PE by `elf2pe.py`, which also writes the import table. It
calls coredll through one table of function pointers that becomes the
import address table.

Needs `rustup target add armv5te-unknown-linux-gnueabi` and Python 3.

## Running it on Device Emulator

Copy `out\window-test.exe` into a folder of the emulator's shared folder
(the storage card), open it from File Explorer, and read `window-test.txt`
from the shared folder afterwards.

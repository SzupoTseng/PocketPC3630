# CE input reference probe

This small ARM probe contains no game code, registration data or CE SDK code.
It runs on the user's Pocket PC 2003 Device Emulator and PocketHLE. Build with
`tools\wince-input-test\build.bat` from Windows CMD; output is in
`target/ui-validation/input-probe/`.

Copy `input-test.exe` to writable shared storage. `input-test.txt` is created
beside the executable. Numbers are hexadecimal. The log includes OS version,
raw key messages, lParam scan/repeat/transition bits, mouse messages, guest tick,
focus, button commands and application-key association results. Key-state
results are normalized to the API's 16-bit return width.

The window offers a short-touch counter, assignment of App1–6 to the main
window A or child B, release of associations, and optional GAPI key-list query.
For the A/B comparison, launch PocketHLE with `POCKETHLE_SEPARATE_WINDOWS=1`;
the existing legacy single-window mode otherwise aliases A and B. The log
records `separate=0` in that case and cannot validate association to B.

Query GAPI only in its separate phase: it changes the application's input
contract. The probe restores its foreground window after one second if a
shell application takes focus; that recovery is logged and must not be
mistaken for hardware-key routing. It does not register a global hotkey.

`SHSetAppKeyWndAssoc` uses the documented `BOOL(BYTE, HWND)` signature;
see [Microsoft's CE API reference](https://learn.microsoft.com/en-us/previous-versions/windows/embedded/aa453734(v=msdn.10)).
Dynamic imports use the Unicode
[CE GetProcAddress contract](https://learn.microsoft.com/en-us/previous-versions/ms908453(v=msdn.10)).
A successful call does not make a recorded stale GetLastError value an error.

Pin ROM, emulator executable, skin XML and image hashes before comparison.
Skin scan codes are not guest virtual-key observations. A passing synthetic
probe does not establish game compatibility or verify unmeasured device skins.

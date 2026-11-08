# Windows CE dialog reference probe

This standalone ARM probe compares dialog geometry, styles, initial focus,
font metrics and system colors against Pocket PC 2003 Device Emulator. It
contains no game code or registration data.

Build from the repository root on Windows with `tools\wince-dialog-test\build.bat`.
Copy the resulting executable from `target/eq-validation/dialog-probe/` to the
emulator's shared storage card and launch it. The probe writes
`\Storage Card\dialog-test.txt` (falling back to the root directory).
Run the same executable in PocketHLE with a writable storage-card mount.

`reference/capture.json` identifies the executable used for the saved logs.
`reference/device-emulator-ppc2003.txt` is the reference;
`reference/pocketpc3630-before.txt` records the current implementation before
follow-up dialog fixes. These logs expose missing window styles, initial edit
focus, caption placement, font-dependent dialog units, and CE-masked system
colors. They do not establish that every top-level dialog must be fullscreen.

EQ's registration layout still requires a separate game screenshot comparison;
passing this synthetic probe alone is insufficient for game acceptance.

Later captures are identified in `capture.json`. To list all remaining
observation differences (exit 1 means differences remain):

```bash
python3 tools/wince-dialog-test/compare.py \
  tools/wince-dialog-test/reference/device-emulator-ppc2003.txt \
  tools/wince-dialog-test/reference/pocketpc3630-styles.txt \
  --output target/eq-validation/style-comparison.json
```

Initial edit focus and the three CE-tagged colors now match. Most dialog
style queries also match; the visible bit during child WM_INITDIALOG still
differs. Caption geometry and font metrics are still incomplete. Observation
counts include repeated lifecycle snapshots and are not a game compatibility
percentage.

# EQ EP1–3 validation

The current baseline tests the pre-existing `docs/pockethle.{js,wasm}`, not the
working Rust source. A successful import/boot is not a gameplay pass.

Run from the repository root with Node:

```sh
node tools/eq-validation/smoke.mjs docs /path/to/your-package.zip target/eq-validation/ep1 20
```

The package should contain the original CAB and a locally obtained compatible
`mfcce300.dll`. The runner records package/build and imported EXE/DLL SHA-256, import/boot outcomes,
bounded warnings and periodic PPM screenshots. By default it sends no input. Optional scenarios only send pointer taps and
take screenshots; they cannot supply a registration code. Keep packages and runtime screenshots under the
ignored `target/` directory. Do not commit game files or system DLLs.

`presentationEvents` counts frontend serial changes, not distinct pictures.
`distinctCapturedFrames` counts unique periodic screenshots only, not every
rendered frame. This distinction matters: the baseline registration screen
reported thousands of serial changes while all sampled pictures were identical.
Do not use that event count as game FPS.

## 2026-11-07 baseline

See `baseline-2026-11-07.json` for reproducible build/package identities.

| Episode | Import | Boot | Visual inspection | Gameplay/save/reload |
|---|---|---|---|---|
| EP1 local candidate | Pass | Pass | Registration dialog, no execution error over 20 seconds | Not tested |
| EP1 disc CAB | Pass | Pass | Independently extracted disc CAB; registration dialog over 20 seconds | Not tested |
| EP2 | Not run | Not run | Available E2 CABs were flagged as modified in the earlier handoff | Not tested |
| EP3 | Pass | Pass | Owner name and registration dialog, no execution error over 20 seconds | Not tested |

The EP1/EP3 inputs are local original-CAB candidates. The registration screen
alone does not prove authenticity. Source provenance must be checked separately.
The existing build still substitutes the copyright symbol with a question mark.

## Acceptance checklist for each episode

1. Record source provenance, CAB hash, game EXE hash and dependency hashes.
2. Build the current commit; record commit, working diff and toolchain.
3. Import into an empty library, launch, inspect registration and input controls.
4. Register with the user's own code, if available, without logging that code.
5. Create a character, finish tutorial, move, fight, loot and change area.
6. Verify real audio output separately from submitted PCM data.
7. Save, close the guest, reopen, and compare character/progress/inventory.
8. Repeat with each intended frontend; record platform-specific outcomes.

At the original baseline capture, Windows interop failed inside the sandbox.
A later recheck succeeded when Windows commands were invoked separately from
Linux setup commands: the current kernel passed all 123 tests, including the
two composition regressions, and workspace formatting passed. The rebuilt WASM
has been tested separately under `target/eq-validation/current-wasm`; the
original baseline files still describe the old `docs/` build. The speculative
static-label wrapping change was withdrawn pending device comparison.

The EP1 disc CAB is 6,769,289 bytes; the first local candidate is 6,851,236
bytes. They are distinct builds and have visibly different registration layouts.
See `ep1-disc-source.json` for the disc extraction identity. A registered copy
is still required for post-registration tests.

Runner contract tests (mocked interface, not game execution):

```sh
node --test tools/eq-validation/smoke.test.mjs
```

These tests passed locally. They check failed imports retain a failure report
and clean exits save a correctly encoded RGB screenshot without being called
a crash.

## Pointer-only registration scenarios

```sh
node tools/eq-validation/smoke.mjs docs /path/to/ep1-disc.zip target/eq-validation/ep1-empty 6 tools/eq-validation/scenarios/ep1-disc-empty-code.json
node tools/eq-validation/smoke.mjs docs /path/to/ep3.zip target/eq-validation/ep3-cancel 6 tools/eq-validation/scenarios/ep3-cancel.json
node tools/eq-validation/scenario.test.mjs
node tools/eq-validation/smoke.test.mjs
```

Scenario coordinates are specific to the baseline layouts; do not silently
reuse them after changing dialog placement. Each action has a relative `at`
time in seconds and exactly one `tap: [x,y]` or safe `shot` filename. Times must
be ordered; a tap includes a down and a release 120 ms later. Unknown fields,
keyboard input, out-of-screen coordinates and path traversal are rejected.

Visually inspected empty-code OK runs: EP1 disc and EP3 stayed on their
registration screens, displayed button focus, and did not crash. Cancel runs
for the local EP1 candidate, EP1 disc and EP3 exited with an empty error string. These
are registration-control checks, not gameplay or successful-registration passes.

See `registration-2026-11-07.json` for observed registration interactions,
actual event timings, screenshot fingerprints and imported module identities.
Empty-code screenshots were inspected. Cancel outcomes were checked against
`stopped=true`, `stopReason=clean-exit`, and an empty error. These are recorded
separately from executable source changes.

## Rebuilt artifact verification

`kernel-verification-2026-11-07.json` records 123 passing kernel tests, passing
workspace formatting, and the precise local Clippy compatibility invocation.
`current-comparison-2026-11-07.json` compares the rebuilt WASM with the original
baseline using identical packages. EP1 disc retained identical pixels with
2 presentation events instead of 2,088 over 20 seconds; EP3 had 1 instead of
2,052 and now shows distinct copyright/registered glyphs. These are idle
registration observations, not gameplay frame-rate measurements.

`native-registration-2026-11-07.json` records the newly built Windows CLI
(Unicorn) results: EP1/EP3 final registration pixels match the current WASM
byte-for-byte. Those native runs ended at their intentional 50,000-slice limit;
this does not prove a clean user-requested game exit or full gameplay.

All changes remain local; no commit or push has been made.

> **Sharing rule.** Some runs below used modified game packages (a replaced
> executable) or supplied registration data, so that play could be tested past
> the registration screen. What they showed about the emulator is kept and
> fixed in the code. The modified files themselves are never part of this
> repository or of anything we publish; shipped builds and the library use
> only the original packages and the player's own registration.

## Native interactive EP1 / EP2 / EP3 checks

`native-ui.py` runs on Windows with pywin32 and Pillow. It uses the isolated
library in `target/eq-validation/modified-library`, records its own process
ID, and controls only that process's `PocketPC3630` window. It supports
launch, screenshots, taps, stylus drags, short ASCII test names, a limited
set of keys, status and window close. Run commands serially; inspect the
screen between dependent actions. Physical pointer input is used because
posting mouse messages alone does not update minifb's polled button state.
The helper raises its own window and checks that the target point belongs
to it before clicking.

```bat
python tools\eq-validation\native-ui.py launch 1
python tools\eq-validation\native-ui.py shot target\eq-validation\ep1-current.png
python tools\eq-validation\native-ui.py tap 120 45
python tools\eq-validation\native-ui.py close
```

Do not infer a terminal screen from the last strided PPM dump. A stride of
50 can omit the final 49 changed frames. Use the live display screenshot
or explicitly preserve the CLI's final framebuffer after each serial run.
The CLI's shared final snapshot path is not safe for concurrent runs.

The separate modified packages are authorized by the user's patch request.
Their results do not validate original registration behavior. EP3 has a separate world-entry verification report.

EP3's bundled instruction file is kept locally under ignored
`target/eq-validation/ep3-install-instructions.txt`. `launch 3 register`
loads that value into the game's own `RegistrationE3` setting and replays
the CAB's owner entry. The value is not embedded in this tool or reports.
This path tests startup with supplied registration data; the MFC dialog's
owner field still needs separate initialization and rendering fixes.

Keyboard messages sent by the helper carry Windows scan codes. The native
frontend compares held key states to emit each guest key transition once,
independent of minifb's window-update timing.

After rebuilding the importer, EP3 can also be imported into the separate
`target/eq-validation/ep3-clean-library`. `launch 3 register clean` uses
that installation and the shared EP3 save directory. Native verification
restored the same Level 25 Warrior in Butcherblock Mtns there. The save
folder includes both character payloads and `pockethle-registry.dat`;
character-list registry data is migrated from the legacy installation
when the save snapshot is first created.


### Web progress and actual browser verification

`smoke.mjs` accepts an optional sixth argument after the scenario JSON:
a save ZIP produced by the launcher's save export. Progress is imported
before boot. Save data and any bundled registration material stay under
ignored `target/eq-validation`; never place them in reports or source.

`browser-smoke.mjs` drives a real Chromium page through CDP, imports a
package and save through the page's file inputs, loads the supplied role,
uses pointer input, saves, reloads the page and checks the library again.
Run each case serially: its local server uses port 8174 and CDP uses 9336.
Supply a separate test-only browser profile for every package. Example:

```bat
node tools\eq-validation\browser-smoke.mjs docs package.zip progress.zip target\eq-validation\browser-case "C:\Program Files\Google\Chrome\Application\chrome.exe" target\eq-validation\browser-profile 0
```

The final argument is the saved character slot (0–3). Reports separate
uncaught runtime exceptions from console warnings; `ok` alone does not
prove the captured image is a world. Inspect `world-canvas.png` and
`reloaded-world-canvas.png` before recording a gameplay result. A browser
run is not an Android device test or an audio listening test.

### EQ audio completion and progress-preserving patches

`complete-audio-materials.py` builds full local test packages from the existing
ignored EQ libraries. `audio-materials.json` records donor paths and SHA-256:
EP2 borrows six missing rat/crocodile clips from the supplied EP1 assets;
EP3 borrows fifteen clips from the supplied episodes. The shared original
sounds compared across episodes had matching hashes. Donor use does not prove
bit identity with an unavailable original copy of the missing file.

For an already installed game, run `create-audio-patches.py` and import the
resulting `ep2-audio-patch.zip` / `ep3-audio-patch.zip` through the normal game
import button. Return to the library before importing. These small ZIPs only
add missing `Sounds/*.wav`; they retain the installed game, settings, registry
and save files. An existing sound with different bytes is retained and the
patch is rejected. Reapplying the same patch is harmless. Game audio remains
under ignored `target/eq-validation/audio-packages`, never in source.

The shared library recognizes `pockethle-audio.json` version 1 before ordinary
CAB/ZIP import. The manifest names an existing game id and the supplied sound
paths. Paths, RIFF/WAVE envelopes, entry counts and decompressed sizes are
checked before writes. CAB packages with supplemental `Sounds/` files also
retain those files when their inner CAB needs checksum recovery.

With `EQ_AUDIO_PROBE=1`, the real Chromium runner observes Web Audio's output
callback and stores metrics and up to ten seconds of audible PCM excerpts.
The WAV is an excerpt, not a continuous game recording. Missing FMOD files or
zero mixed output fail that audio run. An optional argument after the saved
character slot names an audio-only patch ZIP; the runner imports it after the
save and compares hashes of the save files and manifests before boot.

`web-audio.test.mjs` checks that restarting Web audio closes the old context,
disconnects its callback and frees its WASM buffer. The common FMOD tests cover
live master gain, per-channel pause, stream-specific stop, independent guest
state, free-channel allocation and keeping looping music alive across more
than 32 effects, stream replay without layering, and silence after the engine
is dropped even if a frontend still holds its tap. The explicitly ignored local-material test can validate the
user's actual clips when `POCKETHLE_EQ_AUDIO_DIRS` is supplied; ordinary CI
ships no game audio.

Output evidence has separate meanings: decoding validates source data;
`AudioTap::render_mix` validates shared mixing; native cpal and real Web Audio
callbacks validate their output paths; Android logs non-silent PCM accepted by
AudioTrack and an advancing playback head. None alone is an acoustic listening
test, full combat-sound coverage or ARM64 physical-device acceptance. A flushed
Android track can report a reset final playback head; use the earlier advancement
event rather than treating that final number as the session duration.


Latest consolidated audio results are in `audio-verification.json` (2026-11-08).
They bind actual output evidence to the tested executable/WASM/APK hashes.
Android Unicorn and the release CPU configuration (`--no-default-features`,
Interpreter) use separate debug-signed APKs; neither is an ARM64 phone test.
Generated audio-only ZIPs belong to the user's local materials and are not
distributed with this repository.

### Web library introductions and control preferences

`library-ui.test.mjs` covers configuration validation, queued persistence and
multi-source input cleanup. `library-ui-browser.mjs` uses real Chrome / WASM
with an isolated profile and user-local packages to exercise introductions,
preview isolation, per-program controls, persistence failures, viewport sizing
and ten session restarts. It never downloads game material. See
`proof/library-ui/README.md` and `library-ui-verification.json` for exact scope;
the Device Emulator panel is a disabled informational preview and native
frontends are not included in this UI acceptance.

## Shared UI validation (2026-11-08)

Current implementation and remaining acceptance: [UI status](../../docs/UI_IMPLEMENTATION_STATUS.md).
The [shared UI evidence](ui-implementation-verification.json) binds accepted Web
checks and episode runs to their exact artifacts; native and Android acceptance
are listed separately. [Screenshots](../../proof/ui-shared/README.md) are real
frontend captures, not catalog illustrations. `check-ui.cmd`,
`build-ui-wasm.cmd`, `build-ui-android.cmd` and `package-ui-android.py` reproduce
the build stages. Use a dedicated library/profile and owned emulator; never
replace normal game progress or registration material with test fixtures.

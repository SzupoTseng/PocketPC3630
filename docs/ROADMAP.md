# Roadmap

Worked out with an outside review (Codex) in November 2026 and agreed step by
step. Legal and distribution questions are kept separately.

Every step that changes Rust passes the same gate: unit tests, the WebAssembly
build, the 157-program regression with no program getting worse, `cargo fmt`,
`cargo clippy` and CI. One change at a time; a change that makes anything worse
is reverted, not patched over.

| # | Step | Size | Depends on |
|---|---|---|---|
| 1 | Correct the documentation about MFC windows | S | — |
| 2 | Registration consistency tests | M | — |
| 3 | Window conformance program, run on Device Emulator and here | M | — |
| 4 | Audit Haali Reader and SimCity 2000 against it | M | 3 |
| 5 | Shell, document-list and GDI gaps that block 3–5 programs each | M | 4 (audit) |
| 6 | Interpreter: executable pages and instruction-cache invalidation | M | — |
| 7 | Interpreter: per-page predecoded ARM instructions | L | 6 |
| 8 | Split `coredll.rs` by area, moves only | L | 4 |
| 9 | Quarterly compatibility report from the 1,443-download sweep | S | — |

## 1. MFC windows in the documentation
MFC programs have had windows of their own by default since 0d0440c
(`POCKETHLE_MFC_WINDOWS=0` turns it off); the README still said every window
shared one handle. Haali Reader and SimCity 2000 still stop, for reasons step 4
finds.

## 2. Registration consistency
Handlers stay registered the way they are. Tests check that the separate tables
agree:
- every native thunk and every interpreter intrinsic names a registered
  `(dll, name)`;
- every constant-returning registration has the same value as its handler;
- every ordinal alias points at the name in the ordinal tables and the same
  handler;
- every implemented ordinal export appears in its ordinal table;
- registering the same name twice is allowed only with the same handler and
  value (recorded in test builds; today a second registration silently wins).

Ordinal exports published without a handler (so calls can be traced) are
listed as such, not treated as errors. New APIs may use a small descriptor;
existing ones are not rewritten.

## 3. Window conformance program
A small ARM program whose window procedures write a log to a file: handles,
parent and owner, thread ids, focus and activation, and the order of creation,
messages and destruction. Cases: nested `CreateWindowEx`, owned versus child
windows, `DestroyWindow` on a parent, `WM_CREATE` arriving inside
`CreateWindowEx`, activation and focus order, `PostThreadMessage`, which thread
owns which queue. The same program runs on Device Emulator and here; the two
logs are compared. Our own API traces show only what we do, so they cannot be
the reference.

**Done:** `tools/wince-window-test` builds the program without a Windows CE
compiler (Rust for ARMv5, `rust-lld`, a small ELF-to-PE converter), and
`reference/device-emulator-ppc2003.txt` is its log from Device Emulator.
Building it found a CPU bug on the way: the interpreter (browser and
Android) skipped ARMv5's `BLX Rm` and took `LDRD` for a halfword store.

With a handle per window (`POCKETHLE_SEPARATE_WINDOWS=1`) the comparison
lists what to fix in step 4:
- an owned pop-up's owner is lost (`GetParent` and `GW_OWNER` give 0);
- `GetWindow(GW_CHILD)` answers 0;
- focus and activation are not tracked per window: `SetFocus`,
  `SetActiveWindow`, `SetForegroundWindow` and `ShowWindow` send no
  `WM_ACTIVATE`, `WM_SETFOCUS` or `WM_KILLFOCUS`, and focus stays on the
  first window;
- `DestroyWindow` sends `WM_DESTROY` later instead of at once, leaves owned
  and child windows alive, and `IsWindow` still answers 1;
- a child window gets `WM_MOVE` and `WM_SIZE` during `CreateWindowEx` on a
  device, later here;
- `EnableWindow` sends no `WM_ENABLE`.

## 4. Haali Reader and SimCity 2000
Run both with API traces and check them against what step 3 established. Each
stop is tied to a named difference or a missing API before anything is fixed;
fixes go in one at a time.

## 5. Shell, document-list and GDI gaps
After step 4's audit: AYGSHELL lifecycle calls, `SHGetShortcutTarget`,
`DocList_Create` and friends, `SetViewportOrgEx`, each in its own commit.
AYGSHELL ordinal 209 is not in the ordinal table; where it comes from is
established before it is implemented.

## 6. Instruction-cache correctness
Today the interpreter keeps no record of which pages hold code, `write_mem`
does not invalidate anything and `flush_instruction_cache` does nothing. Before
any decoding is cached: mark executable pages, invalidate on guest stores
(`STR`, `STM`, unaligned and cross-page), host writes, range and full flushes,
DLL loading and rebasing, and code-page reuse. Tests include write-then-execute
without a flush, the stack thunks some games build, and GapiDraw's generated
blitters. Behaviour does not change.

## 7. Predecoded ARM instructions
Decode each instruction of a code page once into an operation with its operands
already extracted; execution keeps retiring one instruction at a time, so hooks,
conditions and errors behave exactly as now. Measured on EverQuest, PowerVR
Racer and Bejeweled 2 with a microbenchmark and the differential CPU tests. Kept
only with a measurable speed-up and an identical regression; basic blocks are a
separate, measured proposal afterwards.

## 8. Splitting `coredll.rs`
Moves only, one area per commit, the full gate after each, in this order:
C runtime and strings, registry, wave audio, files, loader and resources,
threads and synchronisation, windows and messages (last, after step 4 has
settled that code). Kernel state is split into sub-structures afterwards, and
only where an area's state stands on its own, as `WaveOutState` already does.

## 9. Compatibility report
The 157-program regression stays the gate for each change. Every quarter the
1,443-download sweep runs again and the report records, in fixed categories:
how many programs fail to start, the distinct missing DLLs and APIs, the top
blockers by programs affected, programs that moved out of "does not start",
and regressions. "MFC not present" and "MFC present but fails" are counted
apart.

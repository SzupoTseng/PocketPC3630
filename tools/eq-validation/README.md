# EverQuest web validation tools

Scripts that drive the real web build with a package the tester supplies,
from a parallel Codex workspace (2026-11-07). Nothing here contains game
files, registration data or results: packages, saves, screenshots and
reports go to an ignored output folder given on the command line.

| Script | What it does |
|---|---|
| `smoke.mjs` | Runs `docs/pockethle.{js,wasm}` in Node with a package, pointer input only, and records start-up, errors, presentation events and screenshots. Can import a save ZIP before boot. |
| `scenario.mjs` | Runs a timed list of taps and screenshots (`scenarios/*.json`, screen coordinates of a 240x320 device) through `smoke.mjs`. |
| `browser-smoke.mjs` | Opens the real page in headless Chromium with an isolated profile and a loopback server, imports a package and progress through the page's own file inputs, taps the canvas and checks the IndexedDB reload. |
| `browser-audio-probe.js` | Optional (`EQ_AUDIO_PROBE=1`): reports whether the page's Web Audio callback produced non-silent output. |

The scenarios cover only what the original packages show before
registration (cancel, an empty code): play after registration needs the
player's own registration code.

Tests (from the repository root):

```
node --test tools/eq-validation/smoke.test.mjs tools/eq-validation/scenario.test.mjs tools/eq-validation/web-audio.test.mjs
```

Results recorded with modified executables or registration data that is
not the tester's own are not part of this repository.

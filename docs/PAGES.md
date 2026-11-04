# Browser version on GitHub Pages

`pocket-wasm` is PocketPC3630 compiled to WebAssembly. The site holds only the
emulator: each player uploads their own game package, and it stays in that
browser. There is no server, and game files must never be committed here.

## Publish

1. Build `pocket-wasm` for `wasm32-unknown-emscripten` with
   `frontends/pocket-wasm/build.sh` (needs an activated emsdk, 3.1.74). It holds
   the link flags and the list of functions the page calls; CI builds the same
   way.
2. Run `node frontends/pocket-wasm/web/prepare-pages.mjs [build dir]`, which
   copies the page and build into `docs/` and refuses a build containing local
   paths.
3. Commit `docs/` and push it to `main`.
4. In Settings -> Pages choose *Deploy from a branch*, `main`, `/docs`.

## What players upload

A `.zip` or `.cab`. A zip either holds one `.cab` in its root plus extra files,
or the installed game's folders. Those folders also appear at the root of the
device, which is where titles such as Pocket Quake expect `\quake\id1`.

## Saves

Progress is kept in IndexedDB and written every 10 seconds, whenever the tab is
hidden and when it closes. Clearing site data removes it, so each game has
buttons to download, import and reset a save. A save zip holds the game's save
folder and its registry; an import accepts nothing else. Only one tab may play at
a time, since two would overwrite each other's progress.

## Checking a build

`node frontends/pocket-wasm/web/verify.mjs <build dir> <package.zip> <out.ppm>
[seconds] [taps]` runs the real build in Node through the page's call sequence.

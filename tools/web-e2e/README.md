# Web end-to-end test

`e2e.mjs` opens the built web page in a real headless Chrome and runs our own
input probe (`tools/wince-input-test`, no game files). It fails on any uncaught
page error and checks that the WASM starts, the software library shows cards
and a story, the probe draws, pausing stops the guest clock, the interface
theme switches, and leaving returns to the library. CI runs it after the web
build.

Locally (the page must contain a built `pockethle.wasm`, e.g. after
`frontends/pocket-wasm/build.sh` and `prepare-pages.mjs`):

```sh
tools/wince-input-test/build.sh            # or build.bat on Windows
(cd target/ui-validation/input-probe && zip ../input-test.zip input-test.exe)
CHROME=/path/to/chrome node tools/web-e2e/e2e.mjs docs target/ui-validation/input-test.zip
```

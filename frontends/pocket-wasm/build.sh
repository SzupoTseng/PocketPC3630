#!/usr/bin/env bash
# Build the browser version: target/wasm32-unknown-emscripten/release/
# pockethle.{js,wasm}. Needs Rust's wasm32-unknown-emscripten target and an
# activated emsdk (3.1.74 is what the published site is built with).
# Extra rustc flags (for example -D warnings) can be passed in EXTRA_RUSTFLAGS.
set -euo pipefail
cd "$(dirname "$0")/../.."

# Every function the page calls. A new `pocket_*` export in src/main.rs must
# be added here too, or the page cannot see it.
exports=(
  _main _malloc _free
  _pocket_import _pocket_game_count _pocket_game_name _pocket_game_id
  _pocket_save_export _pocket_save_import _pocket_save_reset
  _pocket_boot_warning _pocket_import_note _pocket_boot _pocket_tick
  _pocket_frame_ptr _pocket_frame_serial _pocket_width _pocket_height
  _pocket_pointer _pocket_key _pocket_audio _pocket_last_error
  _pocket_million_instructions _pocket_set_lang _pocket_delete_game
)
EX=$(IFS=,; echo "${exports[*]}")

link=(
  "-sEXPORTED_FUNCTIONS=$EX"
  -sEXPORTED_RUNTIME_METHODS=ccall,FS,HEAPU8,HEAPF32,UTF8ToString,stringToNewUTF8
  -sALLOW_MEMORY_GROWTH=1 -sINITIAL_MEMORY=134217728 -sSTACK_SIZE=8388608
  -sMODULARIZE=1 -sEXPORT_NAME=PocketHLE -sFORCE_FILESYSTEM=1 -lidbfs.js
  -sENVIRONMENT=web,node -sEXIT_RUNTIME=0
)
flags="-C panic=abort ${EXTRA_RUSTFLAGS:-}"
for arg in "${link[@]}"; do
  flags+=" -C link-args=$arg"
done

RUSTFLAGS="$flags" cargo build --release -p pocket-wasm --target wasm32-unknown-emscripten
ls -l target/wasm32-unknown-emscripten/release/pockethle.js target/wasm32-unknown-emscripten/release/pockethle.wasm

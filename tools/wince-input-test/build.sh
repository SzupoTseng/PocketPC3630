#!/usr/bin/env bash
# Build input-test.exe on Linux or macOS (build.bat does the same on Windows).
# Needs Rust's armv5te-unknown-linux-gnueabi target and Python 3.
set -euo pipefail
cd "$(dirname "$0")/../.."
OUT=target/ui-validation/input-probe
mkdir -p "$OUT"
rustc -D warnings --edition 2021 --target armv5te-unknown-linux-gnueabi -C panic=abort -C opt-level=s \
  -C relocation-model=static -C target-cpu=arm926ej-s --emit obj -o "$OUT/input-test.o" tools/wince-input-test/src/main.rs
SYSROOT=$(rustc --print sysroot)
LLD=$(ls "$SYSROOT"/lib/rustlib/*/bin/rust-lld | head -1)
"$LLD" -flavor gnu -T tools/wince-window-test/link.ld --no-pie -o "$OUT/input-test.elf" "$OUT/input-test.o"
python3 tools/wince-input-test/convert.py "$OUT/input-test.elf" "$OUT/input-test.exe"

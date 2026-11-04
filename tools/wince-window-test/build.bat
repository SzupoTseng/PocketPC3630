@echo off
rem Build window-test.exe for ARMv5 Windows CE. Needs Rust's
rem armv5te-unknown-linux-gnueabi target (rustup target add ...) and Python.
cd /d "%~dp0"
if not exist out mkdir out
rustc --edition 2021 --target armv5te-unknown-linux-gnueabi -C panic=abort -C opt-level=s ^
  -C relocation-model=static -C target-cpu=arm926ej-s --emit obj -o out\window-test.o src\main.rs || exit /b 1
for /f "delims=" %%L in ('rustc --print sysroot') do set "SYSROOT=%%L"
"%SYSROOT%\lib\rustlib\x86_64-pc-windows-msvc\bin\rust-lld.exe" -flavor gnu -T link.ld --no-pie -o out\window-test.elf out\window-test.o || exit /b 1
python elf2pe.py out\window-test.elf out\window-test.exe

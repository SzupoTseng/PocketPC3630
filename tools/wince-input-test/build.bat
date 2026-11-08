@echo off
setlocal
cd /d "%~dp0..\.."
set "OUT=target\ui-validation\input-probe"
if not exist "%OUT%" mkdir "%OUT%"
rustc -D warnings --edition 2021 --target armv5te-unknown-linux-gnueabi -C panic=abort -C opt-level=s -C relocation-model=static -C target-cpu=arm926ej-s --emit obj -o "%OUT%\input-test.o" tools\wince-input-test\src\main.rs
if errorlevel 1 exit /b %ERRORLEVEL%
for /f "delims=" %%L in ('rustc --print sysroot') do set "INPUT_SYSROOT=%%L"
"%INPUT_SYSROOT%\lib\rustlib\x86_64-pc-windows-msvc\bin\rust-lld.exe" -flavor gnu -T tools\wince-window-test\link.ld --no-pie -o "%OUT%\input-test.elf" "%OUT%\input-test.o"
if errorlevel 1 exit /b %ERRORLEVEL%
python tools\wince-input-test\convert.py "%OUT%\input-test.elf" "%OUT%\input-test.exe"
exit /b %ERRORLEVEL%

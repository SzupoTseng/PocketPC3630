@echo off
setlocal
cd /d "%~dp0..\.."
set "OUT=target\eq-validation\dialog-probe"
if not exist "%OUT%" mkdir "%OUT%"
rustc -D warnings --edition 2021 --target armv5te-unknown-linux-gnueabi -C panic=abort -C opt-level=s -C relocation-model=static -C target-cpu=arm926ej-s --emit obj -o "%OUT%\dialog-test.o" tools\wince-dialog-test\src\main.rs
if errorlevel 1 exit /b %ERRORLEVEL%
for /f "delims=" %%L in ('rustc --print sysroot') do set "SYSROOT=%%L"
"%SYSROOT%\lib\rustlib\x86_64-pc-windows-msvc\bin\rust-lld.exe" -flavor gnu -T tools\wince-window-test\link.ld --no-pie -o "%OUT%\dialog-test.elf" "%OUT%\dialog-test.o"
if errorlevel 1 exit /b %ERRORLEVEL%
python tools\wince-dialog-test\convert.py "%OUT%\dialog-test.elf" "%OUT%\dialog-test.exe"
exit /b %ERRORLEVEL%

@echo off
setlocal
cd /d "%~dp0..\.."
if not exist target\ui-validation mkdir target\ui-validation
cargo fmt --all > target\ui-validation\fmt.log 2>&1
if errorlevel 1 exit /b 1
cargo test --workspace --offline > target\ui-validation\tests.log 2>&1
if errorlevel 1 exit /b 1
cargo clippy --workspace --all-targets --offline > target\ui-validation\clippy.log 2>&1
if errorlevel 1 exit /b 1
node tools\eq-validation\library-ui.test.mjs > target\ui-validation\library-node.log 2>&1
if errorlevel 1 exit /b 1
node tools\eq-validation\controls-host.test.mjs > target\ui-validation\controls-node.log 2>&1
if errorlevel 1 exit /b 1
node tools\eq-validation\smoke.test.mjs > target\ui-validation\smoke-node.log 2>&1
if errorlevel 1 exit /b 1
node tools\eq-validation\web-audio.test.mjs > target\ui-validation\audio-node.log 2>&1
if errorlevel 1 exit /b 1
cargo build --release -p pocket-cli -p pocket-desktop --features unicorn --offline > target\ui-validation\native-build.log 2>&1
exit /b %ERRORLEVEL%

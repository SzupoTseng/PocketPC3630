@echo off
setlocal
cd /d "%~dp0..\.."
if not defined EMSDK set "EMSDK=G:\emsdk"
call "%EMSDK%\emsdk_env.bat" >nul 2>&1
if errorlevel 1 exit /b %ERRORLEVEL%
set "CARGO_TARGET_DIR=target\eq-wasm"
set "RUSTFLAGS=--remap-path-prefix=%USERPROFILE%\.cargo\registry\src=/cargo --remap-path-prefix=%USERPROFILE%\.rustup=/rustup -C link-args=-sEXPORTED_FUNCTIONS=_main,_malloc,_free,_pocket_import,_pocket_game_count,_pocket_game_name,_pocket_game_id,_pocket_save_export,_pocket_save_import,_pocket_save_reset,_pocket_boot_warning,_pocket_import_note,_pocket_boot,_pocket_tick,_pocket_frame_ptr,_pocket_frame_serial,_pocket_width,_pocket_height,_pocket_pointer,_pocket_key,_pocket_audio,_pocket_last_error,_pocket_million_instructions,_pocket_set_lang,_pocket_delete_game,_pocket_controls_json,_pocket_controls_write,_pocket_set_paused,_pocket_set_volume,_pocket_guest_time,_pocket_text -C link-args=-sEXPORTED_RUNTIME_METHODS=ccall,FS,HEAPU8,HEAPF32,UTF8ToString,stringToNewUTF8 -C link-args=-sALLOW_MEMORY_GROWTH=1 -C link-args=-sINITIAL_MEMORY=134217728 -C link-args=-sSTACK_SIZE=8388608 -C link-args=-sMODULARIZE=1 -C link-args=-sEXPORT_NAME=PocketHLE -C link-args=-sFORCE_FILESYSTEM=1 -C link-args=-lidbfs.js -C link-args=-sENVIRONMENT=web,node -C link-args=-sEXIT_RUNTIME=0 -C panic=abort"
cargo build --release -p pocket-wasm --target wasm32-unknown-emscripten --offline
if errorlevel 1 exit /b %ERRORLEVEL%
set "R=target\eq-wasm\wasm32-unknown-emscripten\release"
set "D=target\ui-validation\web"
if not exist "%D%" mkdir "%D%"
copy /y "%R%\pockethle.js" "%D%" >nul
if errorlevel 1 exit /b %ERRORLEVEL%
copy /y "%R%\pockethle.wasm" "%D%" >nul
if errorlevel 1 exit /b %ERRORLEVEL%
copy /y "frontends\pocket-wasm\web\index.html" "%D%" >nul
if errorlevel 1 exit /b %ERRORLEVEL%
copy /y "frontends\pocket-wasm\web\library-ui.js" "%D%" >nul
if errorlevel 1 exit /b %ERRORLEVEL%
xcopy /e /i /y "docs\library" "%D%\library" >nul
if errorlevel 1 exit /b %ERRORLEVEL%
copy /y crates\pocket-library\data\control-guides.json "%D%\library\control-guides.json" >nul
if errorlevel 1 exit /b %ERRORLEVEL%
xcopy /e /i /y "docs\images" "%D%\images" >nul
if errorlevel 1 exit /b %ERRORLEVEL%
xcopy /e /i /y "docs\i18n" "%D%\i18n" >nul
if errorlevel 1 exit /b %ERRORLEVEL%
exit /b 0

@echo off
setlocal
cd /d "%~dp0..\.."
set "UI_REPO=%CD%"
if not exist target\ui-validation mkdir target\ui-validation
set "PATH=%UI_REPO%\target\eq-validation\android-build-tools;C:\Program Files\Git\bin;C:\Program Files\Git\usr\bin;%PATH%"
set "PKG_CONFIG=false"
set "ANDROID_HOME=G:\Android\Sdk"
set "ANDROID_SDK_ROOT=G:\Android\Sdk"
set "ANDROID_NDK_HOME=G:\Android\Sdk\ndk\26.3.11579264"
set "CLANG_PATH=G:\Android\Sdk\ndk\26.3.11579264\toolchains\llvm\prebuilt\windows-x86_64\bin\clang.exe"
set "LIBCLANG_PATH=C:\Program Files\LLVM\bin"
set "BINDGEN_EXTRA_CLANG_ARGS=--sysroot=G:/Android/Sdk/ndk/26.3.11579264/toolchains/llvm/prebuilt/windows-x86_64/sysroot -resource-dir=G:/Android/Sdk/ndk/26.3.11579264/toolchains/llvm/prebuilt/windows-x86_64/lib/clang/17"
set "BINDGEN_EXTRA_CLANG_ARGS_aarch64-linux-android=--sysroot=G:/Android/Sdk/ndk/26.3.11579264/toolchains/llvm/prebuilt/windows-x86_64/sysroot -IG:/Android/Sdk/ndk/26.3.11579264/toolchains/llvm/prebuilt/windows-x86_64/sysroot/usr/include/aarch64-linux-android -resource-dir=G:/Android/Sdk/ndk/26.3.11579264/toolchains/llvm/prebuilt/windows-x86_64/lib/clang/17"
set "BINDGEN_EXTRA_CLANG_ARGS_x86_64-linux-android=--sysroot=G:/Android/Sdk/ndk/26.3.11579264/toolchains/llvm/prebuilt/windows-x86_64/sysroot -IG:/Android/Sdk/ndk/26.3.11579264/toolchains/llvm/prebuilt/windows-x86_64/sysroot/usr/include/x86_64-linux-android -resource-dir=G:/Android/Sdk/ndk/26.3.11579264/toolchains/llvm/prebuilt/windows-x86_64/lib/clang/17"
cargo ndk -t arm64-v8a -t x86_64 -o frontends\pocket-android\app\src\main\jniLibs build --release -p pocket-android-jni --offline > target\ui-validation\android-native-build.log 2>&1
if errorlevel 1 exit /b %ERRORLEVEL%
if not defined JAVA_HOME set "JAVA_HOME=C:\Program Files\Amazon Corretto\jdk21.0.10_7"
if not defined UI_GRADLE set "UI_GRADLE=G:\Android\gradle\gradle-8.7\bin\gradle.bat"
call "%UI_GRADLE%" --offline --no-daemon --project-cache-dir "%UI_REPO%\target\eq-validation\gradle-project-cache" -p frontends\pocket-android :app:assembleDebug > target\ui-validation\android-apk-build.log 2>&1
if errorlevel 1 exit /b %ERRORLEVEL%
copy /y frontends\pocket-android\app\build\outputs\apk\debug\app-debug.apk target\ui-validation\pockethle-ui-unicorn-debug.apk >nul
exit /b %ERRORLEVEL%

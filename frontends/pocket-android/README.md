# PocketHLE — Android frontend

The Android launcher imports Pocket PC `.cab` installers, standalone ARM
`.exe` files, ZIP packages, RAR packages, and Windows self-extracting installers that contain
Pocket PC CABs. It stores games in its private library, renders the live
framebuffer, and forwards touch, keyboard, and D-pad input to the emulator.

## Prerequisites

- Android Studio Iguana (AGP 8.4+) **or** standalone Gradle 8.x with
  `local.properties` pointing at an Android SDK.
- Android NDK r26d (`26.3.11579264`), matching the app's `ndkVersion`.
- `cargo install cargo-ndk` (the cross-compile helper).

## Building the native library

```bash
# From the repo root:
cargo ndk -t arm64-v8a -t x86_64 -o frontends/pocket-android/app/src/main/jniLibs \
    build --release -p pocket-android-jni
```

This drops `libpockethle_jni.so` under
`frontends/pocket-android/app/src/main/jniLibs/<abi>/`.

> **CPU backend on Android.** `pocket-android-jni` enables the real ARM
> `unicorn` backend by default. `--no-default-features` selects the trace-only
> stub and cannot run game code. The JNI build links the NDK's
> `libclang_rt.builtins-<arch>-android.a` to resolve `__clear_cache`; it checks
> both `lib/clang` and `lib64/clang` layouts.

## Building the APK

Inside `frontends/pocket-android`:

```bash
./gradlew assembleDebug
```

> **Note:** No Gradle wrapper jar is committed yet — run
> `gradle wrapper --gradle-version 8.7` once locally to generate it before
> the first build.

## Windows build notes

The verified Windows build uses NDK r26d, Gradle 8.7 and JDK 21.
Set `ANDROID_NDK_HOME` before invoking cargo-ndk. The repository supplies
CMake wrappers for ARM64, ARMv7 and x86_64; the APK above includes ARM64
phones and x86_64 emulators. ARMv7 must be built separately if required.

Unicorn's bundled QEMU configure script also needs a POSIX shell, `sed`
and `strings`. Git for Windows supplies the first two; an executable
`strings` wrapper may delegate to the NDK's `llvm-strings.exe`. Use
`PKG_CONFIG=false` when disabling host package discovery for this bundled
cross-build. Do not modify downloaded Cargo registry sources.

With cargo-ndk on Windows, bindgen may need explicit target-specific
`BINDGEN_EXTRA_CLANG_ARGS_aarch64-linux-android` and
`BINDGEN_EXTRA_CLANG_ARGS_x86_64-linux-android`. Each must name the NDK
`--sysroot`, the corresponding `sysroot/usr/include/<target>` directory,
and the NDK Clang `-resource-dir`. Set `CLANG_PATH` to the actual
`clang.exe` and `LIBCLANG_PATH` to the directory containing libclang.

Run `gradle clean assembleDebug` after changing the stripping toolchain.
The verified debug APK is about 29 MB with both ABIs; an APK near 197 MB
can indicate unstripped libraries or stale ZIP contents. The output is
`app/build/outputs/apk/debug/app-debug.apk`.

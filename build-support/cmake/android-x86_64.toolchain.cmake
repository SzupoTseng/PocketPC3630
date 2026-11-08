# PocketHLE's Android emulator APK includes an x86_64 native library.
# Select that ABI explicitly, as the ARM wrappers do, so cmake-rs and
# Unicorn use the same NDK target as cargo-ndk.
set(ANDROID_ABI x86_64)
if(NOT DEFINED ANDROID_PLATFORM)
    set(ANDROID_PLATFORM android-21)
endif()
if(DEFINED ENV{ANDROID_NDK_HOME})
    set(_pockethle_ndk_path "$ENV{ANDROID_NDK_HOME}")
elseif(DEFINED ENV{ANDROID_NDK_ROOT})
    set(_pockethle_ndk_path "$ENV{ANDROID_NDK_ROOT}")
else()
    message(FATAL_ERROR "Set ANDROID_NDK_HOME or ANDROID_NDK_ROOT before building Android.")
endif()
include("${_pockethle_ndk_path}/build/cmake/android.toolchain.cmake")

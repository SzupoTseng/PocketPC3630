plugins {
    id("com.android.application")
    id("org.jetbrains.kotlin.android")
}

android {
    namespace = "com.pockethle.app"
    compileSdk = 34
    // Match the r26d native build in CI so AGP can strip the EQ emulator libraries.
    ndkVersion = "26.3.11579264"

    defaultConfig {
        applicationId = "com.pockethle.app"
        minSdk = 24
        targetSdk = 34
        versionCode = 7
        versionName = "0.6.0"

        ndk {
            abiFilters += listOf("arm64-v8a", "x86_64")
        }
    }

    // Every published APK is signed with one fixed key, so a new version
    // installs over the old one. The key never enters the repository: the
    // release workflow and local release builds pass it in through these
    // environment variables, and a plain developer build keeps Gradle's
    // own debug key.
    val keystore = System.getenv("POCKETPC_KEYSTORE")
    signingConfigs {
        if (keystore != null) {
            create("published") {
                storeFile = file(keystore)
                storePassword = System.getenv("POCKETPC_KEYSTORE_PASSWORD")
                keyAlias = System.getenv("POCKETPC_KEY_ALIAS") ?: "pocketpc3630"
                keyPassword = System.getenv("POCKETPC_KEYSTORE_PASSWORD")
            }
        }
    }

    buildTypes {
        release {
            isMinifyEnabled = false
            if (keystore != null) signingConfig = signingConfigs.getByName("published")
        }
        debug {
            if (keystore != null) signingConfig = signingConfigs.getByName("published")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions {
        jvmTarget = "17"
    }
}

dependencies {
    implementation("androidx.core:core-ktx:1.13.1")
    implementation("androidx.appcompat:appcompat:1.7.0")
    implementation("androidx.activity:activity-ktx:1.9.0")
    implementation("androidx.fragment:fragment-ktx:1.7.1")
    implementation("androidx.recyclerview:recyclerview:1.3.2")
    implementation("androidx.preference:preference-ktx:1.2.1")
    implementation("androidx.constraintlayout:constraintlayout:2.1.4")
    implementation("com.google.android.material:material:1.12.0")
    implementation("org.jetbrains.kotlinx:kotlinx-coroutines-android:1.8.0")
    implementation("org.json:json:20240303")
}

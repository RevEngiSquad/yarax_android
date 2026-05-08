plugins {
    id("com.android.library")
}

android {
    namespace = "org.revengi.abhi.yarax"
    compileSdk = 35

    defaultConfig {
        minSdk = 21
        consumerProguardFiles("proguard-rules.pro")
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_11
        targetCompatibility = JavaVersion.VERSION_11
    }

    sourceSets {
        getByName("main") {
            jniLibs.srcDirs("jniLibs")
        }
    }
}

// Requires: cargo-ndk (`cargo install cargo-ndk`) + Android NDK.
// Usage:  ./gradlew :yarax_android:cargoNdkBuild
//
// This is NOT required for every build — the pre-built libs in jniLibs/
// are already packaged into the AAR. Run this only when the Rust code changes.
tasks.register<Exec>("cargoNdkBuild") {
    group = "rust"
    description = "Cross-compile Rust JNI lib for Android via cargo-ndk"
    workingDir = projectDir
    commandLine(
        "cargo", "ndk",
        "--target", "arm64-v8a",
        "--target", "armeabi-v7a",
        "--output-dir", "jniLibs",
        "build", "--release"
    )
}

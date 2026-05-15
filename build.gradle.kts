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
tasks.register<Exec>("cargoNdkBuild") {
    group = "rust"
    description = "Cross-compile Rust JNI lib for Android via cargo-ndk"

    inputs.files(
        fileTree("${projectDir}/src") { include("**/*.rs") },
        file("${projectDir}/Cargo.toml"),
        file("${projectDir}/Cargo.lock"),
        file("${projectDir}/build.rs")
    )
    outputs.dir("${projectDir}/jniLibs")

    workingDir = projectDir
    commandLine(
        "cargo", "ndk",
        "--target", "arm64-v8a",
        "--target", "armeabi-v7a",
        "--target", "x86_64",
        "--output-dir", "jniLibs",
        "build", "--release"
    )
}

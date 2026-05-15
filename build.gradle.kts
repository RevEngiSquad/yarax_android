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

val ndkTargets = (project.properties["yarax.ndk.targets"] as? String)?.split(",")?.map { it.trim() }
    ?: (project.properties["target-platform"] as? String)?.split(",")?.mapNotNull {
        when (it.trim()) {
            "android-arm" -> "armeabi-v7a"
            "android-arm64" -> "arm64-v8a"
            "android-x64" -> "x86_64"
            else -> null
        }
    }
    ?: listOf("arm64-v8a", "armeabi-v7a", "x86_64")

val cargoNdkBuild by tasks.registering(Exec::class) {
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
        listOf("cargo", "ndk") +
            ndkTargets.flatMap { listOf("--target", it) } +
            listOf("--output-dir", "jniLibs", "build", "--release")
    )

    doFirst {
        file("${projectDir}/jniLibs").deleteRecursively()
    }
}

tasks.matching { it.name.startsWith("merge") && it.name.endsWith("JniLibFolders") }.configureEach {
    dependsOn(cargoNdkBuild)
}

use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=yarax_patches/.gitmodules");
    println!("cargo:rerun-if-changed=yarax_patches/yara-x");
    println!("cargo:rerun-if-changed=yarax_patches/Makefile");
    println!("cargo:rerun-if-changed=yarax_patches/patches");

    let root = std::env::var("CARGO_MANIFEST_DIR").unwrap();

    let status = Command::new("git")
        .args(["submodule", "update", "--recursive"])
        .current_dir(&root)
        .status()
        .expect("Failed to run git submodule update --recursive");

    if !status.success() {
        panic!("git submodule update --recursive failed");
    }

    let yarax_patches = std::path::Path::new(&root).join("yarax_patches");
    let yara_x = yarax_patches.join("yara-x");

    let has_changes = Command::new("git")
        .args(["diff", "--quiet"])
        .current_dir(&yara_x)
        .output()
        .map(|o| !o.status.success())
        .unwrap_or(false);

    if has_changes {
        println!("cargo:warning=Skipping yara-x update due to local changes");
    } else {
        let status = Command::new("make")
            .arg("update-yarax")
            .current_dir(&yarax_patches)
            .status()
            .expect("Failed to run make update-yarax");

        if !status.success() {
            panic!("make update-yarax failed");
        }
    }
}

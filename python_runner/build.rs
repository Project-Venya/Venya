use std::{env, path::Path, process::Command};

fn main() {
    // Setup python
    let _out_dir = env::var("OUT_DIR").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    let already_setup = Path::new("resources/python").exists();

    if !already_setup {
        let script = if target_os == "windows" {
            "scripts/setup_python.bat"
        } else {
            "scripts/setup_python.sh"
        };

        println!("cargo:warning=Running setup script for Python...");
        let status = Command::new(script)
            .status()
            .expect("Failed to run setup script");

        if !status.success() {
            panic!("Setup script failed with code {:?}", status.code());
        }
    } else {
        println!("cargo:warning=Python already set up, skipping...");
    }

    // Tell Cargo to re-run if the script changes
    println!("cargo:rerun-if-changed=setup_python.sh");
    println!("cargo:rerun-if-changed=setup_python.bat");
}

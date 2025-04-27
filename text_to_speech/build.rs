use std::{env, fs, path::Path, process::Command};

const ORT_VERSION: &str = "1.21.0";

fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();

    let ort_dir = Path::new("resources/onnxruntime");
    if ort_dir.exists() {
        println!("cargo:warning=ONNX Runtime already set up.");
        return;
    }

    fs::create_dir_all(&ort_dir).unwrap();

    let (filename, url) = match (target_os.as_str(), target_arch.as_str()) {
        ("macos", "aarch64") => (
            format!("onnxruntime-osx-arm64-{}.tgz", ORT_VERSION),
            format!(
                "https://github.com/microsoft/onnxruntime/releases/download/v{}/onnxruntime-osx-arm64-{}.tgz",
                ORT_VERSION, ORT_VERSION
            ),
        ),
        ("macos", "x86_64") => (
            format!("onnxruntime-osx-x86_64-{}.tgz", ORT_VERSION),
            format!(
                "https://github.com/microsoft/onnxruntime/releases/download/v{}/onnxruntime-osx-x86_64-{}.tgz",
                ORT_VERSION, ORT_VERSION
            ),
        ),
        ("linux", "x86_64") => (
            format!("onnxruntime-linux-x64-{}.tgz", ORT_VERSION),
            format!(
                "https://github.com/microsoft/onnxruntime/releases/download/v{}/onnxruntime-linux-x64-{}.tgz",
                ORT_VERSION, ORT_VERSION
            ),
        ),
        ("windows", "x86_64") => (
            format!("onnxruntime-win-x64-{}.zip", ORT_VERSION),
            format!(
                "https://github.com/microsoft/onnxruntime/releases/download/v{}/onnxruntime-win-x64-{}.zip",
                ORT_VERSION, ORT_VERSION
            ),
        ),
        _ => panic!("Unsupported platform: {} {}", target_os, target_arch),
    };

    let archive_path = format!("resources/onnxruntime/{}", filename);

    // Download
    if !Path::new(&archive_path).exists() {
        println!("cargo:warning=Downloading ONNX Runtime from {}", url);
        let status = Command::new("curl")
            .args(&["-L", "-o", &archive_path, &url])
            .status()
            .expect("Failed to download ONNX Runtime");
        if !status.success() {
            panic!("Download failed");
        }
    }

    // Extract
    println!("cargo:warning=Extracting {}", filename);
    if filename.ends_with(".zip") {
        Command::new("unzip")
            .args(&[&archive_path, "-d", "resources/onnxruntime"])
            .status()
            .expect("Failed to unzip ONNX Runtime");
    } else {
        Command::new("tar")
            .args(&["-xzf", &archive_path, "-C", "resources/onnxruntime"])
            .status()
            .expect("Failed to extract ONNX Runtime");
    }

    // Set environment var for runtime use
    let lib_dir = fs::read_dir("resources/onnxruntime")
        .unwrap()
        .find_map(|entry| {
            let entry = entry.unwrap();
            if entry.file_type().unwrap().is_dir() {
                Some(entry.path().join("lib"))
            } else {
                None
            }
        })
        .expect("Couldn't find extracted lib directory");

    // Determine correct binary filename and construct the full path
    let dylib_path = match target_os.as_str() {
        "macos" => lib_dir.join("libonnxruntime.dylib"),
        "linux" => lib_dir.join("libonnxruntime.so"),
        "windows" => lib_dir.join("onnxruntime.dll"),
        _ => panic!("Unsupported OS"),
    };

    println!("cargo:rustc-env=ORT_DYLIB_PATH={}", dylib_path.display());
    println!(
        "cargo:warning=ORT_DYLIB_PATH set to {}",
        dylib_path.display()
    );

    // Watch the script so Cargo rebuilds if it changes
    println!("cargo:rerun-if-changed=build.rs");
}

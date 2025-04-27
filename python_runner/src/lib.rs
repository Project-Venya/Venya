use std::io::Read;
use std::{
    path::{Path, PathBuf},
    process::{Command, Output, Stdio},
};

fn get_os_name() -> Result<String, Box<dyn std::error::Error>> {
    // Detect the OS and architecture
    let os_name = if cfg!(target_os = "macos") {
        if cfg!(target_arch = "aarch64") {
            "macos_arm"
        } else {
            "macos_x86"
        }
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        return Err("Unsupported platform".into());
    };

    Ok(os_name.to_string())
}

pub fn get_python_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
    let base = env!("CARGO_MANIFEST_DIR");

    let os_name = get_os_name()?;

    // Define the expected Python executable path
    let path_string = if cfg!(target_os = "windows") {
        format!("{}/resources/{}/python/bin/python.exe", base, os_name)
    } else {
        format!("{}/resources/{}/python/bin/python3", base, os_name)
    };

    let python_bin_path = Path::new(&path_string);

    // Check if the Python executable exists
    if !python_bin_path.exists() {
        return Err(format!("Python executable not found: {}", python_bin_path.display()).into());
    }

    Ok(python_bin_path.to_path_buf())
}

pub fn run_vosk(wav_file_path: &str) -> Result<Output, Box<dyn std::error::Error>> {
    let python_exec = get_python_path();
    let script_path = concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/vosk_runner.py");

    let output = Command::new(python_exec.unwrap())
        .arg(script_path)
        .arg(wav_file_path)
        .output()
        .expect("Failed to run Python");

    Ok(output)
}

pub fn run_hifigan(mel_path: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let python_exec = get_python_path();
    let os_name = get_os_name()?;

    let checkpoint_path = format!(
        "{}/resources/{}/hifigan/checkpoints/generator_v1",
        env!("CARGO_MANIFEST_DIR"),
        os_name
    );

    let script_path = concat!(env!("CARGO_MANIFEST_DIR"), "/scripts/hifigan/vocoder.py");

    let mut command = Command::new(python_exec.unwrap());

    command
        .arg(script_path)
        .arg("--checkpoint")
        .arg(checkpoint_path)
        .arg("--mel")
        .arg(mel_path)
        .arg("--output")
        .arg(output_path);

    println!("Executing: {:?}", command);

    let mut child = command.stdout(Stdio::piped()).spawn()?;

    let mut stdout = child.stdout.take().unwrap();
    let mut buffer = vec![];

    stdout.read_to_end(&mut buffer)?;

    let exit_status = child.wait()?;
    if !exit_status.success() {
        return Err(format!("Python process failed with status {:?}", exit_status).into());
    }

    Ok(())
}

#[cfg(test)]
mod tests {}

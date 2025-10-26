#![doc = include_str!("../README.md")]
use std::env;
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{Layer, Registry};

pub mod audio_player;
pub mod kokoro;
pub mod text_normalizer;

fn _get_os_name() -> Result<String, Box<dyn std::error::Error>> {
    // Detect the OS and architecture
    let os_name = if cfg!(target_os = "macos") {
        "macos"
    } else if cfg!(target_os = "linux") {
        "linux"
    } else if cfg!(target_os = "windows") {
        "windows"
    } else {
        return Err("Unsupported platform".into());
    };

    Ok(os_name.to_string())
}

/// Convenience function to setup logging for any binaries I create. Automatically sets all
/// binaries and the tts library crate to `info` logging by default.
pub fn setup_logging() {
    let filter = match env::var("RUST_LOG") {
        Ok(_) => EnvFilter::from_env("RUST_LOG"),
        _ => EnvFilter::new("xd_tts=info,app=info,trainer=info"),
    };

    let fmt = tracing_subscriber::fmt::Layer::default();

    let subscriber = filter.and_then(fmt).with_subscriber(Registry::default());

    tracing::subscriber::set_global_default(subscriber).unwrap();
}

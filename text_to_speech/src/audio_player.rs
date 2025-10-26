use rodio::cpal::traits::{DeviceTrait, HostTrait};
use rodio::cpal::Device;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use tokio::task;

/// Custom error types for the AudioPlayer
#[derive(Debug)]
pub enum AudioPlayerError {
    /// Error during audio stream initialization.
    StreamInitError(String),
    /// Error during audio sink initialization.
    SinkInitError(String),
    /// Audio file not found.
    FileNotFound(String),
    /// Could not open audio file.
    FileOpenError(String),
    /// Audio decoding error.
    DecodingError(String),
    /// Audio sink is unavailable.
    SinkUnavailable,
    /// Error acquiring lock.
    LockError(String),
    /// Device error.
    DeviceError(String),
    /// Audio device not found.
    DeviceNotFound(String),
    /// Error listing audio devices.
    DeviceListError(String),
    /// No default audio device available.
    NoDefaultDevice,
}

impl std::fmt::Display for AudioPlayerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            AudioPlayerError::StreamInitError(msg) => {
                write!(f, "Audio stream initialization error: {}", msg)
            }
            AudioPlayerError::SinkInitError(msg) => {
                write!(f, "Audio sink initialization error: {}", msg)
            }
            AudioPlayerError::FileNotFound(path) => write!(f, "Audio file not found: {}", path),
            AudioPlayerError::FileOpenError(msg) => write!(f, "Could not open audio file: {}", msg),
            AudioPlayerError::DecodingError(msg) => write!(f, "Audio decoding error: {}", msg),
            AudioPlayerError::SinkUnavailable => write!(f, "Audio sink is unavailable"),
            AudioPlayerError::LockError(msg) => write!(f, "Lock error: {}", msg),
            AudioPlayerError::DeviceError(msg) => write!(f, "Device error: {}", msg),
            AudioPlayerError::DeviceNotFound(name) => write!(f, "Audio device not found: {}", name),
            AudioPlayerError::DeviceListError(msg) => {
                write!(f, "Error listing audio devices: {}", msg)
            }
            AudioPlayerError::NoDefaultDevice => write!(f, "No default audio device available"),
        }
    }
}

impl std::error::Error for AudioPlayerError {}

/// A simple audio player.
pub struct AudioPlayer {
    /// The audio stream.
    _stream: OutputStream,
    /// Handle to the audio stream.
    stream_handle: OutputStreamHandle,
    /// The audio sink for playback. Wrapped in Arc<Mutex<>> for thread-safe access.
    sink: Arc<Mutex<Option<Sink>>>,
    /// Device info.
    device_info: Device,
}

impl AudioPlayer {
    /// Creates a new `AudioPlayer` instance with the default output device.
    pub fn new() -> Result<Self, AudioPlayerError> {
        let host = rodio::cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or(AudioPlayerError::NoDefaultDevice)?;
        let (_stream, stream_handle) = OutputStream::try_from_device(&device)
            .map_err(|e| AudioPlayerError::StreamInitError(e.to_string()))?;
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| AudioPlayerError::SinkInitError(e.to_string()))?;

        println!(
            "Device name: {}",
            device.name().unwrap_or("Unknown".to_string())
        );
        println!(
            "Supported sample rates: {:?}",
            device
                .supported_output_configs()
                .unwrap()
                .map(|c| c.max_sample_rate())
                .collect::<Vec<_>>()
        );
        println!(
            "Supported channel configs: {:?}",
            device
                .supported_output_configs()
                .unwrap()
                .map(|c| c.channels())
                .collect::<Vec<_>>()
        );

        Ok(AudioPlayer {
            _stream,
            stream_handle,
            sink: Arc::new(Mutex::new(Some(sink))),
            device_info: device,
        })
    }

    /// Creates a new `AudioPlayer` instance with a specified device name.
    pub fn with_device_name(name: &str) -> Result<Self, AudioPlayerError> {
        let host = rodio::cpal::default_host();
        let device = host
            .output_devices()
            .map_err(|e| AudioPlayerError::DeviceListError(e.to_string()))?
            .find(|d| d.name().map_or(false, |n| n == name))
            .ok_or_else(|| AudioPlayerError::DeviceNotFound(name.to_string()))?;

        Self::with_device(device)
    }

    /// Creates a new `AudioPlayer` instance with a specified `Device`.
    pub fn with_device(device: Device) -> Result<Self, AudioPlayerError> {
        let (_stream, stream_handle) = OutputStream::try_from_device(&device)
            .map_err(|e| AudioPlayerError::StreamInitError(e.to_string()))?;
        let sink = Sink::try_new(&stream_handle)
            .map_err(|e| AudioPlayerError::SinkInitError(e.to_string()))?;

        Ok(AudioPlayer {
            _stream,
            stream_handle,
            sink: Arc::new(Mutex::new(Some(sink))),
            device_info: device,
        })
    }

    /// Lists the available audio output devices.
    pub fn list_available_devices() -> Result<(), AudioPlayerError> {
        let host = rodio::cpal::default_host();
        let devices = host
            .output_devices()
            .map_err(|e| AudioPlayerError::DeviceListError(e.to_string()))?;
        for device in devices {
            println!(
                "Available device: {}",
                device.name().unwrap_or_else(|_| "Unknown".to_string())
            );
        }
        Ok(())
    }

    /// Gets the device info, such as the device name.
    pub fn get_device_info(&self) -> Result<String, AudioPlayerError> {
        self.device_info
            .name()
            .map_err(|e| AudioPlayerError::DeviceError(e.to_string()))
    }

    pub fn get_output_streams(&self) -> OutputStreamHandle {
        self.stream_handle.clone()
    }

    /// Sets the volume of the audio playback.
    pub fn set_volume(&self, volume: f32) {
        if let Some(ref sink) = *self.sink.lock().unwrap() {
            sink.set_volume(volume);
        }
    }
    /// Plays the audio file.
    pub fn play(&self, path: &str) -> Result<(), AudioPlayerError> {
        println!(
            "Attempting to play audio file: {} on device: {}",
            path,
            self.get_device_info()?
        );

        if !std::path::Path::new(&path).exists() {
            return Err(AudioPlayerError::FileNotFound(path.to_string().clone()));
        }

        // Read WAV file metadata using the hound crate
        let reader = hound::WavReader::open(&path)
            .map_err(|e| AudioPlayerError::FileOpenError(e.to_string()))?;
        let spec = reader.spec();

        println!("Sample rate: {} Hz", spec.sample_rate);
        println!("Channels: {}", spec.channels);
        println!("Bits per sample: {}", spec.bits_per_sample);

        let file = File::open(&path).map_err(|e| AudioPlayerError::FileOpenError(e.to_string()))?;
        let source = Decoder::new(BufReader::new(file))
            .map_err(|e| AudioPlayerError::DecodingError(e.to_string()))?;

        let sink_clone = Arc::clone(&self.sink);
        task::spawn(async move {
            let mut sink_guard = sink_clone.lock().unwrap();
            if let Some(ref mut sink) = *sink_guard {
                sink.set_volume(1.0);
                sink.append(source);

                println!("Sink is empty before sleep: {}", sink.empty());

                std::thread::sleep(std::time::Duration::from_millis(100));

                println!("Sink is empty after sleep: {}", sink.empty());

                sink.play();

                sink.sleep_until_end();
            }
        });
        Ok(())
    }

    /// Pauses the audio playback.
    pub fn pause(&self) {
        if let Some(ref sink) = *self.sink.lock().unwrap() {
            sink.pause();
            println!("Playback paused");
        }
    }

    /// Resumes the audio playback.
    pub fn resume(&self) {
        if let Some(ref sink) = *self.sink.lock().unwrap() {
            sink.play();
            println!("Playback resumed");
        }
    }

    /// Stops the audio playback.
    pub fn stop(&self) {
        if let Some(ref sink) = *self.sink.lock().unwrap() {
            sink.stop();
            println!("Playback stopped");
        }
    }

    /// Waits until the end of the audio playback.
    pub fn wait_until_end(&self) {
        if let Some(ref sink) = *self.sink.lock().unwrap() {
            sink.sleep_until_end();
            println!("Audio playback completed");
        }
    }

    /// Checks if the audio is currently playing.
    pub fn is_playing(&self) -> bool {
        if let Some(ref sink) = *self.sink.lock().unwrap() {
            !sink.empty()
        } else {
            false
        }
    }
}

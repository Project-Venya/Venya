use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, Sample};
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use rubato::{FftFixedInOut, Resampler};
use std::env;
use std::path::PathBuf;
use std::process::Command;
use std::{
    fs::File,
    io::BufWriter,
    sync::{Arc, Mutex},
};

/// Struct for recording audio from the default input device and saving it to a WAV file.
pub struct AudioRecorder {
    stream: Option<cpal::Stream>,
    writer: WavWriterHandle,
    manifest_dir: String,
}

/// Struct representing a segment of transcribed audio.
#[derive(Debug, Clone)]
pub struct TranscriptSegment {
    pub start_time: String,
    pub end_time: String,
    pub content: String,
}

type WavWriterHandle = Arc<Mutex<Option<hound::WavWriter<BufWriter<File>>>>>;

impl AudioRecorder {
    /// Creates a new `AudioRecorder`.
    pub fn new(cargo_manifest_dir: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let writer = Arc::new(Mutex::new(None));

        Ok(AudioRecorder {
            stream: None,
            writer,
            manifest_dir: cargo_manifest_dir.to_string(),
        })
    }

    /// Starts recording audio from the default input device.
    pub fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let host = cpal::default_host();
        let device = host
            .default_input_device()
            .expect("no input device available");
        let config = device
            .default_input_config()
            .expect("Failed to get default input config");

        let path = format!("{}/assets/audios/recorded.wav", self.manifest_dir);
        let spec = self.wav_spec_from_config(&config);
        let writer = hound::WavWriter::create(path.as_str(), spec)?;
        *self.writer.lock().unwrap() = Some(writer);

        let writer_2 = self.writer.clone();

        let err_fn = move |err| {
            eprintln!("an error occurred on stream: {}", err);
        };

        let stream = match config.sample_format() {
            cpal::SampleFormat::I8 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i8, i8>(data, &writer_2),
                err_fn,
                None,
            )?,
            cpal::SampleFormat::I16 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i16, i16>(data, &writer_2),
                err_fn,
                None,
            )?,
            cpal::SampleFormat::I32 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<i32, i32>(data, &writer_2),
                err_fn,
                None,
            )?,
            cpal::SampleFormat::F32 => device.build_input_stream(
                &config.into(),
                move |data, _: &_| write_input_data::<f32, f32>(data, &writer_2),
                err_fn,
                None,
            )?,
            sample_format => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Unsupported sample format '{sample_format}'"),
                )));
            }
        };

        stream.play()?;
        self.stream = Some(stream);
        println!("Recording started");
        Ok(())
    }

    /// Stops the recording, finalizes the WAV file, and resamples the audio to mono 16kHz.
    pub fn stop(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(stream) = self.stream.take() {
            drop(stream);
            self.writer.lock().unwrap().take().unwrap().finalize()?;
            println!("Recording complete!");
            println!("  ");
            println!("Resampling audio file...");

            let input_path = format!("{}/assets/audios/recorded.wav", self.manifest_dir);
            let output_path = format!("{}/assets/audios/resampled.wav", self.manifest_dir);

            self.resample_wav_to_mono_16khz(input_path.as_str(), output_path.as_str());

            println!("  ");
            println!("Done Resampling audio file...");
        }
        Ok(())
    }

    fn sample_format(&self, format: cpal::SampleFormat) -> hound::SampleFormat {
        if format.is_float() {
            hound::SampleFormat::Float
        } else {
            hound::SampleFormat::Int
        }
    }

    fn wav_spec_from_config(&self, config: &cpal::SupportedStreamConfig) -> hound::WavSpec {
        hound::WavSpec {
            channels: config.channels() as _,
            sample_rate: config.sample_rate().0 as _,
            bits_per_sample: (config.sample_format().sample_size() * 8) as _,
            sample_format: self.sample_format(config.sample_format()),
        }
    }

    fn resample_wav_to_mono_16khz(&self, input_path: &str, output_path: &str) {
        let reader = WavReader::open(input_path).expect("Cannot open input WAV");
        let spec = reader.spec();

        if spec.channels != 1 {
            panic!("Expected mono audio");
        }

        let samples: Vec<f32> = reader
            .into_samples::<f32>()
            .map(|s| s.unwrap_or(0.0))
            .collect();

        let input_sample_rate = spec.sample_rate as usize;
        let target_sample_rate = 16000;
        let chunk_size = 1024;

        let mut resampler = FftFixedInOut::<f32>::new(
            input_sample_rate,
            target_sample_rate,
            chunk_size,
            1, // mono
        )
        .expect("Failed to create resampler");

        let required_chunk_size = resampler.input_frames_next();

        // Split input into chunks
        let input_chunks: Vec<Vec<f32>> = samples
            .chunks(required_chunk_size)
            .map(|chunk| {
                // pad with zeros if too short (especially for the last chunk)
                let mut padded = chunk.to_vec();
                padded.resize(required_chunk_size, 0.0);
                padded
            })
            .collect();

        // Resample each chunk
        let mut resampled_samples = Vec::new();
        for chunk in input_chunks {
            let input_per_channel = vec![chunk];
            let output = resampler
                .process(&input_per_channel, None)
                .expect("Resample failed");

            resampled_samples.extend_from_slice(&output[0]);
        }

        // Convert f32 to i16
        let i16_samples: Vec<i16> = resampled_samples
            .iter()
            .map(|s| (*s * i16::MAX as f32).clamp(i16::MIN as f32, i16::MAX as f32) as i16)
            .collect();

        let out_spec = WavSpec {
            channels: 1,
            sample_rate: target_sample_rate as u32,
            bits_per_sample: 16,
            sample_format: SampleFormat::Int,
        };

        let mut writer = WavWriter::create(output_path, out_spec).expect("Failed to write WAV");
        for sample in i16_samples {
            writer.write_sample(sample).unwrap();
        }
        writer.finalize().unwrap();
    }

    /// Runs Whisper on an audio file and returns a vector of transcript segments.
    pub fn convert_speech_to_text_whisper(&self) -> Result<Vec<TranscriptSegment>, String> {
        let audio_path = format!("{}/assets/audios/resampled.wav", self.manifest_dir);
        println!("wav file path - {}", audio_path);

        let base = env!("CARGO_MANIFEST_DIR");

        let whisper_bin =
            PathBuf::from(format!("{}/resources/whisper/build/bin/whisper-cli", base));
        let model_path = PathBuf::from(format!("{}/resources/models/ggml-small.en.bin", base));

        let output = Command::new(whisper_bin)
            .args(&[
                "-m",
                model_path.to_str().unwrap(),
                "-f",
                audio_path.as_str(),
            ])
            .output()
            .map_err(|e| format!("Failed to run whisper: {}", e))?;

        let mut segments = Vec::new();

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout).to_string();

            for line in output_str.lines() {
                if line.starts_with("[") {
                    if let Some(end_bracket) = line.find("]") {
                        let times = &line[1..end_bracket];
                        if let Some((start, end)) = times.split_once("-->") {
                            let content = line[end_bracket + 1..].trim().to_string();
                            segments.push(TranscriptSegment {
                                start_time: start.trim().to_string(),
                                end_time: end.trim().to_string(),
                                content,
                            });
                        }
                    }
                }
            }
        }

        Ok(segments)
    }
}

fn write_input_data<T, U>(input: &[T], writer: &WavWriterHandle)
where
    T: Sample,
    U: Sample + hound::Sample + FromSample<T>,
{
    if let Ok(mut guard) = writer.try_lock() {
        if let Some(writer) = guard.as_mut() {
            for &sample in input.iter() {
                let sample: U = U::from_sample(sample);
                writer.write_sample(sample).ok();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_audio_recorder_new() {
        let manifest_dir = "."; // Use current directory for test
        let recorder = AudioRecorder::new(manifest_dir).unwrap();
        assert_eq!(recorder.manifest_dir, manifest_dir);
        assert!(recorder.stream.is_none());
    }

    #[test]
    #[ignore] // This test requires audio input and proper setup, so it's ignored by default
    fn test_audio_recorder_start_stop() {
        let manifest_dir = "."; // Use current directory for test
        let mut recorder = AudioRecorder::new(manifest_dir).unwrap();

        // Define paths
        let recorded_path = format!("{}/assets/audios/recorded.wav", manifest_dir);
        let resampled_path = format!("{}/assets/audios/resampled.wav", manifest_dir);

        // Ensure the assets/audios directory exists
        fs::create_dir_all("./assets/audios").unwrap();

        // Clean up any previous test runs
        fs::remove_file(&recorded_path).ok();
        fs::remove_file(&resampled_path).ok();

        // Start recording
        let start_result = recorder.start();
        assert!(start_result.is_ok());

        // Record for a short time (e.g., 2 seconds)
        std::thread::sleep(std::time::Duration::from_secs(2));

        // Stop recording
        let stop_result = recorder.stop();
        assert!(stop_result.is_ok());

        // Check if the recorded and resampled files were created
        assert!(fs::metadata(&recorded_path).is_ok());
        assert!(fs::metadata(&resampled_path).is_ok());

        // Clean up the created files
        fs::remove_file(&recorded_path).unwrap();
        fs::remove_file(&resampled_path).unwrap();
    }
}

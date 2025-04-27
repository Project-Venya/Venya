#![doc = include_str!("../README.md")]
use crate::phonemes::Unit;
use crate::tacotron2::*;
use crate::text_normaliser::NormaliserChunk;
use hifigan::HiFiGan;
// use griffin_lim::GriffinLim;
use hound::{SampleFormat, WavSpec, WavWriter};
use std::env;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use tracing::{debug, error, info};
use tracing_subscriber::filter::EnvFilter;
use tracing_subscriber::{Layer, Registry};

pub mod cmu_dict;
pub mod phonemes;
// This failed for various reasons. Look in the module so see the pains of ML.
//pub mod speedyspeech;
pub mod hifigan;
pub mod kokoro;
pub mod tacotron2;
pub mod text_normaliser;
pub mod training;

pub use cmu_dict::CmuDictionary;

pub const WAV_SPEC: WavSpec = WavSpec {
    channels: 1,
    sample_rate: 22050,
    bits_per_sample: 16,
    sample_format: SampleFormat::Int,
};

pub struct XdTts {
    dict: CmuDictionary,
    model: Tacotron2,
    vocoder: HiFiGan,
    phoneme_input: bool,
}

fn get_os_name() -> Result<String, Box<dyn std::error::Error>> {
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

fn set_ort_path() {
    // Assuming extracted ONNX Runtime is in resources/onnxruntime/onnxruntime-*/lib
    let mut ort_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    ort_path.push("resources/onnxruntime");

    let target_os = get_os_name().unwrap();

    // Find the actual lib directory
    let lib_path = std::fs::read_dir(&ort_path)
        .unwrap()
        .find_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_dir() && path.join("lib").exists() {
                Some(path.join("lib"))
            } else {
                None
            }
        })
        .expect("ONNX Runtime lib directory not found");

    // Determine correct binary filename and construct the full path
    let dylib_path = match target_os.as_str() {
        "macos" => lib_path.join("libonnxruntime.dylib"),
        "linux" => lib_path.join("libonnxruntime.so"),
        "windows" => lib_path.join("onnxruntime.dll"),
        _ => panic!("Unsupported OS"),
    };

    unsafe {
        env::set_var("ORT_DYLIB_PATH", &dylib_path);
    }
    println!("ORT_DYLIB_PATH = {}", dylib_path.display());
}

impl XdTts {
    pub fn new(phoneme_input: bool) -> anyhow::Result<Self> {
        // Set the ORT_DYLIB_PATH environment variable to the path of the ONNX Runtime library.
        set_ort_path();

        let cargo_path = env!("CARGO_MANIFEST_DIR");
        let dict = if phoneme_input {
            let mut dict = CmuDictionary::open(format!("{}/data/cmudict-0.7b.txt", cargo_path))?;
            if let Ok(custom) =
                CmuDictionary::open(format!("{}/resources/custom_dict.txt", cargo_path))
            {
                dict.merge(custom);
            }
            dict
        } else {
            CmuDictionary::default()
        };

        let model = Tacotron2::load(format!("{}/models/tacotron2", cargo_path))?;
        let vocoder = HiFiGan::new();
        Ok(Self {
            dict,
            model,
            vocoder,
            phoneme_input,
        })
    }

    pub fn generate_audio(
        &self,
        text: &str,
        output_path: &str,
        output_spectrogram: Option<PathBuf>,
    ) -> anyhow::Result<()> {
        let start = Instant::now();
        info!("Text normalisation");
        let mut text = text_normaliser::normalise(text)?;
        if self.phoneme_input {
            // Sad tacotron2 was trained with ARPA support
            text.words_to_pronunciation(&self.dict);
        } else {
            text.convert_to_units();
        }
        let mut inference_chunk = vec![];

        let text_end = Instant::now();
        info!("Text processing time: {:?}", text_end - start);
        info!("Generating audio");
        for chunk in text.drain_all() {
            debug!("Chunk: {:?}", chunk);
            match chunk {
                NormaliserChunk::Pronunciation(mut units) => inference_chunk.append(&mut units),
                NormaliserChunk::Break(_duration) => {
                    // Infer here.
                    // Potentially we could use the alignments in the network output and return them
                    // with the spectrogram to insert this stuff. That might be better - it depends if
                    // coarticulation sounds more or less natural when a giant pause is inserted.
                    self.infer(&inference_chunk, output_spectrogram.as_ref(), output_path)?;
                    inference_chunk.clear();
                }
                NormaliserChunk::Text(t) => {
                    unreachable!("'{}' Should have been converted to pronunciation", t)
                }
                NormaliserChunk::Punct(p) => {
                    inference_chunk.push(Unit::Punct(p));
                }
            }
        }
        self.infer(
            &inference_chunk,
            output_spectrogram.as_ref(),
            output_path.as_ref(),
        )?;
        let end = Instant::now();
        info!("Finished processing in: {:?}", end - start);
        Ok(())
    }

    fn infer(
        &self,
        input: &[Unit],
        output_spectrogram: Option<&PathBuf>,
        output_path: &str,
    ) -> anyhow::Result<()> {
        if input.is_empty() {
            return Ok(());
        }
        let mel_gen_start = Instant::now();
        let spectrogram = self.model.infer(input)?;

        if let Some(output_spectrogram) = output_spectrogram {
            if let Err(e) = ndarray_npy::write_npy(output_spectrogram, &spectrogram) {
                error!(
                    "Failed to write spectrogram to '{}': {}",
                    output_spectrogram.display(),
                    e
                );
            }
        }
        let vocoder_start = Instant::now();
        let _ = self
            .vocoder
            .infer(
                output_spectrogram
                    .map(|p| p.to_str().expect("Spectrogram path not valid unicode"))
                    .unwrap_or(""),
                output_path,
            )
            .unwrap();

        let end = Instant::now();

        info!("Mel gen time: {:?}", vocoder_start - mel_gen_start);
        info!("Vocoder time: {:?}", end - vocoder_start);

        Ok(())
    }
}

fn _write_silence(
    duration: Duration,
    wav_writer: &mut WavWriter<BufWriter<File>>,
) -> anyhow::Result<()> {
    let n_samples = (wav_writer.spec().sample_rate as f32 * duration.as_secs_f32()).round() as u32;

    if n_samples > 0 {
        let mut i16_writer = wav_writer.get_i16_writer(n_samples);
        for _ in 0..n_samples {
            i16_writer.write_sample(0);
        }
        i16_writer.flush()?;
    }
    Ok(())
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

use ndarray::Array1;
use python_runner::run_hifigan;

pub struct HiFiGan {
    // Fields and methods go here
}

impl HiFiGan {
    pub fn new() -> Self {
        Self {}
    }

    pub fn infer(
        &self,
        spectrogram_path: &str,
        output_path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let _ = run_hifigan(spectrogram_path, output_path)?;
        Ok(())
    }
}

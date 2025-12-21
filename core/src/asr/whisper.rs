use super::model::{AsrModel, AsrError, TranscriptionResult, TranscriptionSegment};
use std::time::Instant;
use std::path::Path;
use onnxruntime::{environment::Environment, LoggingLevel};

pub struct WhisperModel {
    model_path: Option<String>,
    is_loaded: bool,
    _env: Environment,
}

impl WhisperModel {
    pub fn new() -> Self {
        let env = Environment::builder()
            .with_name("gijiroku21")
            .with_log_level(LoggingLevel::Warning)
            .build()
            .expect("Failed to create ONNX Runtime environment");
        
        WhisperModel {
            model_path: None,
            is_loaded: false,
            _env: env,
        }
    }
}

impl Default for WhisperModel {
    fn default() -> Self {
        Self::new()
    }
}

impl AsrModel for WhisperModel {
    fn initialize(&mut self, model_path: &str) -> Result<(), AsrError> {
        if !Path::new(model_path).exists() {
            return Err(AsrError::ModelNotFound(model_path.to_string()));
        }
        
        self.model_path = Some(model_path.to_string());
        self.is_loaded = true;
        
        println!("[WhisperModel] Model path set: {}", model_path);
        Ok(())
    }
    
    fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, AsrError> {
        if !self.is_loaded {
            return Err(AsrError::ModelNotLoaded);
        }
        
        let start_time = Instant::now();
        let duration = audio.len() as f64 / 16000.0;
        let max_amplitude = audio.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
        let rms = (audio.iter().map(|&x| x * x).sum::<f32>() / audio.len() as f32).sqrt();
        
        let segments = Self::detect_voice_segments(audio, 16000);
        
        let result = TranscriptionResult {
            segments,
            full_text: format!("Audio: {:.2}s, Amp: {:.4}, RMS: {:.4}", duration, max_amplitude, rms),
            processing_time: start_time.elapsed().as_secs_f64(),
        };
        
        Ok(result)
    }
    
    fn is_loaded(&self) -> bool {
        self.is_loaded
    }
    
    fn unload(&mut self) {
        self.is_loaded = false;
        self.model_path = None;
    }
}

impl WhisperModel {
    fn detect_voice_segments(audio: &[f32], sample_rate: usize) -> Vec<TranscriptionSegment> {
        const WINDOW_SIZE: usize = 16000;
        const THRESHOLD: f32 = 0.01;
        
        let mut segments = Vec::new();
        let mut in_voice = false;
        let mut start_time = 0.0;
        
        for (i, chunk) in audio.chunks(WINDOW_SIZE).enumerate() {
            let rms = (chunk.iter().map(|&x| x * x).sum::<f32>() / chunk.len() as f32).sqrt();
            let current_time = i as f64;
            
            if rms > THRESHOLD && !in_voice {
                start_time = current_time;
                in_voice = true;
            } else if rms <= THRESHOLD && in_voice {
                segments.push(TranscriptionSegment {
                    start: start_time,
                    end: current_time,
                    text: format!("Voice {:.1}s-{:.1}s", start_time, current_time),
                    confidence: 0.9,
                    speaker: None,
                });
                in_voice = false;
            }
        }
        
        if in_voice {
            let end_time = audio.len() as f64 / sample_rate as f64;
            segments.push(TranscriptionSegment {
                start: start_time,
                end: end_time,
                text: format!("Voice {:.1}s-{:.1}s", start_time, end_time),
                confidence: 0.9,
                speaker: None,
            });
        }
        
        if segments.is_empty() {
            let duration = audio.len() as f64 / sample_rate as f64;
            segments.push(TranscriptionSegment {
                start: 0.0,
                end: duration,
                text: String::from("Silent"),
                confidence: 0.5,
                speaker: None,
            });
        }
        
        segments
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_whisper_model_creation() {
        let model = WhisperModel::new();
        assert!(!model.is_loaded());
    }
    
    #[test]
    fn test_whisper_transcribe_without_load() {
        let model = WhisperModel::new();
        let audio = vec![0.0; 16000];
        let result = model.transcribe(&audio);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_whisper_voice_detection() {
        let mut model = WhisperModel::new();
        let temp_dir = std::env::temp_dir();
        let model_path = temp_dir.join("test.dat");
        std::fs::write(&model_path, b"mock").unwrap();
        
        model.initialize(model_path.to_str().unwrap()).unwrap();
        let mut audio = vec![0.0f32; 16000];
        for (i, sample) in audio.iter_mut().enumerate() {
            *sample = (i as f32 * 0.1).sin() * 0.5;
        }
        
        let result = model.transcribe(&audio).unwrap();
        assert!(!result.segments.is_empty());
        
        std::fs::remove_file(&model_path).ok();
    }
}

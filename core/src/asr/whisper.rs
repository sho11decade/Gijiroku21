use super::model::{AsrModel, AsrError, TranscriptionResult, TranscriptionSegment};
use std::path::Path;
use tokenizers::Tokenizer;

pub struct WhisperModel {
    model_path: Option<String>,
    encoder_path: Option<String>,
    decoder_path: Option<String>,
    tokenizer_path: Option<String>,
    is_loaded: bool,
    tokenizer: Option<Tokenizer>,
}

impl WhisperModel {
    pub fn new() -> Self {
        WhisperModel {
            model_path: None,
            encoder_path: None,
            decoder_path: None,
            tokenizer_path: None,
            is_loaded: false,
            tokenizer: None,
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
        // model_path はディレクトリもしくは単一ONNXを想定する
        let p = Path::new(model_path);
        let (enc, dec) = if p.is_dir() {
            let enc = p.join("encoder_model.onnx");
            let dec = p.join("decoder_model.onnx");
            (enc, dec)
        } else {
            // 単一ファイルの場合は encoder/decoder と同パスを指す（単一ONNX対応）
            (p.to_path_buf(), p.to_path_buf())
        };

        if !enc.exists() {
            return Err(AsrError::ModelNotFound(enc.display().to_string()));
        }
        if !dec.exists() {
            return Err(AsrError::ModelNotFound(dec.display().to_string()));
        }

        // tokenizer は model_path と同階層の tokenizer/tokenizer.json を優先
        let tok_path = if p.is_dir() {
            p.parent()
                .unwrap_or_else(|| Path::new("."))
                .join("tokenizer")
                .join("tokenizer.json")
        } else {
            Path::new(model_path)
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .join("tokenizer")
                .join("tokenizer.json")
        };

        if !tok_path.exists() {
            return Err(AsrError::ModelNotFound(tok_path.display().to_string()));
        }

        let tokenizer = Tokenizer::from_file(tok_path.to_string_lossy().to_string())
            .map_err(|e| AsrError::InferenceFailed(format!("Tokenizer load error: {e}")))?;

        self.model_path = Some(model_path.to_string());
        self.encoder_path = Some(enc.display().to_string());
        self.decoder_path = Some(dec.display().to_string());
        self.tokenizer_path = Some(tok_path.display().to_string());
        self.tokenizer = Some(tokenizer);
        self.is_loaded = true;
        
        println!("[WhisperModel] encoder: {}, decoder: {}", self.encoder_path.as_deref().unwrap_or(""), self.decoder_path.as_deref().unwrap_or(""));
        Ok(())
    }
    
    fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, AsrError> {
        if !self.is_loaded {
            return Err(AsrError::ModelNotLoaded);
        }

        let duration = audio.len() as f64 / 16000.0;
        let segment = TranscriptionSegment {
            start: 0.0,
            end: duration,
            text: "(ASR not implemented yet)".to_string(),
            confidence: 0.0,
            speaker: None,
        };

        Ok(TranscriptionResult {
            segments: vec![segment],
            full_text: "(ASR not implemented yet)".to_string(),
            processing_time: 0.0,
        })
    }
    
    fn is_loaded(&self) -> bool {
        self.is_loaded
    }
    
    fn unload(&mut self) {
        self.is_loaded = false;
        self.model_path = None;
        self.encoder_path = None;
        self.decoder_path = None;
        self.tokenizer_path = None;
        self.tokenizer = None;
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
    fn test_initialize_missing_model() {
        let mut model = WhisperModel::new();
        let result = model.initialize("./no_such_model.onnx");
        assert!(result.is_err());
    }
}

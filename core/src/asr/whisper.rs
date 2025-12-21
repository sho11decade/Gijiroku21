use super::model::{AsrModel, AsrError, TranscriptionResult, TranscriptionSegment};
use super::onnx_runtime::ONNX_ENV;
use crate::audio::{log_mel_spectrogram, MelConfig};
use ndarray::Array;
use onnxruntime::{GraphOptimizationLevel, ndarray::IxDyn, tensor::OrtOwnedTensor};
use std::path::Path;
use std::time::Instant;
use tokenizers::Tokenizer;

pub struct WhisperModel {
    model_path: Option<String>,
    encoder_path: Option<String>,
    decoder_path: Option<String>,
    tokenizer_path: Option<String>,
    is_loaded: bool,
    tokenizer: Option<Tokenizer>,
    bos_id: Option<u32>,
    lang_id: Option<u32>,
    task_id: Option<u32>,
    eos_id: Option<u32>,
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
            bos_id: None,
            lang_id: None,
            task_id: None,
            eos_id: None,
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

        // Whisper 用に必要となる特殊トークンを事前に解決しておく
        // いずれかが存在しない場合は、その tokenizer.json は Whisper 用としては利用不可とみなす
        let bos_id = tokenizer
            .token_to_id("<|startoftranscript|>")
            .ok_or_else(|| AsrError::InferenceFailed("Tokenizer is missing <|startoftranscript|> token".into()))?;
        let lang_id = tokenizer
            .token_to_id("<|ja|>")
            .ok_or_else(|| AsrError::InferenceFailed("Tokenizer is missing <|ja|> token".into()))?;
        let task_id = tokenizer
            .token_to_id("<|transcribe|>")
            .ok_or_else(|| AsrError::InferenceFailed("Tokenizer is missing <|transcribe|> token".into()))?;
        let eos_id = tokenizer
            .token_to_id("<|endoftext|>")
            .ok_or_else(|| AsrError::InferenceFailed("Tokenizer is missing <|endoftext|> token".into()))?;

        self.model_path = Some(model_path.to_string());
        self.encoder_path = Some(enc.display().to_string());
        self.decoder_path = Some(dec.display().to_string());
        self.tokenizer_path = Some(tok_path.display().to_string());
        self.tokenizer = Some(tokenizer);
        self.bos_id = Some(bos_id);
        self.lang_id = Some(lang_id);
        self.task_id = Some(task_id);
        self.eos_id = Some(eos_id);
        self.is_loaded = true;
        
        println!("[WhisperModel] encoder: {}, decoder: {}", self.encoder_path.as_deref().unwrap_or(""), self.decoder_path.as_deref().unwrap_or(""));
        Ok(())
    }
    
    fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, AsrError> {
        if !self.is_loaded {
            return Err(AsrError::ModelNotLoaded);
        }

        let start_time = Instant::now();
        
        // ステップ1: メルスペクトログラム生成（Encoder入力）
        let mel = log_mel_spectrogram(audio, &MelConfig::default());
        let mel_array = Array::from_shape_vec((1, 80, 3000), mel)
            .map_err(|e| AsrError::InferenceFailed(format!("Mel reshape: {e}")))?;

        // ステップ2: ONNX Runtime 環境を取得（LazyStatic で初期化済み）
        let env_guard = ONNX_ENV.lock()
            .map_err(|e| AsrError::InferenceFailed(format!("ONNX env lock: {e}")))?;

        // ステップ3: Tokenizer でトークン処理（事前チェック済み）
        let tokenizer = self.tokenizer.as_ref()
            .ok_or_else(|| AsrError::InferenceFailed("Tokenizer missing".into()))?;

        // 初期化時に解決済みの特殊トークン ID を利用
        let bos_id = self.bos_id.unwrap_or(1);
        let lang_id = self.lang_id.unwrap_or(10107);
        let task_id = self.task_id.unwrap_or(10404);
        let eos_id = self.eos_id.unwrap_or(2);
        
        // モデルパス取得
        let encoder_path = self.encoder_path.as_ref()
            .ok_or_else(|| AsrError::InferenceFailed("Encoder path not set".into()))?;
        let decoder_path = self.decoder_path.as_ref()
            .ok_or_else(|| AsrError::InferenceFailed("Decoder path not set".into()))?;

        // 単一 ONNX ファイル（encoder/decoder 一体型）の場合はフルパイプラインを 1 回の run で実行
        // その場合、encoder_path == decoder_path となる
        let use_single_model = encoder_path == decoder_path;

        let text = if use_single_model {
            // ---- 単一モデルパス: mel -> logits -> greedy decode ----
            let mut session = env_guard
                .new_session_builder()
                .and_then(|b| b.with_optimization_level(GraphOptimizationLevel::Basic))
                .and_then(|b| b.with_model_from_file(encoder_path))
                .map_err(|e| AsrError::InferenceFailed(format!("ONNX session build failed: {e}")))?;

            // 入力は [1, 80, 3000] の f32 テンソル 1 つのみと想定
            let input_tensor = vec![mel_array];
            let outputs: Vec<OrtOwnedTensor<f32, IxDyn>> = session
                .run(input_tensor)
                .map_err(|e| AsrError::InferenceFailed(format!("ONNX run failed: {e}")))?;

            if outputs.is_empty() {
                return Err(AsrError::InferenceFailed("ONNX model returned no outputs".into()));
            }

            let logits = &outputs[0];
            let logits_view = logits.view();
            let shape = logits_view.shape();
            if shape.len() != 3 {
                return Err(AsrError::InferenceFailed(format!(
                    "Unexpected logits shape: {:?} (expected [1, seq_len, vocab])",
                    shape
                )));
            }

            let batch = shape[0];
            let seq_len = shape[1];
            let vocab_size = shape[2];
            if batch != 1 {
                return Err(AsrError::InferenceFailed(format!(
                    "Unexpected batch size: {} (expected 1)",
                    batch
                )));
            }

            // Greedy decoding: 各タイムステップ t で argmax_v logits[0, t, v]
            let mut token_ids: Vec<u32> = Vec::with_capacity(seq_len);
            let max_steps = seq_len.min(448); // Whisper のデフォルト max_tokens に近い値

            for t in 0..max_steps {
                let mut best_id: usize = 0;
                let mut best_val: f32 = f32::NEG_INFINITY;

                for v in 0..vocab_size {
                    let val = logits_view[[0, t, v]];
                    if val > best_val {
                        best_val = val;
                        best_id = v;
                    }
                }

                let token_id = best_id as u32;
                token_ids.push(token_id);

                if token_id == eos_id {
                    break;
                }
            }

            // BOS / 言語 / タスクなどの特殊トークンは decode 時に skip_special_tokens=true で除去
            let decoded = tokenizer
                .decode(&token_ids, true)
                .unwrap_or_else(|_| "[decode error]".to_string());

            if decoded.trim().is_empty() {
                format!(
                    "[Whisper single-ONNX inference] BOS={}, JA={}, Task={}, EOS={} | tokens={} (empty decode)",
                    bos_id, lang_id, task_id, eos_id,
                    token_ids.len(),
                )
            } else {
                decoded
            }
        } else {
            // ---- encoder_model.onnx / decoder_model.onnx の 2 ファイル構成の場合 ----
            // onnxruntime 0.0.14 の Rust API では、単一の run() 呼び出しに対して
            // 複数の異なる型の入力（例: encoder_hidden_state=f32, input_ids=i64）を
            // 同時に渡す手段が提供されていないため、完全な Encoder/Decoder 連携は
            // 現時点では未対応。
            //
            // 現段階では Encoder のみ実行し、その結果形状をログとして返す。

            let mut enc_session = env_guard
                .new_session_builder()
                .and_then(|b| b.with_optimization_level(GraphOptimizationLevel::Basic))
                .and_then(|b| b.with_model_from_file(encoder_path))
                .map_err(|e| AsrError::InferenceFailed(format!("Encoder session build failed: {e}")))?;

            let enc_outputs: Vec<OrtOwnedTensor<f32, IxDyn>> = enc_session
                .run(vec![mel_array])
                .map_err(|e| AsrError::InferenceFailed(format!("Encoder run failed: {e}")))?;

            if enc_outputs.is_empty() {
                return Err(AsrError::InferenceFailed("Encoder returned no outputs".into()));
            }

            let enc_view = enc_outputs[0].view();
            let enc_shape = enc_view.shape();

            format!(
                "[Whisper encoder-only] shape={:?} | BOS={}, JA={}, Task={}, EOS={} (decoder not yet wired)",
                enc_shape, bos_id, lang_id, task_id, eos_id,
            )
        };

        let duration = audio.len() as f64 / 16000.0;
        let segment = TranscriptionSegment {
            start: 0.0,
            end: duration,
            text: text.clone(),
            confidence: 0.7,
            speaker: None,
        };

        Ok(TranscriptionResult {
            segments: vec![segment],
            full_text: text,
            processing_time: start_time.elapsed().as_secs_f64(),
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
        self.bos_id = None;
        self.lang_id = None;
        self.task_id = None;
        self.eos_id = None;
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

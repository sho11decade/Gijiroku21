use super::model::{AsrModel, AsrError, TranscriptionResult, TranscriptionSegment};
use std::time::Instant;

/// Whisperモデル（スタブ実装）
/// 
/// 注意: これは実装のスケルトンです。実際のONNX Runtimeとの統合は
/// Phase 2の後半で行います。現在はモックデータを返します。
pub struct WhisperModel {
    model_path: Option<String>,
    is_loaded: bool,
}

impl WhisperModel {
    pub fn new() -> Self {
        WhisperModel {
            model_path: None,
            is_loaded: false,
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
        // TODO: ONNX Runtimeセッション作成
        // - モデルファイルの存在確認
        // - SessionOptionsの設定（CPU/GPU/DirectML）
        // - エンコーダー・デコーダーのロード
        
        // 現在はスタブ実装
        if !std::path::Path::new(model_path).exists() {
            return Err(AsrError::ModelNotFound(model_path.to_string()));
        }
        
        self.model_path = Some(model_path.to_string());
        self.is_loaded = true;
        
        Ok(())
    }
    
    fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, AsrError> {
        if !self.is_loaded {
            return Err(AsrError::ModelNotLoaded);
        }
        
        let start_time = Instant::now();
        
        // TODO: 実際のWhisper推論
        // 1. 音声データの前処理（メルスペクトログラム変換）
        // 2. エンコーダー実行
        // 3. デコーダー実行（トークン生成）
        // 4. テキストデコード
        
        // 現在はモックデータを返す
        let duration = audio.len() as f64 / 16000.0; // 16kHz想定
        
        let mock_segment = TranscriptionSegment {
            start: 0.0,
            end: duration,
            text: format!("モック文字起こし（音声長: {:.2}秒）", duration),
            confidence: 0.95,
            speaker: None,
        };
        
        let result = TranscriptionResult {
            segments: vec![mock_segment],
            full_text: format!("モック文字起こし（音声長: {:.2}秒）", duration),
            processing_time: start_time.elapsed().as_secs_f64(),
        };
        
        Ok(result)
    }
    
    fn is_loaded(&self) -> bool {
        self.is_loaded
    }
    
    fn unload(&mut self) {
        // TODO: ONNX Runtimeセッションのクリーンアップ
        self.model_path = None;
        self.is_loaded = false;
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
        let audio = vec![0.0; 16000]; // 1秒分
        
        let result = model.transcribe(&audio);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), AsrError::ModelNotLoaded));
    }
    
    #[test]
    fn test_whisper_mock_transcription() {
        let mut model = WhisperModel::new();
        
        // 仮のモデルパス（実際のファイルは不要、スタブなので）
        // テスト用に一時ファイルを作成
        let temp_dir = std::env::temp_dir();
        let model_path = temp_dir.join("mock_whisper.onnx");
        std::fs::write(&model_path, b"mock").unwrap();
        
        model.initialize(model_path.to_str().unwrap()).unwrap();
        assert!(model.is_loaded());
        
        let audio = vec![0.0; 16000]; // 1秒分
        let result = model.transcribe(&audio).unwrap();
        
        assert_eq!(result.segments.len(), 1);
        assert!(result.full_text.contains("モック"));
        assert!(result.processing_time >= 0.0);
        
        // クリーンアップ
        std::fs::remove_file(model_path).ok();
    }
}

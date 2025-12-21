use thiserror::Error;
use serde::{Serialize, Deserialize};

/// ASRエラー型
#[derive(Error, Debug)]
pub enum AsrError {
    #[error("Model not loaded")]
    ModelNotLoaded,
    
    #[error("Model file not found: {0}")]
    ModelNotFound(String),
    
    #[error("Inference failed: {0}")]
    InferenceFailed(String),
    
    #[error("Audio processing error: {0}")]
    AudioProcessing(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

/// 文字起こしセグメント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionSegment {
    /// 開始時刻（秒）
    pub start: f64,
    
    /// 終了時刻（秒）
    pub end: f64,
    
    /// 文字起こしテキスト
    pub text: String,
    
    /// 確信度（0.0 ~ 1.0）
    pub confidence: f32,
    
    /// 話者ID（オプション）
    pub speaker: Option<String>,
}

/// 文字起こし結果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionResult {
    /// セグメントのリスト
    pub segments: Vec<TranscriptionSegment>,
    
    /// 全文字起こしテキスト
    pub full_text: String,
    
    /// 処理時間（秒）
    pub processing_time: f64,
}

/// ASRモデルのトレイト
pub trait AsrModel {
    /// モデルを初期化
    fn initialize(&mut self, model_path: &str) -> Result<(), AsrError>;
    
    /// 音声データから文字起こし
    /// 
    /// # Arguments
    /// * `audio` - 音声データ（f32、16kHz、モノラル）
    /// 
    /// # Returns
    /// 文字起こし結果
    fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, AsrError>;
    
    /// モデルがロードされているか
    fn is_loaded(&self) -> bool;
    
    /// モデルをアンロード
    fn unload(&mut self);
}

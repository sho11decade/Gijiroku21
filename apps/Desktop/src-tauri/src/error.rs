use serde::{Serialize, Deserialize};
use thiserror::Error;

/// アプリケーション全体で使用する統一エラー型
#[derive(Debug, Error)]
pub enum AppError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Audio error: {0}")]
    Audio(String),

    #[error("Recording error: {0}")]
    Recording(String),

    #[error("Transcription error: {0}")]
    Transcription(String),

    #[error("NPU error: {0}")]
    Npu(String),

    #[error("State error: {0}")]
    State(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

/// Tauri command用のシリアライズ可能なエラー表現
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl From<AppError> for ErrorResponse {
    fn from(error: AppError) -> Self {
        ErrorResponse {
            error: format!("{:?}", error),
            message: error.to_string(),
        }
    }
}

/// Result型のエイリアス
pub type AppResult<T> = Result<T, AppError>;

/// Tauri command用のResult型（String errorを返す）
impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        error.to_string()
    }
}

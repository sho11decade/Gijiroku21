/// ONNX Runtime 環境・Session の管理モジュール
/// 
/// ONNX Runtime Session は 'static lifetime を要求するため、
/// LazyStatic + Mutex で process-wide singleton として管理

use onnxruntime::{environment::Environment, LoggingLevel};
use std::sync::Mutex;
use once_cell::sync::Lazy;

/// ONNX Runtime のグローバル環境（LazyStatic で初期化）
pub static ONNX_ENV: Lazy<Mutex<Environment>> = Lazy::new(|| {
    let env = Environment::builder()
        .with_name("gijiroku21-whisper")
        .with_log_level(LoggingLevel::Info)
        .build()
        .expect("Failed to create ONNX Runtime environment");
    Mutex::new(env)
});

/// Encoder/Decoder セッション用の設定
pub struct SessionConfig {
    /// モデルファイルパス
    pub model_path: String,
    /// 実行プロバイダの選択：CPUのみ, DirectML有効, CUDA有効など
    /// 将来的に NPU/DirectML 検出結果に基づいて選択
    pub execution_provider: ExecutionProvider,
    /// セッション数の制限（メモリ節約用）
    pub session_limit: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum ExecutionProvider {
    /// CPU のみ
    Cpu,
    /// DirectML (Windows NPU/GPU)
    DirectML,
    /// CUDA (NVIDIA GPU)
    Cuda,
    /// CoreML (macOS)
    CoreML,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            model_path: "".to_string(),
            execution_provider: ExecutionProvider::Cpu,
            session_limit: 2,
        }
    }
}

/// Encoder Session の遅延初期化キャッシュ
pub static ENCODER_SESSION: Lazy<Mutex<Option<String>>> = Lazy::new(|| {
    Mutex::new(None)
});

/// Decoder Session の遅延初期化キャッシュ
pub static DECODER_SESSION: Lazy<Mutex<Option<String>>> = Lazy::new(|| {
    Mutex::new(None)
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onnx_env_creation() {
        let env = ONNX_ENV.lock().unwrap();
        // 環境が正常に作成されていることを確認
        drop(env);
    }
}

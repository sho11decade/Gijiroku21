use tauri::State;
use crate::state::{AppState, Settings, NpuInfo};
use std::path::{Path, PathBuf};

/// モデルダウンロード用のデフォルトURL
/// 実運用では適切なホスティング先のURLに差し替えてください。
///
/// - エンコーダ: encoder_model.onnx
/// - デコーダ:  decoder_model.onnx
/// - Tokenizer: tokenizer.json
///
/// 例: 自前でホストする場合
///   const DEFAULT_ASR_ENCODER_URL: &str = "https://example.com/models/whisper-small/encoder_model.onnx";
///   const DEFAULT_ASR_DECODER_URL: &str = "https://example.com/models/whisper-small/decoder_model.onnx";
///   const DEFAULT_TOKENIZER_URL: &str = "https://example.com/models/whisper-small/tokenizer.json";
///
/// Hugging Face 上の onnx-community/whisper-small を利用する場合は
/// 「blob」ではなく「resolve」を使う必要があります（生バイナリ取得のため）。
const DEFAULT_ASR_ENCODER_URL: &str = "https://huggingface.co/onnx-community/whisper-small/resolve/main/onnx/encoder_model.onnx";
const DEFAULT_ASR_DECODER_URL: &str = "https://huggingface.co/onnx-community/whisper-small/resolve/main/onnx/decoder_model.onnx";
const DEFAULT_TOKENIZER_URL: &str = "https://huggingface.co/onnx-community/whisper-small/resolve/main/tokenizer.json";

/// システム情報を取得
#[tauri::command]
pub async fn get_system_info(state: State<'_, AppState>) -> Result<SystemInfo, String> {
    let npu_info = state.get_npu_info().await;

    Ok(SystemInfo {
        npu_info,
        os: std::env::consts::OS.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        app_version: env!("CARGO_PKG_VERSION").to_string(),
    })
}

/// 設定を取得
#[tauri::command]
pub async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    Ok(state.get_settings().await)
}

/// 設定を更新
#[tauri::command]
pub async fn update_settings(
    state: State<'_, AppState>,
    settings: Settings,
) -> Result<(), String> {
    state.update_settings(settings).await
        .map_err(|e| e.to_string())
}

/// モデル存在チェック結果
#[derive(serde::Serialize)]
pub struct ModelCheck {
    pub ok: bool,
    pub model_dir: String,
    pub tokenizer_dir: String,
    pub required: Vec<String>,
    pub missing: Vec<String>,
}

/// モデルダウンロード結果
#[derive(serde::Serialize)]
pub struct DownloadResult {
    pub ok: bool,
    pub downloaded: Vec<String>,
    pub failed: Vec<String>,
}

pub(crate) fn default_model_dir() -> PathBuf {
    // プロジェクト相対 models/asr をデフォルトとする
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("models")
        .join("asr")
}

pub(crate) fn default_tokenizer_dir() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("models")
        .join("tokenizer")
}

/// モデル存在チェック（最小）
#[tauri::command]
pub async fn check_models(state: State<'_, AppState>) -> Result<ModelCheck, String> {
    let s = state.get_settings().await;
    let base = s
        .model_directory
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(default_model_dir);
    let tokenizer_dir = s
        .tokenizer_directory
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(default_tokenizer_dir);

    // 必須ファイル: whisper-small (単一ONNX) または encoder/decoder ONNX と tokenizer
    let mut required_paths = vec![
        base.join("whisper-small.onnx"),
        tokenizer_dir.join("tokenizer.json"),
    ];
    // マルチファイル構成も許容（encoder_model.onnx / decoder_model.onnx）
    required_paths.push(base.join("encoder_model.onnx"));
    required_paths.push(base.join("decoder_model.onnx"));

    let required = required_paths
        .iter()
        .map(|p| format!("{}", p.display()))
        .collect::<Vec<_>>();
    let mut missing = Vec::new();
    // 条件: (whisper-small.onnx) OR (encoder_model.onnx AND decoder_model.onnx)
    let single_ok = Path::new(&required_paths[0]).exists();
    let enc_ok = Path::new(&required_paths[2]).exists();
    let dec_ok = Path::new(&required_paths[3]).exists();
    let tokenizer_ok = Path::new(&required_paths[1]).exists();

    if !tokenizer_ok {
        missing.push(required[1].clone());
    }
    if !(single_ok || (enc_ok && dec_ok)) {
        if !single_ok {
            missing.push(required[0].clone());
        }
        if !enc_ok {
            missing.push(required[2].clone());
        }
        if !dec_ok {
            missing.push(required[3].clone());
        }
    }

    Ok(ModelCheck {
        ok: missing.is_empty(),
        model_dir: base.to_string_lossy().to_string(),
        tokenizer_dir: tokenizer_dir.to_string_lossy().to_string(),
        required,
        missing,
    })
}

/// モデルファイルを既定URLからダウンロードする
/// - ASRエンコーダ (encoder_model.onnx)
/// - ASRデコーダ (decoder_model.onnx)
/// - Tokenizer (tokenizer.json)
#[tauri::command]
pub async fn download_models(state: State<'_, AppState>) -> Result<DownloadResult, String> {
    use tokio::fs;

    let settings = state.get_settings().await;

    let model_dir = settings
        .model_directory
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(default_model_dir);
    let tokenizer_dir = settings
        .tokenizer_directory
        .as_ref()
        .map(PathBuf::from)
        .unwrap_or_else(default_tokenizer_dir);

    fs::create_dir_all(&model_dir).await.map_err(|e| e.to_string())?;
    fs::create_dir_all(&tokenizer_dir).await.map_err(|e| e.to_string())?;

    let client = reqwest::Client::new();

    let mut downloaded = Vec::new();
    let mut failed = Vec::new();

    // ASR エンコーダ
    let enc_path = model_dir.join("encoder_model.onnx");
    match download_file(&client, DEFAULT_ASR_ENCODER_URL, &enc_path).await {
        Ok(()) => downloaded.push(enc_path.to_string_lossy().to_string()),
        Err(e) => failed.push(format!("ASR encoder: {} -> {}", DEFAULT_ASR_ENCODER_URL, e)),
    }

    // ASR デコーダ
    let dec_path = model_dir.join("decoder_model.onnx");
    match download_file(&client, DEFAULT_ASR_DECODER_URL, &dec_path).await {
        Ok(()) => downloaded.push(dec_path.to_string_lossy().to_string()),
        Err(e) => failed.push(format!("ASR decoder: {} -> {}", DEFAULT_ASR_DECODER_URL, e)),
    }

    // Tokenizer
    let tok_path = tokenizer_dir.join("tokenizer.json");
    match download_file(&client, DEFAULT_TOKENIZER_URL, &tok_path).await {
        Ok(()) => downloaded.push(tok_path.to_string_lossy().to_string()),
        Err(e) => failed.push(format!("Tokenizer: {} -> {}", DEFAULT_TOKENIZER_URL, e)),
    }

    Ok(DownloadResult {
        ok: failed.is_empty(),
        downloaded,
        failed,
    })
}

async fn download_file(
    client: &reqwest::Client,
    url: &str,
    dest: &Path,
) -> Result<(), String> {
    let resp = client
        .get(url)
        .send()
        .await
        .map_err(|e| format!("request error: {e}"))?;

    if !resp.status().is_success() {
        return Err(format!("HTTP status {}", resp.status()));
    }

    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("read body error: {e}"))?;

    tokio::fs::write(dest, &bytes)
        .await
        .map_err(|e| format!("write file error: {e}"))?;

    Ok(())
}

/// NPU情報を取得
#[tauri::command]
pub async fn get_npu_info(state: State<'_, AppState>) -> Result<Option<NpuInfo>, String> {
    Ok(state.get_npu_info().await)
}

/// NPU検出を実行
#[tauri::command]
pub async fn detect_npu(state: State<'_, AppState>) -> Result<NpuInfo, String> {
    state.initialize_npu().await
        .map_err(|e| e.to_string())?;
    
    state.get_npu_info().await
        .ok_or_else(|| "NPU detection failed".to_string())
}

/// システム情報の構造体
#[derive(serde::Serialize)]
pub struct SystemInfo {
    pub npu_info: Option<NpuInfo>,
    pub os: String,
    pub arch: String,
    pub app_version: String,
}

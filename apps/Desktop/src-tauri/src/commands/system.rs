use tauri::State;
use crate::state::{AppState, Settings, NpuInfo};
use std::path::{Path, PathBuf};

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
    pub required: Vec<String>,
    pub missing: Vec<String>,
}

fn default_model_dir() -> PathBuf {
    // プロジェクト相対 models/asr をデフォルトとする
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join("models")
        .join("asr")
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

    // 最小要件: whisper-small.onnx の存在のみ確認（拡張予定）
    let required = vec!["whisper-small.onnx".to_string()];
    let mut missing = Vec::new();
    for name in &required {
        let p = base.join(name);
        if !Path::new(&p).exists() {
            missing.push(name.clone());
        }
    }

    Ok(ModelCheck {
        ok: missing.is_empty(),
        model_dir: base.to_string_lossy().to_string(),
        required,
        missing,
    })
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

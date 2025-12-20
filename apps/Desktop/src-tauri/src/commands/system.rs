use tauri::State;
use crate::state::{AppState, Settings, NpuInfo};

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

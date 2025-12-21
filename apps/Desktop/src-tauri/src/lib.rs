mod error;
mod state;
mod commands;

use state::{AppState, MeetingState};
use commands::RecordingManager;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // アプリケーション状態を初期化
    let app_state = AppState::new();
    let meeting_state = MeetingState::new();
    let recording_manager = RecordingManager::new();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(app_state)
        .manage(meeting_state)
        .manage(recording_manager)
        .invoke_handler(tauri::generate_handler![
            commands::get_system_info,
            commands::get_settings,
            commands::update_settings,
            commands::get_npu_info,
            commands::detect_npu,
            commands::check_models,
            commands::start_recording,
            commands::stop_recording,
            commands::pause_recording,
            commands::resume_recording,
            commands::get_recording_status,
            commands::list_audio_devices,
            commands::start_transcription,
            commands::stop_transcription,
            commands::is_transcription_enabled,
        ])
        .setup(|app| {
            // アプリ起動時の初期化処理
            let app_state = app.state::<AppState>().inner().clone();
            
            // NPU検出を非同期で実行
            tauri::async_runtime::spawn(async move {
                if let Err(e) = app_state.initialize_npu().await {
                    eprintln!("Failed to initialize NPU: {}", e);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use tauri::{State, Emitter};
use crate::state::MeetingState;
use serde::{Serialize, Deserialize};

/// UI送信用の文字起こしセグメント
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSegment {
    pub start: f64,       // 開始時刻（秒）
    pub end: f64,         // 終了時刻（秒）
    pub text: String,     // 文字起こし結果
    pub confidence: f32,  // 信頼度（0.0〜1.0）
    pub speaker: Option<String>, // 話者名（将来拡張）
}

/// 文字起こし開始コマンド
#[tauri::command]
pub async fn start_transcription(
    state: State<'_, MeetingState>,
) -> Result<(), String> {
    *state.transcription_enabled.write().await = true;
    println!("[ASR] 文字起こし開始");
    Ok(())
}

/// 文字起こし停止コマンド
#[tauri::command]
pub async fn stop_transcription(
    state: State<'_, MeetingState>,
) -> Result<(), String> {
    *state.transcription_enabled.write().await = false;
    println!("[ASR] 文字起こし停止");
    Ok(())
}

/// 文字起こし有効状態の取得
#[tauri::command]
pub async fn is_transcription_enabled(
    state: State<'_, MeetingState>,
) -> Result<bool, String> {
    let enabled = *state.transcription_enabled.read().await;
    Ok(enabled)
}

/// セグメントをUIに送信するヘルパー
pub fn emit_transcript_segment<R: tauri::Runtime>(
    app_handle: &tauri::AppHandle<R>,
    segment: &TranscriptSegment,
) {
    if let Err(e) = app_handle.emit("transcript_update", segment) {
        eprintln!("[ASR] イベント送信失敗: {}", e);
    }
}

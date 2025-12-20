use tauri::State;
use gijiroku21_core::audio::AudioCapture;
use gijiroku21_core::storage::MeetingStorage;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use crate::state::MeetingState;

/// 録音コマンド
pub enum RecordingCommand {
    Start,
    Stop,
    Pause,
    Resume,
}

/// 録音マネージャー（スレッド間通信用）
pub struct RecordingManager {
    tx: Arc<RwLock<Option<mpsc::Sender<RecordingCommand>>>>,
}

impl RecordingManager {
    pub fn new() -> Self {
        RecordingManager {
            tx: Arc::new(RwLock::new(None)),
        }
    }

    pub async fn send_command(&self, cmd: RecordingCommand) -> Result<(), String> {
        if let Some(tx) = self.tx.read().await.as_ref() {
            tx.send(cmd).await.map_err(|e| format!("Failed to send command: {}", e))?;
        }
        Ok(())
    }

    pub async fn set_sender(&self, tx: mpsc::Sender<RecordingCommand>) {
        let mut sender = self.tx.write().await;
        *sender = Some(tx);
    }

    pub async fn clear_sender(&self) {
        let mut sender = self.tx.write().await;
        *sender = None;
    }
}

impl Default for RecordingManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 録音を開始
#[tauri::command]
pub async fn start_recording(
    meeting_state: State<'_, MeetingState>,
    recording_manager: State<'_, RecordingManager>,
    title: String,
) -> Result<String, String> {
    // 会議を開始
    meeting_state.start_meeting(title.clone()).await;
    
    let meeting_info = meeting_state.get_current_meeting().await
        .ok_or_else(|| "Failed to start meeting".to_string())?;

    let meeting_id = meeting_info.id.clone();
    
    // Stateの内部データを取得（TauriのStateはすでにArc<T>をラップしている）
    let meeting_state_handle = meeting_state.inner().clone();

    // チャネルを作成
    let (tx, rx) = mpsc::channel::<RecordingCommand>(32);
    
    // 録音マネージャーにsenderを設定
    recording_manager.set_sender(tx).await;

    // 別スレッドで録音処理を実行（std::threadを使用）
    std::thread::spawn(move || {
        // 新しいtokioランタイムを作成
        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        rt.block_on(async move {
            audio_recording_thread(rx, meeting_id, meeting_state_handle).await;
        });
    });

    Ok(meeting_info.id)
}

/// 録音処理スレッド（非同期タスク）
async fn audio_recording_thread(
    mut rx: mpsc::Receiver<RecordingCommand>,
    meeting_id: String,
    meeting_state: MeetingState,
) {
    // AudioCaptureを初期化
    let mut capture = match AudioCapture::new() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to create audio capture: {}", e);
            return;
        }
    };

    if let Err(e) = capture.initialize() {
        eprintln!("Failed to initialize audio device: {}", e);
        return;
    }

    if let Err(e) = capture.start_recording() {
        eprintln!("Failed to start recording: {}", e);
        return;
    }

    // コマンドを待機
    while let Some(cmd) = rx.recv().await {
        match cmd {
            RecordingCommand::Stop => {
                // 録音を停止して保存
                if let Err(e) = capture.stop_recording() {
                    eprintln!("Failed to stop recording: {}", e);
                }

                let buffer = capture.get_buffer();
                let samples = buffer.get_all().await;
                let sample_rate = capture.sample_rate();

                // ストレージに保存
                if let Ok(storage) = MeetingStorage::default_location() {
                    if let Err(e) = storage.save_audio(&meeting_id, &samples, sample_rate) {
                        eprintln!("Failed to save audio: {}", e);
                    }
                }

                meeting_state.end_meeting().await;
                break;
            }
            RecordingCommand::Pause => {
                if let Err(e) = capture.stop_recording() {
                    eprintln!("Failed to pause recording: {}", e);
                }
                meeting_state.pause().await;
            }
            RecordingCommand::Resume => {
                if let Err(e) = capture.start_recording() {
                    eprintln!("Failed to resume recording: {}", e);
                }
                meeting_state.resume().await;
            }
            _ => {}
        }
    }
}

/// 録音を停止
#[tauri::command]
pub async fn stop_recording(
    recording_manager: State<'_, RecordingManager>,
) -> Result<(), String> {
    recording_manager.send_command(RecordingCommand::Stop).await?;
    recording_manager.clear_sender().await;
    Ok(())
}

/// 録音を一時停止
#[tauri::command]
pub async fn pause_recording(
    recording_manager: State<'_, RecordingManager>,
) -> Result<(), String> {
    recording_manager.send_command(RecordingCommand::Pause).await
}

/// 録音を再開
#[tauri::command]
pub async fn resume_recording(
    recording_manager: State<'_, RecordingManager>,
) -> Result<(), String> {
    recording_manager.send_command(RecordingCommand::Resume).await
}

/// 録音状態を取得
#[tauri::command]
pub async fn get_recording_status(
    meeting_state: State<'_, MeetingState>,
) -> Result<String, String> {
    let status = meeting_state.get_status().await;
    Ok(serde_json::to_string(&status).unwrap_or_default())
}

/// 利用可能な音声入力デバイス一覧を取得
#[tauri::command]
pub async fn list_audio_devices() -> Result<Vec<String>, String> {
    let capture = AudioCapture::new()
        .map_err(|e| format!("Failed to create audio capture: {}", e))?;
    
    Ok(capture.list_input_devices())
}

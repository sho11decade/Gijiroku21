use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use chrono::{DateTime, Utc};

/// 録音状態
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum RecordingStatus {
    Idle,
    Recording,
    Paused,
    Processing,
}

/// 会議メタデータ
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingMetadata {
    pub id: String,
    pub title: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub duration_seconds: Option<u64>,
}

/// 会議の状態
#[derive(Clone)]
pub struct MeetingState {
    /// 現在の録音状態
    pub status: Arc<RwLock<RecordingStatus>>,
    /// 現在の会議情報
    pub current_meeting: Arc<RwLock<Option<MeetingMetadata>>>,
    /// 文字起こし中間結果（リアルタイム表示用）
    pub transcript_buffer: Arc<RwLock<Vec<String>>>,
}

impl MeetingState {
    pub fn new() -> Self {
        MeetingState {
            status: Arc::new(RwLock::new(RecordingStatus::Idle)),
            current_meeting: Arc::new(RwLock::new(None)),
            transcript_buffer: Arc::new(RwLock::new(Vec::new())),
        }
    }

    /// 新しい会議を開始
    pub async fn start_meeting(&self, title: String) {
        let meeting = MeetingMetadata {
            id: Uuid::new_v4().to_string(),
            title,
            started_at: Utc::now(),
            ended_at: None,
            duration_seconds: None,
        };

        let mut current = self.current_meeting.write().await;
        *current = Some(meeting);

        let mut status = self.status.write().await;
        *status = RecordingStatus::Recording;
    }

    /// 会議を終了
    pub async fn end_meeting(&self) {
        let mut current = self.current_meeting.write().await;
        if let Some(ref mut meeting) = *current {
            meeting.ended_at = Some(Utc::now());
            
            let duration = meeting.ended_at.unwrap()
                .signed_duration_since(meeting.started_at)
                .num_seconds();
            meeting.duration_seconds = Some(duration as u64);
        }

        let mut status = self.status.write().await;
        *status = RecordingStatus::Idle;
    }

    /// 録音を一時停止
    pub async fn pause(&self) {
        let mut status = self.status.write().await;
        *status = RecordingStatus::Paused;
    }

    /// 録音を再開
    pub async fn resume(&self) {
        let mut status = self.status.write().await;
        *status = RecordingStatus::Recording;
    }

    /// 現在の録音状態を取得
    pub async fn get_status(&self) -> RecordingStatus {
        self.status.read().await.clone()
    }

    /// 現在の会議情報を取得
    pub async fn get_current_meeting(&self) -> Option<MeetingMetadata> {
        self.current_meeting.read().await.clone()
    }

    /// 文字起こしテキストを追加
    pub async fn add_transcript(&self, text: String) {
        let mut buffer = self.transcript_buffer.write().await;
        buffer.push(text);
    }

    /// 文字起こしバッファをクリア
    pub async fn clear_transcript(&self) {
        let mut buffer = self.transcript_buffer.write().await;
        buffer.clear();
    }

    /// 文字起こし全体を取得
    pub async fn get_transcript(&self) -> Vec<String> {
        self.transcript_buffer.read().await.clone()
    }
}

impl Default for MeetingState {
    fn default() -> Self {
        Self::new()
    }
}

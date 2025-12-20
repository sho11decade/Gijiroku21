use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use chrono::{DateTime, Utc};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("Meeting not found: {0}")]
    NotFound(String),
    
    #[error("Invalid path: {0}")]
    InvalidPath(String),
}

pub type Result<T> = std::result::Result<T, StorageError>;

/// 会議データの保存構造
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingData {
    pub id: String,
    pub title: String,
    pub started_at: DateTime<Utc>,
    pub ended_at: Option<DateTime<Utc>>,
    pub transcript: Vec<TranscriptSegment>,
    pub summary: Option<String>,
    pub audio_file: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptSegment {
    pub timestamp: f64,
    pub text: String,
    pub speaker: Option<String>,
}

/// 会議データのストレージ管理
pub struct MeetingStorage {
    base_dir: PathBuf,
}

impl MeetingStorage {
    /// 新しいMeetingStorageインスタンスを作成
    pub fn new(base_dir: impl AsRef<Path>) -> Result<Self> {
        let base_dir = base_dir.as_ref().to_path_buf();
        
        // ベースディレクトリを作成
        if !base_dir.exists() {
            std::fs::create_dir_all(&base_dir)?;
        }

        Ok(MeetingStorage { base_dir })
    }

    /// デフォルトのストレージディレクトリを使用
    pub fn default_location() -> Result<Self> {
        let base_dir = directories::ProjectDirs::from("com", "gijiroku21", "Gijiroku21")
            .ok_or_else(|| StorageError::InvalidPath("Could not determine data directory".to_string()))?
            .data_dir()
            .join("meetings");

        Self::new(base_dir)
    }

    /// 会議ディレクトリのパスを取得
    fn meeting_dir(&self, meeting_id: &str) -> PathBuf {
        self.base_dir.join(meeting_id)
    }

    /// 会議データを保存
    pub fn save_meeting(&self, meeting: &MeetingData) -> Result<()> {
        let meeting_dir = self.meeting_dir(&meeting.id);
        std::fs::create_dir_all(&meeting_dir)?;

        let json_path = meeting_dir.join("meeting.json");
        let json_content = serde_json::to_string_pretty(meeting)?;
        std::fs::write(json_path, json_content)?;

        Ok(())
    }

    /// 会議データを読み込み
    pub fn load_meeting(&self, meeting_id: &str) -> Result<MeetingData> {
        let json_path = self.meeting_dir(meeting_id).join("meeting.json");
        
        if !json_path.exists() {
            return Err(StorageError::NotFound(meeting_id.to_string()));
        }

        let json_content = std::fs::read_to_string(json_path)?;
        let meeting: MeetingData = serde_json::from_str(&json_content)?;

        Ok(meeting)
    }

    /// すべての会議IDを取得
    pub fn list_meetings(&self) -> Result<Vec<String>> {
        let mut meetings = Vec::new();

        if !self.base_dir.exists() {
            return Ok(meetings);
        }

        for entry in std::fs::read_dir(&self.base_dir)? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                if let Some(name) = entry.file_name().to_str() {
                    meetings.push(name.to_string());
                }
            }
        }

        Ok(meetings)
    }

    /// 会議を削除
    pub fn delete_meeting(&self, meeting_id: &str) -> Result<()> {
        let meeting_dir = self.meeting_dir(meeting_id);
        
        if meeting_dir.exists() {
            std::fs::remove_dir_all(meeting_dir)?;
        }

        Ok(())
    }

    /// 音声ファイルのパスを取得
    pub fn audio_file_path(&self, meeting_id: &str) -> PathBuf {
        self.meeting_dir(meeting_id).join("audio.wav")
    }

    /// 音声データを保存
    pub fn save_audio(&self, meeting_id: &str, samples: &[f32], sample_rate: u32) -> Result<()> {
        let audio_path = self.audio_file_path(meeting_id);
        let meeting_dir = self.meeting_dir(meeting_id);
        std::fs::create_dir_all(&meeting_dir)?;

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(audio_path, spec)
            .map_err(|e| StorageError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        for &sample in samples {
            let amplitude = (sample * i16::MAX as f32) as i16;
            writer.write_sample(amplitude)
                .map_err(|e| StorageError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;
        }

        writer.finalize()
            .map_err(|e| StorageError::Io(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        Ok(())
    }

    /// Markdownでエクスポート
    pub fn export_markdown(&self, meeting: &MeetingData) -> Result<String> {
        let mut md = String::new();
        
        md.push_str(&format!("# {}\n\n", meeting.title));
        md.push_str(&format!("**開始時刻**: {}\n\n", meeting.started_at.format("%Y年%m月%d日 %H:%M")));
        
        if let Some(ended_at) = meeting.ended_at {
            md.push_str(&format!("**終了時刻**: {}\n\n", ended_at.format("%Y年%m月%d日 %H:%M")));
        }

        if let Some(summary) = &meeting.summary {
            md.push_str("## 要約\n\n");
            md.push_str(summary);
            md.push_str("\n\n");
        }

        md.push_str("## 文字起こし\n\n");
        for segment in &meeting.transcript {
            let timestamp = format!("{:02}:{:02}", 
                (segment.timestamp / 60.0) as u32,
                (segment.timestamp % 60.0) as u32
            );
            
            if let Some(speaker) = &segment.speaker {
                md.push_str(&format!("[{}] **{}**: {}\n\n", timestamp, speaker, segment.text));
            } else {
                md.push_str(&format!("[{}] {}\n\n", timestamp, segment.text));
            }
        }

        Ok(md)
    }
}

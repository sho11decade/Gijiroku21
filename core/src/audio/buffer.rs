use std::sync::Arc;
use tokio::sync::RwLock;

/// リングバッファ形式の音声データ保存
pub struct AudioBuffer {
    buffer: Arc<RwLock<Vec<f32>>>,
    capacity: usize,
}

impl AudioBuffer {
    pub fn new(capacity: usize) -> Self {
        AudioBuffer {
            buffer: Arc::new(RwLock::new(Vec::with_capacity(capacity))),
            capacity,
        }
    }

    /// 音声データを追加（非同期）
    pub async fn push(&self, samples: &[f32]) {
        let mut buffer = self.buffer.write().await;
        
        // 容量を超えた場合は古いデータを削除
        if buffer.len() + samples.len() > self.capacity {
            let overflow = buffer.len() + samples.len() - self.capacity;
            buffer.drain(0..overflow);
        }
        
        buffer.extend_from_slice(samples);
    }

    /// 音声データを追加（同期版）
    /// cpal のコールバックスレッドなど、Tokio ランタイム外から呼び出すために使用
    pub fn push_blocking(&self, samples: &[f32]) {
        let mut buffer = self.buffer.blocking_write();

        if buffer.len() + samples.len() > self.capacity {
            let overflow = buffer.len() + samples.len() - self.capacity;
            buffer.drain(0..overflow);
        }

        buffer.extend_from_slice(samples);
    }

    /// バッファ内の全データを取得
    pub async fn get_all(&self) -> Vec<f32> {
        self.buffer.read().await.clone()
    }

    /// バッファをクリア
    pub async fn clear(&self) {
        self.buffer.write().await.clear();
    }

    /// バッファ内のサンプル数
    pub async fn len(&self) -> usize {
        self.buffer.read().await.len()
    }

    /// バッファが空かどうか
    pub async fn is_empty(&self) -> bool {
        self.buffer.read().await.is_empty()
    }

    /// 最新のN秒分のチャンクを取得（ASR用）
    /// 
    /// # Arguments
    /// * `duration_sec` - チャンクの長さ（秒）
    /// * `sample_rate` - サンプルレート（Hz）
    /// 
    /// # Returns
    /// 指定された長さの音声データ。データが不足している場合は利用可能な分のみ返す
    pub async fn get_chunk(&self, duration_sec: f32, sample_rate: u32) -> Vec<f32> {
        let chunk_size = (duration_sec * sample_rate as f32) as usize;
        let buffer = self.buffer.read().await;
        
        if buffer.len() <= chunk_size {
            // バッファ全体を返す
            buffer.clone()
        } else {
            // 最新のchunk_size分を返す
            buffer[buffer.len() - chunk_size..].to_vec()
        }
    }

    /// 指定された範囲のチャンクを取得（タイムスタンプベース）
    /// 
    /// # Arguments
    /// * `start_sec` - 開始位置（秒）
    /// * `end_sec` - 終了位置（秒）
    /// * `sample_rate` - サンプルレート（Hz）
    pub async fn get_chunk_range(&self, start_sec: f32, end_sec: f32, sample_rate: u32) -> Vec<f32> {
        let start_sample = (start_sec * sample_rate as f32) as usize;
        let end_sample = (end_sec * sample_rate as f32) as usize;
        let buffer = self.buffer.read().await;
        
        let start_idx = start_sample.min(buffer.len());
        let end_idx = end_sample.min(buffer.len());
        
        if start_idx >= end_idx {
            return Vec::new();
        }
        
        buffer[start_idx..end_idx].to_vec()
    }

    /// バッファの総時間を取得（秒）
    pub async fn duration_sec(&self, sample_rate: u32) -> f32 {
        let len = self.len().await;
        len as f32 / sample_rate as f32
    }
}


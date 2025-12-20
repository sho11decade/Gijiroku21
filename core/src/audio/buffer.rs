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

    /// 音声データを追加
    pub async fn push(&self, samples: &[f32]) {
        let mut buffer = self.buffer.write().await;
        
        // 容量を超えた場合は古いデータを削除
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
}

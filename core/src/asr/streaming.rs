/// ストリーミング文字起こし処理
/// 
/// リアルタイム音声認識のためのチャンク処理とASR実行を管理

use super::model::{AsrModel, AsrError, TranscriptionSegment};
use crate::audio::{AudioBuffer, resample_for_whisper};
use std::sync::Arc;
use tokio::time::{Duration, interval};

/// ストリーミング文字起こしの設定
#[derive(Debug, Clone)]
pub struct StreamingConfig {
    /// チャンクの長さ（秒）
    pub chunk_duration: f32,
    
    /// チャンク処理間隔（秒）
    pub interval_sec: f32,
    
    /// 入力サンプルレート（Hz）
    pub input_sample_rate: u32,
    
    /// オーバーラップ長（秒）- 音声の途切れ防止
    pub overlap_duration: f32,
}

impl Default for StreamingConfig {
    fn default() -> Self {
        StreamingConfig {
            chunk_duration: 30.0,      // 30秒チャンク
            interval_sec: 5.0,          // 5秒間隔
            input_sample_rate: 48000,   // 48kHz
            overlap_duration: 1.0,      // 1秒オーバーラップ
        }
    }
}

/// ストリーミング文字起こしプロセッサ
pub struct StreamingTranscriber<M: AsrModel> {
    model: Arc<M>,
    config: StreamingConfig,
    last_processed_time: f64,
}

impl<M: AsrModel + Send + Sync> StreamingTranscriber<M> {
    pub fn new(model: Arc<M>, config: StreamingConfig) -> Self {
        StreamingTranscriber {
            model,
            config,
            last_processed_time: 0.0,
        }
    }

    /// 音声バッファから次のチャンクを処理
    /// 
    /// # Arguments
    /// * `buffer` - 音声バッファ
    /// 
    /// # Returns
    /// 文字起こし結果（新しいセグメントがある場合のみ）
    pub async fn process_next_chunk(
        &mut self,
        buffer: &AudioBuffer,
    ) -> Result<Option<Vec<TranscriptionSegment>>, AsrError> {
        // バッファの総時間を取得
        let buffer_duration = buffer.duration_sec(self.config.input_sample_rate).await as f64;
        
        // まだ処理間隔に達していない場合はスキップ
        if buffer_duration - self.last_processed_time < self.config.interval_sec as f64 {
            return Ok(None);
        }
        
        // 最新のチャンクを取得
        let chunk = buffer
            .get_chunk(self.config.chunk_duration, self.config.input_sample_rate)
            .await;
        
        if chunk.is_empty() {
            return Ok(None);
        }
        
        // 16kHzにリサンプリング
        let resampled = resample_for_whisper(&chunk, self.config.input_sample_rate);
        
        // ASR実行
        let result = self.model.transcribe(&resampled)?;
        
        // タイムスタンプを調整（バッファの開始位置からの相対時間）
        let chunk_start_time = buffer_duration - (chunk.len() as f64 / self.config.input_sample_rate as f64);
        let adjusted_segments: Vec<TranscriptionSegment> = result
            .segments
            .into_iter()
            .map(|mut seg| {
                seg.start += chunk_start_time;
                seg.end += chunk_start_time;
                seg
            })
            .collect();
        
        // 処理済み時刻を更新
        self.last_processed_time = buffer_duration;
        
        Ok(Some(adjusted_segments))
    }
    
    /// タイマー駆動でストリーミング処理を実行
    /// 
    /// # Arguments
    /// * `buffer` - 音声バッファ
    /// * `callback` - セグメント生成時のコールバック
    pub async fn run_streaming<F>(
        &mut self,
        buffer: Arc<AudioBuffer>,
        mut callback: F,
    ) -> Result<(), AsrError>
    where
        F: FnMut(Vec<TranscriptionSegment>) + Send,
    {
        let mut tick = interval(Duration::from_secs_f32(self.config.interval_sec));
        
        loop {
            tick.tick().await;
            
            if let Some(segments) = self.process_next_chunk(&buffer).await? {
                callback(segments);
            }
        }
    }
    
    /// 処理状態をリセット
    pub fn reset(&mut self) {
        self.last_processed_time = 0.0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::asr::model::{TranscriptionResult, TranscriptionSegment};

    struct DummyModel;

    impl AsrModel for DummyModel {
        fn initialize(&mut self, _model_path: &str) -> Result<(), AsrError> { Ok(()) }
        fn transcribe(&self, audio: &[f32]) -> Result<TranscriptionResult, AsrError> {
            let duration = audio.len() as f64 / 16000.0;
            Ok(TranscriptionResult {
                segments: vec![TranscriptionSegment {
                    start: 0.0,
                    end: duration,
                    text: "dummy".to_string(),
                    confidence: 1.0,
                    speaker: None,
                }],
                full_text: "dummy".to_string(),
                processing_time: 0.0,
            })
        }
        fn is_loaded(&self) -> bool { true }
        fn unload(&mut self) {}
    }
    
    #[tokio::test]
    async fn test_streaming_config_default() {
        let config = StreamingConfig::default();
        assert_eq!(config.chunk_duration, 30.0);
        assert_eq!(config.interval_sec, 5.0);
        assert_eq!(config.input_sample_rate, 48000);
    }
    
    #[tokio::test]
    async fn test_streaming_transcriber_empty_buffer() {
        let model = Arc::new(DummyModel);
        let config = StreamingConfig::default();
        let mut transcriber = StreamingTranscriber::new(model, config);
        
        let buffer = AudioBuffer::new(48000 * 60); // 60秒バッファ
        
        let result = transcriber.process_next_chunk(&buffer).await;
        assert!(result.is_ok());
        assert!(result.unwrap().is_none()); // 空バッファなので何も返さない
    }
    
    #[tokio::test]
    async fn test_streaming_transcriber_with_mock_data() {
        let model = Arc::new(DummyModel);
        let config = StreamingConfig {
            chunk_duration: 5.0,  // 短めのチャンク
            interval_sec: 2.0,
            input_sample_rate: 48000,
            overlap_duration: 0.5,
        };
        
        let mut transcriber = StreamingTranscriber::new(model, config);
        
        // 10秒分の音声データを追加
        let buffer = AudioBuffer::new(48000 * 60);
        let samples = vec![0.1; 48000 * 10]; // 10秒分
        buffer.push(&samples).await;
        
        // 最初の処理
        let result1 = transcriber.process_next_chunk(&buffer).await;
        assert!(result1.is_ok());
        let segments1 = result1.unwrap();
        assert!(segments1.is_some()); // データがあるので処理される
        
        // すぐに再処理（interval未満なのでスキップされる）
        let result2 = transcriber.process_next_chunk(&buffer).await;
        assert!(result2.is_ok());
        assert!(result2.unwrap().is_none());
    }
}

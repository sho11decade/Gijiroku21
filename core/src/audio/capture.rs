use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{Device, Host, Stream, StreamConfig};
use std::sync::Arc;
use thiserror::Error;

use super::AudioBuffer;

#[derive(Debug, Error)]
pub enum AudioCaptureError {
    #[error("Audio device error: {0}")]
    DeviceError(String),
    
    #[error("Stream error: {0}")]
    StreamError(String),
    
    #[error("No default input device found")]
    NoDefaultDevice,
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

pub type Result<T> = std::result::Result<T, AudioCaptureError>;

/// 音声キャプチャシステム
pub struct AudioCapture {
    host: Host,
    device: Option<Device>,
    stream: Option<Stream>,
    buffer: Arc<AudioBuffer>,
    sample_rate: u32,
}

impl AudioCapture {
    /// 新しいAudioCaptureインスタンスを作成
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        
        Ok(AudioCapture {
            host,
            device: None,
            stream: None,
            buffer: Arc::new(AudioBuffer::new(48000 * 60)), // 60秒分のバッファ
            sample_rate: 48000,
        })
    }

    /// デフォルトの入力デバイスを初期化
    pub fn initialize(&mut self) -> Result<()> {
        let device = self.host
            .default_input_device()
            .ok_or(AudioCaptureError::NoDefaultDevice)?;
        
        self.device = Some(device);
        Ok(())
    }

    /// 録音を開始
    pub fn start_recording(&mut self) -> Result<()> {
        let device = self.device.as_ref()
            .ok_or_else(|| AudioCaptureError::DeviceError("Device not initialized".to_string()))?;

        let config = device.default_input_config()
            .map_err(|e| AudioCaptureError::ConfigError(e.to_string()))?;

        self.sample_rate = config.sample_rate().0;
        let channels = config.channels() as usize;

        let buffer = Arc::clone(&self.buffer);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                self.build_input_stream::<f32>(device, &config.into(), buffer, channels)?
            }
            cpal::SampleFormat::I16 => {
                self.build_input_stream::<i16>(device, &config.into(), buffer, channels)?
            }
            cpal::SampleFormat::U16 => {
                self.build_input_stream::<u16>(device, &config.into(), buffer, channels)?
            }
            _ => {
                return Err(AudioCaptureError::ConfigError(
                    "Unsupported sample format".to_string()
                ));
            }
        };

        stream.play()
            .map_err(|e| AudioCaptureError::StreamError(e.to_string()))?;

        self.stream = Some(stream);
        Ok(())
    }

    /// 録音を停止
    pub fn stop_recording(&mut self) -> Result<()> {
        if let Some(stream) = self.stream.take() {
            stream.pause()
                .map_err(|e| AudioCaptureError::StreamError(e.to_string()))?;
        }
        Ok(())
    }

    /// 入力ストリームを構築
    fn build_input_stream<T>(
        &self,
        device: &Device,
        config: &StreamConfig,
        buffer: Arc<AudioBuffer>,
        channels: usize,
    ) -> Result<Stream>
    where
        T: cpal::Sample + cpal::SizedSample + ToFloat,
        f32: From<T>,
    {
        let err_fn = |err| eprintln!("Stream error: {}", err);

        let stream = device.build_input_stream(
            config,
            move |data: &[T], _: &cpal::InputCallbackInfo| {
                // サンプルをf32に変換してモノラルに変換
                let samples: Vec<f32> = data
                    .chunks(channels)
                    .map(|chunk| {
                        let sum = chunk.iter()
                            .map(|&s| s.to_float())
                            .sum::<f32>();
                        sum / channels as f32
                    })
                    .collect();
                // Tokio ランタイムに依存しない同期追加
                let buffer_clone = Arc::clone(&buffer);
                buffer_clone.push_blocking(&samples);
            },
            err_fn,
            None,
        )
        .map_err(|e| AudioCaptureError::StreamError(e.to_string()))?;

        Ok(stream)
    }

    /// バッファを取得
    pub fn get_buffer(&self) -> Arc<AudioBuffer> {
        Arc::clone(&self.buffer)
    }

    /// サンプルレートを取得
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    /// 利用可能な入力デバイスのリストを取得
    pub fn list_input_devices(&self) -> Vec<String> {
        self.host
            .input_devices()
            .ok()
            .map(|devices| {
                devices
                    .filter_map(|d| d.name().ok())
                    .collect()
            })
            .unwrap_or_default()
    }
}

impl Default for AudioCapture {
    fn default() -> Self {
        Self::new().expect("Failed to create AudioCapture")
    }
}

/// cpal::Sampleトレイトのf32変換ヘルパー
trait ToFloat {
    fn to_float(&self) -> f32;
}

impl ToFloat for f32 {
    fn to_float(&self) -> f32 {
        *self
    }
}

impl ToFloat for i16 {
    fn to_float(&self) -> f32 {
        *self as f32 / i16::MAX as f32
    }
}

impl ToFloat for u16 {
    fn to_float(&self) -> f32 {
        (*self as f32 / u16::MAX as f32) * 2.0 - 1.0
    }
}

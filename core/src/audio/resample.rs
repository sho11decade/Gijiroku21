/// 音声リサンプリング機能
/// 
/// WhisperモデルはサンプルレートとしてWhisperモデルは16kHzを期待するため、
/// cpalから取得した48kHzの音声を16kHzにダウンサンプリングする

/// シンプルなリサンプリング（線形補間）
/// 
/// # Arguments
/// * `input` - 入力音声データ
/// * `input_rate` - 入力サンプルレート（Hz）
/// * `output_rate` - 出力サンプルレート（Hz）
/// 
/// # Returns
/// リサンプリングされた音声データ
pub fn resample_linear(input: &[f32], input_rate: u32, output_rate: u32) -> Vec<f32> {
    if input_rate == output_rate {
        return input.to_vec();
    }

    let ratio = input_rate as f64 / output_rate as f64;
    let output_len = (input.len() as f64 / ratio).ceil() as usize;
    let mut output = Vec::with_capacity(output_len);

    for i in 0..output_len {
        let src_idx = i as f64 * ratio;
        let idx0 = src_idx.floor() as usize;
        let idx1 = (idx0 + 1).min(input.len() - 1);
        let frac = src_idx - idx0 as f64;

        // 線形補間
        let sample = input[idx0] * (1.0 - frac) as f32 + input[idx1] * frac as f32;
        output.push(sample);
    }

    output
}

/// Whisper用に48kHz → 16kHzへリサンプリング
pub fn resample_for_whisper(input: &[f32], input_rate: u32) -> Vec<f32> {
    const WHISPER_SAMPLE_RATE: u32 = 16000;
    resample_linear(input, input_rate, WHISPER_SAMPLE_RATE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resample_same_rate() {
        let input = vec![1.0, 2.0, 3.0, 4.0];
        let output = resample_linear(&input, 16000, 16000);
        assert_eq!(input, output);
    }

    #[test]
    fn test_resample_downsample() {
        let input = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let output = resample_linear(&input, 48000, 16000);
        
        // 48000 -> 16000 は 1/3 なので、出力は約2サンプル
        assert!(output.len() >= 1 && output.len() <= 3);
    }

    #[test]
    fn test_resample_for_whisper() {
        // 48kHz 1秒分のサンプル（48000サンプル）
        let input: Vec<f32> = (0..48000).map(|_| 0.5).collect();
        let output = resample_for_whisper(&input, 48000);
        
        // 16kHz 1秒分のサンプル（16000サンプル）になるはず
        assert_eq!(output.len(), 16000);
    }

    #[test]
    fn test_resample_preserves_amplitude() {
        let input = vec![0.5; 4800]; // 48kHz 0.1秒
        let output = resample_linear(&input, 48000, 16000);
        
        // 振幅がおおよそ保たれているか確認
        for sample in output.iter() {
            assert!((sample - 0.5).abs() < 0.1);
        }
    }
}

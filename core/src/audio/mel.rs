use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

/// Whisper向けメルスペクトログラム設定
pub struct MelConfig {
    pub sample_rate: u32,
    pub n_fft: usize,
    pub hop_length: usize,
    pub win_length: usize,
    pub n_mels: usize,
    pub f_min: f32,
    pub f_max: f32,
    pub target_frames: usize,
}

impl Default for MelConfig {
    fn default() -> Self {
        MelConfig {
            sample_rate: 16_000,
            n_fft: 400,
            hop_length: 160,
            win_length: 400,
            n_mels: 80,
            f_min: 0.0,
            f_max: 8_000.0,
            target_frames: 3_000,
        }
    }
}

/// 16kHz音声から log-mel スペクトログラム（80x3000）を生成
pub fn log_mel_spectrogram(samples: &[f32], cfg: &MelConfig) -> Vec<f32> {
    if samples.is_empty() {
        return vec![f32::NEG_INFINITY; cfg.n_mels * cfg.target_frames];
    }

    let pad = cfg.n_fft / 2;
    let padded = reflect_pad(samples, pad);
    let frame_count = ((padded.len().saturating_sub(cfg.n_fft)) / cfg.hop_length) + 1;
    let hann = hann_window(cfg.win_length);
    let fft_size = cfg.n_fft;

    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(fft_size);
    let mut spectrum = vec![Complex::new(0.0, 0.0); fft_size];
    let mel_filters = build_mel_filterbank(cfg);

    let max_frames = cfg.target_frames.min(frame_count);
    let mut mel_output = vec![f32::NEG_INFINITY; cfg.n_mels * cfg.target_frames];

    for frame_idx in 0..max_frames {
        let start = frame_idx * cfg.hop_length;
        let end = start + cfg.n_fft;
        let frame = &padded[start..end];

        for i in 0..fft_size {
            let windowed = if i < cfg.win_length { frame[i] * hann[i] } else { frame[i] };
            spectrum[i] = Complex::new(windowed, 0.0);
        }

        fft.process(&mut spectrum);

        // power spectrum (only half + 1)
        let mut power = vec![0.0f32; cfg.n_fft / 2 + 1];
        for i in 0..power.len() {
            let re = spectrum[i].re;
            let im = spectrum[i].im;
            power[i] = re * re + im * im;
        }

        let mel_bins = apply_mel_filters(&mel_filters, &power);
        for mel_idx in 0..cfg.n_mels {
            mel_output[mel_idx * cfg.target_frames + frame_idx] = mel_bins[mel_idx];
        }
    }

    // log10 with clamp, fill remaining frames with minimum value
    const EPS: f32 = 1e-10;
    for val in mel_output.iter_mut() {
        if *val <= 0.0 {
            *val = (EPS).log10();
        } else {
            *val = (*val).max(EPS).log10();
        }
    }

    mel_output
}

fn reflect_pad(samples: &[f32], pad: usize) -> Vec<f32> {
    if pad == 0 || samples.is_empty() {
        return samples.to_vec();
    }

    let mut padded = Vec::with_capacity(samples.len() + pad * 2);

    // left pad (reflect)
    for i in 0..pad {
        let idx = pad - i;
        let sample = samples.get(idx).copied().unwrap_or_else(|| samples.first().copied().unwrap_or(0.0));
        padded.push(sample);
    }

    padded.extend_from_slice(samples);

    // right pad
    for i in 0..pad {
        let idx = samples.len().saturating_sub(2 + i);
        let sample = samples.get(idx).copied().unwrap_or_else(|| samples.last().copied().unwrap_or(0.0));
        padded.push(sample);
    }

    padded
}

fn hann_window(len: usize) -> Vec<f32> {
    (0..len)
        .map(|i| 0.5 - 0.5 * (2.0 * PI * i as f32 / (len as f32 - 1.0)).cos())
        .collect()
}

fn hz_to_mel(hz: f32) -> f32 {
    2595.0 * (1.0 + hz / 700.0).log10()
}

fn mel_to_hz(mel: f32) -> f32 {
    700.0 * (10f32.powf(mel / 2595.0) - 1.0)
}

fn build_mel_filterbank(cfg: &MelConfig) -> Vec<Vec<f32>> {
    let n_fft = cfg.n_fft;
    let n_mels = cfg.n_mels;
    let f_min = cfg.f_min;
    let f_max = cfg.f_max;
    let sample_rate = cfg.sample_rate as f32;

    let mel_min = hz_to_mel(f_min);
    let mel_max = hz_to_mel(f_max);
    let mut mel_points = Vec::with_capacity(n_mels + 2);
    for i in 0..(n_mels + 2) {
        mel_points.push(mel_min + (mel_max - mel_min) * i as f32 / (n_mels as f32 + 1.0));
    }

    let hz_points: Vec<f32> = mel_points.into_iter().map(mel_to_hz).collect();
    let bin: Vec<usize> = hz_points
        .iter()
        .map(|&hz| ((n_fft + 1) as f32 * hz / sample_rate).floor() as usize)
        .collect();

    let mut filterbank = vec![vec![0.0f32; n_fft / 2 + 1]; n_mels];
    for m in 1..=n_mels {
        let f_m_minus = bin[m - 1];
        let f_m = bin[m];
        let f_m_plus = bin[m + 1];

        for k in f_m_minus..f_m {
            if k < filterbank[m - 1].len() {
                filterbank[m - 1][k] = (k - f_m_minus) as f32 / (f_m - f_m_minus).max(1) as f32;
            }
        }
        for k in f_m..f_m_plus {
            if k < filterbank[m - 1].len() {
                filterbank[m - 1][k] = (f_m_plus - k) as f32 / (f_m_plus - f_m).max(1) as f32;
            }
        }
    }

    filterbank
}

fn apply_mel_filters(filters: &Vec<Vec<f32>>, power: &[f32]) -> Vec<f32> {
    let mut mel = vec![0.0f32; filters.len()];
    for (m, filter) in filters.iter().enumerate() {
        let mut sum = 0.0f32;
        for (k, weight) in filter.iter().enumerate() {
            if k < power.len() {
                sum += weight * power[k];
            }
        }
        mel[m] = sum;
    }
    mel
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_mel_output_size() {
        let cfg = MelConfig::default();
        let samples = vec![0.0f32; cfg.sample_rate as usize]; // 1秒
        let mel = log_mel_spectrogram(&samples, &cfg);
        assert_eq!(mel.len(), cfg.n_mels * cfg.target_frames);
    }
}

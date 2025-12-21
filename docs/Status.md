# 実装状況サマリー（2025-12-21）

## 現在のフェーズ
- Phase 5 まで完了（録音→前処理→RMS検出→Tauriイベント→UI表示）
- Phase 6 着手済み（NPU検出スタブ、モデル存在チェック）

## 実装済みの主要機能
- 音声キャプチャ: cpal 48kHz/mono、リングバッファ
- 前処理: 48kHz→16kHz 線形リサンプリング、RMS計算
- ASR基盤: WhisperModel（RMS VADスタブ）、StreamingTranscriber（5s周期/30sチャンク）
- イベント配信: `transcript_update` を Tauri → React へ送信、UI即時反映
- UI: MeetingDashboard でリアルタイム表示、SettingsPanel に NPU設定とモデル確認
- NPU検出: Windowsで DirectML.dll の存在チェック（簡易）
- モデル確認: `check_models` コマンドで `models/asr/whisper-small.onnx` の有無確認
- 永続化: WAV 保存（hound）、設定 JSON 保存

## 未実装/課題
- Whisper 実推論（Mel生成 + Encoder/Decoder + Tokenizer）
- モデル管理の拡張（必須ファイル群、ダウンロード/検証）
- Execution Provider 適用（DirectML/CUDA/CPU切替）
- 文字起こし結果の永続化（JSON/Markdown）
- UI: 録音開始前のモデル存在チェック連動

## テスト・ビルド状況
- `cargo check` (src-tauri): 成功（既知 dead_code 警告のみ）
- core crate 既存テスト: 10/10 合格（Phase 5 時点）
- pnpm はグローバル導入済み（`npm install -g pnpm` 実行済）

## 主要ファイル
- 前処理/ASR: core/src/asr/whisper.rs, core/src/asr/streaming.rs, core/src/audio/resample.rs
- Tauri commands: apps/Desktop/src-tauri/src/commands/{system,recording,transcription}.rs
- UI: apps/Desktop/src/components/{MeetingDashboard.tsx, SettingsPanel.tsx}
- モデル確認: apps/Desktop/src-tauri/src/commands/system.rs -> check_models

## 次のステップ案
1) Whisper 実推論の導入（Mel生成 + Encoder/Decoder ONNX + Tokenizer）
2) EP 適用（DirectML優先、CPUフォールバック）
3) モデル必須ファイルの拡張と録音開始前チェック
4) 文字起こし結果の永続化（JSON/Markdown）

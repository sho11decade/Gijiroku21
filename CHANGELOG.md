# Changelog

このファイルはGijiroku21プロジェクトの変更履歴を記録します。

形式は [Keep a Changelog](https://keepachangelog.com/ja/1.0.0/) に基づき、
バージョニングは [Semantic Versioning](https://semver.org/lang/ja/) に準拠します。

## [Unreleased]

### 計画中
- ONNX Runtime統合
- Whisperモデルによる音声認識
- LLM要約生成機能
- NPU検出と最適化
- 話者分離機能

## [0.1.0-alpha] - 2025-12-21

### 追加
- Rustバックエンド基盤の実装
  - 統一エラーハンドリング (`AppError`)
  - アプリケーション状態管理 (`AppState`, `MeetingState`)
  - JSON設定永続化 (`Settings`)
- 音声録音機能
  - cpalによるクロスプラットフォーム音声キャプチャ
  - リアルタイムバッファリング (`AudioBuffer`)
  - WAVファイルエクスポート
  - デバイス一覧取得
- Tauri Commands API
  - システム情報取得 (`get_system_info`)
  - 設定管理 (`get_settings`, `update_settings`)
  - NPU情報取得 (`get_npu_info`, `detect_npu`) ※スタブ
  - 録音制御 (`start_recording`, `stop_recording`, `pause_recording`, `resume_recording`)
  - 録音状態取得 (`get_recording_status`)
  - 音声デバイス一覧 (`list_audio_devices`)
- React UI統合
  - TypeScript API wrapper (`src/api/tauri.ts`)
  - 設定パネル実装（リアルAPI統合）
  - 録音ダッシュボード実装（リアルAPI統合）
- データ永続化
  - 会議データストレージ (`MeetingStorage`)
  - ローカルファイルシステム保存
  - Markdownエクスポート機能
- ドキュメント
  - Implementation.md（実装状況）
  - 更新されたREADME.md
  - Copilot開発ガイド

### 変更
- monorepo構造へのリファクタリング
  - `apps/Desktop`: Tauriアプリケーション
  - `core`: UI非依存のビジネスロジック
- cpal Send trait問題の解決
  - チャネルベース設計への移行
  - 専用スレッドでの録音処理

### 技術仕様
- サンプルレート: 48000 Hz
- チャンネル: モノラル
- 録音フォーマット: WAV (16-bit PCM)
- 保存場所: `%APPDATA%/Gijiroku21/`
- 設定ファイル: `config/settings.json`
- 会議データ: `data/meetings/{meeting_id}/`

### 既知の問題
- NPU検出が未実装（常に `available: false`）
- 文字起こし機能が未実装
- 会議履歴UIが未実装
- エクスポート形式がWAVとMarkdownのみ

### 依存関係
#### Rust
- tauri: 2.9.5
- tokio: 1.48.0
- cpal: 0.15.3
- hound: 3.5.1
- serde: 1.0.228
- thiserror: 1.0.69
- anyhow: 1.0.100

#### TypeScript/React
- react: 19.1.0
- vite: 7.0.4
- typescript: 5.7.3
- @tauri-apps/api: 2.3.1

## [0.0.1] - 2025-12-初旬

### 追加
- プロジェクト初期化
- UI設計とモックアップ
- 技術選定
- ドキュメント作成（提案書、開発計画）

[Unreleased]: https://github.com/sho11decade/Gijiroku21/compare/v0.1.0-alpha...HEAD
[0.1.0-alpha]: https://github.com/sho11decade/Gijiroku21/releases/tag/v0.1.0-alpha
[0.0.1]: https://github.com/sho11decade/Gijiroku21/releases/tag/v0.0.1

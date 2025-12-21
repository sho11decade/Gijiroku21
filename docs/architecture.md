# アーキテクチャ概要

最終更新: 2025-12-21

Gijiroku21 は、Tauri (Rust + React) をベースにした完全ローカル動作の議事録アプリです。モノレポ構成で UI 依存とコアロジックを分離し、AI 処理の置換可能性と将来拡張性を重視しています。

## ディレクトリ構成（Monorepo）
```
Gijiroku21/
├─ apps/
│  └─ Desktop/            # Tauriアプリ本体
│     ├─ src-tauri/       # Rust バックエンド
│     └─ src/             # React UI
├─ core/                  # UI非依存のビジネスロジック（将来）
├─ models/                # AIモデル格納（Git管理外）
├─ docs/                  # 設計・実装ドキュメント
└─ tests/                 # 統合/回帰テスト
```

## データフロー
```
Audio Capture (48kHz)
  → RingBuffer
  → (5s interval) ASR Preprocess (48k→16k)
  → ASR (Whisper / ONNX) [Phase 8]
  → LLM Summarize [Phase 10+]
  → Minutes Structuring → Export (JSON/Markdown/PDF)
```

- ハードウェア検出は起動時に実行（`NPU/GPU/CPU`）
- 非同期パイプラインで UI へ進捗通知（Tauri Events）

## バックエンド構成（計画）
```
apps/Desktop/src-tauri/src/
├─ main.rs
├─ lib.rs
├─ commands/            # UIから呼ばれるAPI
├─ pipeline/            # 音声→文字→要約の非同期処理
├─ state/               # アプリ・会議の状態管理
└─ error.rs             # 統一エラー型
```

## Core ロジック（計画）
```
core/
├─ audio/               # キャプチャ/バッファ/前処理
├─ asr/                 # Whisper/ONNX 推論
├─ summarizer/          # LLM要約
├─ npu/                 # 検出/最適化
└─ storage/             # 永続化/エクスポート
```

## 実行プロバイダ（EP）選択（概要）
- 優先度: DirectML > CUDA > CPU
- Phase 6 で検出、Phase 8 で推論時に適用

## 参考リンク
- DevelopmentPlan: ./DevelopmentPlan.md
- Implementation: ./Implementation.md
- NPU Strategy: ./npu_strategy.md

# Gijiroku21

[![開発状態](https://img.shields.io/badge/状態-アルファ版-yellow)](https://github.com/sho11decade/Gijiroku21)
[![バージョン](https://img.shields.io/badge/version-0.1.0--alpha-blue)](https://github.com/sho11decade/Gijiroku21)
[![License](https://img.shields.io/badge/license-未定-lightgrey)](./LICENSE)

## 概要

Gijiroku21 は、**完全ローカル動作**する高性能な議事録作成デスクトップアプリケーションです。Tauri フレームワーク（Rust + React）を採用し、NPUを活用したAI音声認識（ASR）と大規模言語モデル（LLM）により、プライバシーを保護しながら高精度な議事録生成を実現します。

**重要**: すべての処理はローカルで完結し、音声データやテキストはクラウドに送信されません。

## 主な特徴

- 🔒 **完全ローカル動作**: インターネット接続不要、クラウド通信なし
- 🎙️ **リアルタイム音声認識**: ONNX + Whisperモデルによる高精度文字起こし（予定）
- 🤖 **AI要約生成**: 大規模言語モデルによる議事録構造化（予定）
- ⚡ **NPU活用**: NPUアクセラレーションで効率的な処理（予定）
- 🎨 **モダンUI**: React + Radix UIによる直感的なインターフェース
- 💾 **データ永続化**: ローカルストレージに安全に保存
- 🌐 **多言語対応**: 日本語を中心に複数言語サポート予定

## 現在の実装状況（2025-12-21）

### ✅ 実装完了

#### Phase 1-4: 基本インフラ + UI統合
- [x] Rustバックエンド基盤（エラーハンドリング、状態管理）
- [x] 音声キャプチャ機能（cpal 0.15.3、48kHz モノラル）
- [x] 録音開始/停止/一時停止/再開 API
- [x] WAVファイルエクスポート（hound 3.5.1）
- [x] 設定管理（JSON永続化）
- [x] React UI統合（録音画面、設定画面）
- [x] Tauri Event統合（リアルタイム更新）

#### Phase 5: 実動作基盤
- [x] ONNX Runtime統合（onnxruntime 0.0.14）
- [x] 音声リサンプリング（48kHz → 16kHz）
- [x] ASR基本モジュール（トレイト定義）
- [x] WhisperModel実装（RMS VAD音声区間検出）
- [x] ストリーミング処理パイプライン（5秒間隔）
- [x] UI⇔ASR間のイベント通信
- [x] 完全エンドツーエンドテスト（10/10 passing）

### 🚧 開発中・計画中

#### Phase 6-8: 高度な機能
- [ ] NPU検出・DirectML最適化
- [ ] Whisper ONNXモデル管理・ダウンロード
- [ ] 実際のWhisper推論（メルスペクトログラム + Encoder/Decoder）
- [ ] 議事録永続化（JSON/Markdown保存）
- [ ] LLM要約生成
- [ ] 話者分離

詳細は [Implementation.md](./docs/Implementation.md) を参照してください。

## 技術スタック

- **バックエンド**: Rust 1.70+, Tauri 2.9.5
- **フロントエンド**: React 19, TypeScript 5, Vite 7
- **音声処理**: cpal 0.15.3, hound 3.5.1
- **非同期**: Tokio 1.48.0
- **UI**: Radix UI (shadcn/ui), Framer Motion
- **予定**: ONNX Runtime, DirectML, llama.cpp

## セットアップ

### 必須要件
- Rust 1.70+ ([rustup](https://rustup.rs/)でインストール)
- Node.js 18+ & pnpm
- Tauri CLI: `cargo install tauri-cli --version "^2.0.0"`
- Windows SDK（Windows開発時）

### インストール手順

```powershell
# リポジトリをクローン
git clone https://github.com/sho11decade/Gijiroku21.git
cd Gijiroku21

# フロントエンドの依存関係をインストール
cd apps/Desktop
pnpm install

# 開発サーバーを起動
pnpm tauri dev
```

### ビルド

```powershell
pnpm tauri build
```

## 実装フェーズ

### Phase 1-2: 基本インフラ ✅
- システム情報、設定管理
- 音声キャプチャ、リングバッファ

### Phase 3-4: UI統合 ✅
- React コンポーネント
- Tauri Command API
- リアルタイムイベント通信

### Phase 5: 実動作（完成） ✅
- ONNX Runtime環境構築
- 音声前処理（リサンプリング、正規化）
- ASR基本パイプライン（RMS VAD）
- **エンドツーエンド動作確認完了**

### Phase 6+: 高度な機能 🚧
- NPU最適化、モデル管理
- 実Whisper推論、要約生成
- 永続化、エクスポート

## 使い方

1. アプリを起動
2. マイク権限を許可（初回のみ）
3. 会議名を入力
4. 「録音開始」ボタンをクリック
5. 会議終了後「録音停止」で自動保存

録音データは `%APPDATA%/Gijiroku21/data/meetings/` に保存されます。

## プロジェクト構成

```
Gijiroku21/
├── apps/Desktop/        # Tauriアプリ本体
│   ├── src-tauri/      # Rustバックエンド
│   └── src/            # React UI
├── core/               # 共有Rustライブラリ
├── models/             # AIモデル（予定）
├── docs/               # ドキュメント
└── tests/              # テスト
```

## ドキュメント

- [Implementation.md](./docs/Implementation.md) - 実装状況の詳細
- [DevelopmentPlan.md](./docs/DevelopmentPlan.md) - 開発計画と設計
- [architecture.md](./docs/architecture.md) - アーキテクチャ概要（要点）
- [npu_strategy.md](./docs/npu_strategy.md) - NPU検出と最適化方針（Phase 6）
- [proposal.md](./docs/proposal.md) - プロジェクト提案書
- [RecommendationTech.md](./docs/RecommendationTech.md) - 技術選定理由
- [.github/copilot-instructions.md](./.github/copilot-instructions.md) - AI開発ガイド

## トラブルシューティング

### ビルドエラー
- **cpalのコンパイルが失敗する**: Windows SDKをインストールしてください
- **Tauri CLIが見つからない**: `cargo install tauri-cli --version "^2.0.0"`を実行

### 録音できない
- マイクの権限を確認（Windows設定 → プライバシー → マイク）
- 他のアプリケーションがマイクを使用していないか確認

### 設定が保存されない
- `%APPDATA%/Gijiroku21/config/`への書き込み権限を確認

詳細は [Implementation.md](./docs/Implementation.md) のトラブルシューティングセクションを参照してください。

## ロードマップ

- [x] **Phase 1**: 基本インフラ（完了）
- [x] **Phase 2**: 音声録音機能（完了）
- [ ] **Phase 3**: ASR統合（開発中）
- [ ] **Phase 4**: LLM要約機能
- [ ] **Phase 5**: NPU最適化
- [ ] **Phase 6**: 高度な機能（検索、履歴管理）

## ライセンス

未定（開発中）

## 貢献

現在アクティブに開発中です。Issue や Pull Request を歓迎します。

## お問い合わせ

プロジェクトに関する質問は GitHub Issues でお願いします。

---

**注意**: このプロジェクトはアルファ版です。本番環境での使用は推奨されません。

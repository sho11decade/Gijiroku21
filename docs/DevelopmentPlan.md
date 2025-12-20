以下では、**Gijiroku21 を実装する前提での「現実的かつスケール可能な開発ディレクトリ構成」**と、
**開発フロー・役割分離・将来拡張を見据えた設計指針**を整理します。

前提：

* **Tauri + Rust（Core） + Web UI（React想定）**
* AI処理は **ONNX / llama.cpp / Whisper.cpp**
* 個人〜小規模チーム開発を想定

---

## 1. リポジトリ全体構成（Monorepo）

```
gijiroku21/
├─ README.md
├─ LICENSE
├─ .gitignore
├─ .editorconfig
├─ docs/                 # 設計・仕様ドキュメント
│  ├─ architecture.md
│  ├─ npu_strategy.md
│  ├─ privacy_policy.md
│  └─ ui_guidelines.md
│
├─ apps/
│  ├─ desktop/           # デスクトップアプリ本体（Tauri）
│  │  ├─ src-tauri/      # Rustバックエンド
│  │  └─ web/            # Web UI
│  │
│  └─ cli/               # 将来用CLI（任意）
│
├─ core/                 # UI非依存の中核ロジック
│  ├─ audio/
│  ├─ asr/
│  ├─ llm/
│  ├─ summarizer/
│  ├─ npu/
│  └─ storage/
│
├─ models/               # AIモデル管理
│  ├─ asr/
│  └─ llm/
│
├─ scripts/              # 開発・ビルド補助
│
└─ tests/                # 統合テスト
```

---

## 2. Tauriアプリ内部構成

### `apps/desktop/`

```
apps/desktop/
├─ src-tauri/
│  ├─ src/
│  │  ├─ main.rs
│  │  ├─ commands/       # UIから呼ばれるAPI
│  │  ├─ state/          # アプリ状態管理
│  │  ├─ pipeline/       # 音声→文字→議事録
│  │  └─ error.rs
│  │
│  ├─ Cargo.toml
│  └─ tauri.conf.json
│
└─ web/
   ├─ src/
   │  ├─ components/
   │  ├─ pages/
   │  ├─ store/
   │  ├─ hooks/
   │  ├─ api/
   │  └─ styles/
   ├─ index.html
   ├─ package.json
   └─ vite.config.ts
```

---

## 3. Rustバックエンド詳細設計

### `src-tauri/src/`

```
src/
├─ main.rs
├─ commands/
│  ├─ recording.rs       # 録音開始・停止
│  ├─ transcription.rs  # ASR制御
│  ├─ summary.rs        # 要約生成
│  ├─ export.rs
│  └─ system.rs         # NPU状態取得等
│
├─ pipeline/
│  ├─ audio_pipeline.rs
│  ├─ asr_pipeline.rs
│  ├─ llm_pipeline.rs
│  └─ meeting_pipeline.rs
│
├─ state/
│  ├─ app_state.rs
│  ├─ meeting_state.rs
│  └─ settings.rs
│
├─ error.rs
└─ lib.rs
```

### 設計方針

* **commands = UI API**
* **pipeline = 非同期処理本体**
* **state = 状態と設定のみ**
* ビジネスロジックは `core/` に逃がす

---

## 4. Coreロジック（UI非依存）

### `core/` ディレクトリ

```
core/
├─ audio/
│  ├─ capture.rs         # WASAPI等
│  ├─ buffer.rs
│  └─ preprocess.rs
│
├─ asr/
│  ├─ whisper.rs
│  ├─ onnx.rs
│  └─ segment.rs
│
├─ llm/
│  ├─ model.rs
│  ├─ prompt.rs
│  └─ inference.rs
│
├─ summarizer/
│  ├─ minutes.rs        # 議事録構造化
│  └─ keywords.rs
│
├─ npu/
│  ├─ detect.rs
│  ├─ directml.rs
│  └─ fallback.rs
│
├─ storage/
│  ├─ meeting_fs.rs
│  ├─ export.rs
│  └─ index.rs
│
└─ lib.rs
```

### 利点

* CLI / サーバ / 将来のWeb版でも再利用可
* テストが書きやすい
* AI更新時の影響範囲が限定される

---

## 5. Web UI 構成（React例）

### `apps/desktop/web/src/`

```
src/
├─ pages/
│  ├─ Home.tsx
│  ├─ Meeting.tsx
│  └─ Settings.tsx
│
├─ components/
│  ├─ RecorderPanel.tsx
│  ├─ TranscriptView.tsx
│  ├─ SummaryView.tsx
│  └─ NpuIndicator.tsx
│
├─ api/
│  └─ tauri.ts           # invokeラッパー
│
├─ store/
│  ├─ meetingStore.ts
│  └─ settingsStore.ts
│
├─ hooks/
│  ├─ useRecording.ts
│  └─ useTranscription.ts
│
└─ styles/
```

### UI責務を明確化

* UIは「表示と操作のみ」
* AI判断・処理は一切持たせない

---

## 6. モデル管理方針

### `models/`

```
models/
├─ asr/
│  ├─ whisper-small.onnx
│  └─ README.md
│
└─ llm/
   ├─ phi-3-q4.gguf
   └─ README.md
```

### ルール

* Git管理しない（LFSも非推奨）
* 初回起動時DL or 手動配置
* UIで明示的に管理

---

## 7. 開発フロー（現実的）

### 初期MVP

1. 録音 → WAV保存
2. Whisper small（CPU）
3. 手動要約（テンプレ）

### フェーズ2

* ONNX化
* NPU検出
* リアルタイムASR

### フェーズ3

* LLM要約
* 話者分離
* 高度UI

---

## 8. テスト戦略

```
tests/
├─ audio_test.rs
├─ asr_test.rs
└─ pipeline_test.rs
```

* AI部分は **ゴールデンテキスト比較**
* UIはE2E最小限

---

## 9. 将来拡張を見据えた設計原則

* **すべての処理は「置き換え可能」**
* モデル・HW依存コードは隔離
* UIとCoreを強く分離

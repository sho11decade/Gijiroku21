# Gijiroku21 推奨技術構成ガイド
## 1. 全体アーキテクチャ概要

### レイヤ構成（推奨）

```
┌────────────────────────────┐
│ UIレイヤ                    │
│  Web UI (React / Vue)       │
└───────────▲────────────────┘
            │ IPC / Local API
┌───────────┴────────────────┐
│ アプリ制御レイヤ            │
│  Rust / C++ (Core)          │
│  ・状態管理                 │
│  ・音声パイプライン制御     │
│  ・NPU / HW抽象化           │
└───────────▲────────────────┘
            │ FFI / C-API
┌───────────┴────────────────┐
│ AI・信号処理レイヤ          │
│  ・音声認識（Whisper系）   │
│  ・話者分離                 │
│  ・要約 / LLM               │
│  ・NPUアクセラレーション   │
└───────────▲────────────────┘
            │
┌───────────┴────────────────┐
│ OS / Hardware               │
│  CPU / GPU / NPU            │
└────────────────────────────┘
```

---

## 2. UIレイヤ構成

### 技術選定

| 項目             | 推奨                       |
| ---------------- | -------------------------- |
| UIフレームワーク | **React** または **Vue 3** |
| UIランタイム     | **Tauri**（強く推奨）      |
| 状態管理         | Zustand / Pinia            |
| 描画             | HTML/CSS（Canvas最小限）   |

### Tauriを推奨する理由

* Electronより**軽量・高速**
* Rustバックエンドと親和性が高い
* **ローカル・オフライン前提設計**
* セキュリティ（IPC制御）が明確

---

## 3. UI ↔ バックエンド接続方式

### 通信方式

#### 基本

* **Tauri IPC（invoke / command）**

```
UI (JS/TS)
  ↓ invoke()
Rust Backend
  ↓ 処理
結果をJSONで返却
```

#### 例

```ts
await invoke("start_recording", { meeting_id })
```

```rust
#[tauri::command]
fn start_recording(meeting_id: String) {
    audio_pipeline.start(meeting_id);
}
```

### 特徴

* HTTPサーバ不要
* localhost通信すら不要（完全ローカル）
* プライバシー面で非常に強い

---

## 4. バックエンド（コア）構成

### 開発言語

**Rust（第一候補）**

* 安全性・並列性
* 音声処理・AI推論の制御に適する
* Tauri標準言語

※ 一部高速処理は **C++** を併用可（FFI）

---

### バックエンド責務

| 機能           | 内容               |
| -------------- | ------------------ |
| 音声取得       | WASAPI / CoreAudio |
| 音声ストリーム | RingBuffer管理     |
| 音声前処理     | ノイズ除去・正規化 |
| AI推論制御     | ASR / LLM          |
| NPU制御        | DirectML / ONNX    |
| 状態管理       | 会議状態・UI通知   |

---

## 5. AI・音声認識レイヤ

### 音声認識（ASR）

| 要素           | 推奨                  |
| -------------- | --------------------- |
| モデル         | Whisper / Whisper.cpp |
| 実行           | ONNX Runtime          |
| 精度切替       | small / medium        |
| ストリーミング | Chunk分割             |

### 話者分離（任意）

* pyannote.audio（軽量化要）
* または簡易エネルギーベース

---

## 6. LLM・要約処理

### ローカルLLM構成

| 項目           | 推奨                   |
| -------------- | ---------------------- |
| モデル         | Phi-3 / Qwen / Llama系 |
| 量子化         | int8 / int4            |
| 実行           | llama.cpp / ONNX       |
| 使用タイミング | 会議後                 |

### UI思想に合わせた設計

* 自動要約は**即時実行しない**
* ユーザー操作で明示的に実行
* AI生成部分はUIで明確区別

---

## 7. NPU活用戦略（差別化の核）

### Windows（Copilot+ PC）

* **ONNX Runtime + DirectML**
* NPUがあれば自動使用
* なければGPU → CPUへフォールバック

```text
NPU → GPU → CPU（自動）
```

### UI連動

* 「NPU使用中」表示
* フォールバック時は表示変更

---

## 8. データ保存設計

### 保存場所

* ユーザーローカルのみ

```
/Documents/AppName/
 ├ meetings/
 │ ├ 2025-01-01/
 │ │ ├ audio.wav
 │ │ ├ transcript.json
 │ │ └ summary.md
```

### フォーマット

* 音声：WAV / FLAC
* 文字起こし：JSON
* 議事録：Markdown

→ **可搬性・透明性が高い**

---

## 9. セキュリティ・プライバシー設計

* 外部通信コードを原則排除
* 通信発生時はUIで明示
* ログはローカルのみ
* OS権限は最小限

---

## 10. 技術スタックまとめ

| レイヤ   | 技術             |
| -------- | ---------------- |
| UI       | React / Vue      |
| App      | Tauri            |
| Backend  | Rust             |
| 高速処理 | C++（任意）      |
| ASR      | Whisper + ONNX   |
| LLM      | llama.cpp / ONNX |
| HW加速   | DirectML / NPU   |
| 保存     | ローカルFS       |
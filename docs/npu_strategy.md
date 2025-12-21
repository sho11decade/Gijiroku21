# NPU戦略（Phase 6）

最終更新: 2025-12-21

本ドキュメントは、Gijiroku21 における NPU 検出と推論アクセラレーション方針を定義します。対象は主に Windows 環境（DirectML）ですが、将来的に macOS (Metal) / Linux (CUDA/Vulkan) への拡張も考慮します。

## 目的
- 利用可能なハードウェア（NPU/GPU/CPU）を自動検出し、最適な実行プロバイダ（Execution Provider, EP）を選択
- 既存コードへの侵入を最小化したプラガブル設計
- フォールバック戦略による確実な実行

## スコープ
- ONNX Runtime での EP 選択（DirectML 優先）
- Tauri 起動時にハードウェア検出し、UIへ公開
- Whisper 推論時に選択 EP を適用（Phase 8 で利用）

## 優先順位とポリシー
1. DirectML (Windows, GPU/NPU)
2. CUDA (Windows/Linux, NVIDIA)
3. CPU (全プラットフォーム)

注: 現時点（Phase 6）では DirectML の有無検出と設定反映までを実施。実推論での EP 適用は Phase 8 で有効化します。

## 実装計画

### 1. 検出
- Windows: 以下の順で判定
  - `DirectML.dll` の存在チェック（`System32` / `SysWOW64`）
  - WMI `Win32_VideoController` から GPU 情報取得（ベンダ/ドライバ）
  - OS ビルド判定（Windows 11 現行）
- macOS: Metal 対応は将来（未実装）
- Linux: CUDA/Vulkan は将来（未実装）

### 2. 公開インターフェース
- core/npu/detect.rs
  - `HardwareAccel { ep: ExecProvider, device_name: String, available: bool }`
  - `detect_accel() -> HardwareAccel`
- apps/Desktop/src-tauri/src/commands/system.rs
  - `get_npu_info() -> HardwareAccel`
  - `detect_npu() -> HardwareAccel`（再スキャン）

### 3. 設定反映
- 起動時に `AppState` に保存（`selected_ep`）
- UI の `NpuIndicator` で表示（将来）

## データモデル
```rust
pub enum ExecProvider {
    DirectML,
    Cuda,
    Cpu,
}

pub struct HardwareAccel {
    pub ep: ExecProvider,
    pub device_name: String,
    pub available: bool,
}
```

## フォールバック戦略
- DirectML 不可 -> CUDA 判定
- CUDA 不可 -> CPU 強制
- いずれもエラーとせず、`available=false` でも CPU で処理継続

## ONNX Runtime への適用（Phase 8 予定）
- Whisper 推論（Encoder/Decoder）セッション生成時に EP を選択
- 例: `SessionOptions` に DirectML を追加（onnxruntime 0.0.14 の API 制約に応じた実装）

## 既知の課題
- onnxruntime 0.0.14 は EP 選択 API が限定的
- Windows での NPU 検出は正式 API が乏しく、間接的な判定が中心
- ベンダ毎の最適化（Intel/NVIDIA/AMD）の調整は Phase 8+ で対応

## 検証項目（Phase 6 完了条件）
- [ ] `detect_accel()` が Windows で DirectML 有無を判定
- [ ] Tauri コマンドで結果を取得し UI ログに表示
- [ ] CPU フォールバック時に ASR パイプラインが問題なく動作

## 参考
- ONNX Runtime Execution Providers: https://onnxruntime.ai/docs/execution-providers/
- DirectML: https://learn.microsoft.com/windows/ai/directml/dml-intro
- WMI `Win32_VideoController`: https://learn.microsoft.com/windows/win32/cimwin32prov/win32-videocontroller

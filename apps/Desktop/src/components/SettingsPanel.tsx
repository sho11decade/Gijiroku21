import { useState, useEffect } from "react";
import { HelpCircle, Cpu, Zap, BarChart3, Download } from "lucide-react";
import { ToastType } from "./Toast";
import { Tooltip } from "./Tooltip";
import { getSettings, updateSettings, getNpuInfo, checkModels, downloadModels, type Settings, type NpuInfo, type ModelCheck, type DownloadResult } from "../api/tauri";

export function SettingsPanel({
  onToast,
}: {
  onToast: (type: ToastType, message: string) => void;
}) {
    const [settings, setSettings] = useState<Settings>({
      use_npu: true,
      asr_model_size: "small",
      use_llm: true,
      auto_save: true,
      save_directory: null,
      model_directory: null,
      tokenizer_directory: null,
    });
  
  const [npuInfo, setNpuInfo] = useState<NpuInfo | null>(null);
  const [loading, setLoading] = useState(true);

  const [showPerformance, setShowPerformance] = useState(false);

  // 初回ロード時に設定とNPU情報を取得
  useEffect(() => {
    const loadSettings = async () => {
      try {
        const [settingsData, npuData] = await Promise.all([
          getSettings(),
          getNpuInfo(),
        ]);
        setSettings(settingsData);
        setNpuInfo(npuData);
      } catch (error) {
        console.error("Failed to load settings:", error);
        onToast("error", "設定の読み込みに失敗しました");
      } finally {
        setLoading(false);
      }
    };
    loadSettings();
  }, [onToast]);

  const handleSettingsUpdate = async (newSettings: Settings) => {
    try {
      await updateSettings(newSettings);
      setSettings(newSettings);
      onToast("success", "設定を更新しました");
    } catch (error) {
      console.error("Failed to update settings:", error);
      onToast("error", "設定の更新に失敗しました");
    }
  };

  const toggleNpu = () => {
    handleSettingsUpdate({ ...settings, use_npu: !settings.use_npu });
  };

  const toggleLlm = () => {
    handleSettingsUpdate({ ...settings, use_llm: !settings.use_llm });
  };

  const toggleAutoSave = () => {
    handleSettingsUpdate({ ...settings, auto_save: !settings.auto_save });
  };

  const handleCheckModels = async () => {
    try {
      const result: ModelCheck = await checkModels();
      if (result.ok) {
        onToast("success", `モデルOK: ${result.model_dir}`);
      } else {
        onToast("warning", `不足モデル: ${result.missing.join(", ")}`);
      }
    } catch (e) {
      console.error("checkModels failed", e);
      onToast("error", "モデル確認に失敗しました");
    }
  };

  const handleModelDirChange = (value: string) => {
    const updated = { ...settings, model_directory: value.trim() === "" ? null : value };
    handleSettingsUpdate(updated);
  };

  const handleTokenizerDirChange = (value: string) => {
    const updated = { ...settings, tokenizer_directory: value.trim() === "" ? null : value };
    handleSettingsUpdate(updated);
  };

  const handleDownloadModels = async () => {
    try {
      onToast("info", "モデルのダウンロードを開始します...");
      const result: DownloadResult = await downloadModels();

      if (result.ok) {
        onToast("success", "モデルのダウンロードが完了しました");
      } else {
        onToast("warning", `一部のモデルのダウンロードに失敗しました: ${result.failed.join("; ")}`);
      }

      // ダウンロード後に存在チェックも実行
      try {
        const check = await checkModels();
        if (check.ok) {
          onToast("success", `モデルOK: ${check.model_dir}`);
        } else {
          onToast("warning", `不足モデル: ${check.missing.join(", ")}`);
        }
      } catch (e) {
        console.error("checkModels after download failed", e);
      }
    } catch (e) {
      console.error("downloadModels failed", e);
      onToast("error", "モデルのダウンロードに失敗しました");
    }
  };

  if (loading) {
    return (
      <div className="max-w-4xl mx-auto p-6">
        <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
          <p className="text-gray-600">設定を読み込んでいます...</p>
        </div>
      </div>
    );
  }

  const performanceData = {
    audioToText: 0.8,
    textAnalysis: 0.3,
    summaryGeneration: 1.2,
    cpuUsage: 15,
    npuUsage: 85,
  };

  return (
    <div className="max-w-4xl mx-auto p-6">
      {/* 基本設定 */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6 mb-6">
        <h2 className="text-gray-900 mb-6">基本設定</h2>

        <div className="space-y-6">
          {/* 自動要約 */}
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-1">
                <label className="text-gray-900">
                  LLM要約機能を使用する
                </label>
                <button className="text-gray-400 hover:text-gray-600">
                  <HelpCircle className="w-4 h-4" />
                </button>
              </div>
              <p className="text-sm text-gray-600">
                会議終了後、LLMによる自動要約を生成します。あとから自由に編集できます。
              </p>
            </div>
            <button
              onClick={toggleLlm}
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ml-4 ${
                settings.use_llm
                  ? "bg-blue-600"
                  : "bg-gray-300"
              }`}
            >
              <span
                className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  settings.use_llm
                    ? "translate-x-6"
                    : "translate-x-1"
                }`}
              />
            </button>
          </div>

          {/* 自動保存 */}
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-1">
                <label className="text-gray-900">
                  自動保存
                </label>
                <button className="text-gray-400 hover:text-gray-600">
                  <HelpCircle className="w-4 h-4" />
                </button>
              </div>
              <p className="text-sm text-gray-600">
                会議内容を自動的に保存します。
              </p>
            </div>
            <button
              onClick={toggleAutoSave}
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ml-4 ${
                settings.auto_save
                  ? "bg-blue-600"
                  : "bg-gray-300"
              }`}
            >
              <span
                className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  settings.auto_save
                    ? "translate-x-6"
                    : "translate-x-1"
                }`}
              />
            </button>
          </div>
        </div>
      </div>

      {/* NPU設定 / モデル */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6 mb-6">
        <div className="flex items-center gap-2 mb-6">
          <Cpu className="w-5 h-5 text-blue-600" />
          <h2 className="text-gray-900">NPU高速化 / モデル</h2>
        </div>

        <div className="flex items-start justify-between mb-4">
          <div className="flex-1">
            <div className="flex items-center gap-2 mb-1">
              <label className="text-gray-900">
                NPU（AI専用チ��プ）を使用する
              </label>
              <button className="text-gray-400 hover:text-gray-600">
                <HelpCircle className="w-4 h-4" />
              </button>
            </div>
            <p className="text-sm text-gray-600">
              NPUを使用することで、CPUへの負荷を減らしながら高速に処理できます。
            </p>
            {npuInfo && (
              <p className="text-xs text-gray-500 mt-1">
                検出されたNPU: {npuInfo.available ? (npuInfo.device_name || "利用可能") : "利用不可"}
              </p>
            )}
          </div>
          <button
            onClick={toggleNpu}
            className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ml-4 ${
              settings.use_npu
                ? "bg-blue-600"
                : "bg-gray-300"
            }`}
          >
            <span
              className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                settings.use_npu
                  ? "translate-x-6"
                  : "translate-x-1"
              }`}
            />
          </button>
        </div>

        {settings.use_npu && npuInfo?.available && (
          <div className="p-4 bg-green-50 border border-green-200 rounded-lg">
            <div className="flex items-center gap-2 text-green-800 mb-2">
              <Zap className="w-4 h-4" />
              <span className="text-sm">NPUが有効です</span>
            </div>
            <p className="text-xs text-green-700">
              AI処理が高速化され、バッテリー消費も抑えられます。
            </p>
          </div>
        )}
        
        {settings.use_npu && !npuInfo?.available && (
          <div className="p-4 bg-yellow-50 border border-yellow-200 rounded-lg">
            <div className="flex items-center gap-2 text-yellow-800 mb-2">
              <HelpCircle className="w-4 h-4" />
              <span className="text-sm">NPUが検出されませんでした</span>
            </div>
            <p className="text-xs text-yellow-700">
              CPUまたはGPUで処理を行います。
            </p>
          </div>
        )}

        <div className="mt-4 space-y-2">
          <label className="text-sm text-gray-700">モデルディレクトリ</label>
          <input
            type="text"
            placeholder="(未指定はプロジェクト直下 models/asr)"
            value={settings.model_directory ?? ""}
            onChange={(e) => handleModelDirChange(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <label className="text-sm text-gray-700">Tokenizerディレクトリ</label>
          <input
            type="text"
            placeholder="(未指定はプロジェクト直下 models/tokenizer)"
            value={settings.tokenizer_directory ?? ""}
            onChange={(e) => handleTokenizerDirChange(e.target.value)}
            className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
          />
          <div className="flex items-center gap-3 mt-2">
            <button
              onClick={handleCheckModels}
              className="px-4 py-2 rounded-lg border border-gray-300 text-gray-700 hover:bg-gray-50"
            >
              モデル確認
            </button>
            <button
              onClick={handleDownloadModels}
              className="inline-flex items-center gap-1 px-4 py-2 rounded-lg bg-blue-600 text-white text-sm hover:bg-blue-700"
            >
              <Download className="w-4 h-4" />
              モデルを自動ダウンロード
            </button>
            <span className="text-xs text-gray-500">
              未指定時の既定: ASR → models/asr, Tokenizer → models/tokenizer
            </span>
          </div>
        </div>
      </div>

      {/* パフォーマンス可視化 */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
        <div className="flex items-center justify-between mb-6">
          <div className="flex items-center gap-2">
            <BarChart3 className="w-5 h-5 text-blue-600" />
            <h2 className="text-gray-900">
              パフォーマンス統計
            </h2>
          </div>
          <button
            onClick={() => setShowPerformance(!showPerformance)}
            className="text-sm text-blue-600 hover:text-blue-700"
          >
            {showPerformance ? "非表示" : "表示"}
          </button>
        </div>

        {showPerformance && (
          <div className="space-y-6">
            {/* 処理時間 */}
            <div>
              <h3 className="text-sm text-gray-700 mb-3">
                平均処理時間
              </h3>
              <div className="space-y-3">
                <div>
                  <div className="flex justify-between text-sm mb-1">
                    <span className="text-gray-600">
                      音声 → テキスト
                    </span>
                    <span className="text-gray-900">
                      {performanceData.audioToText}秒
                    </span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div
                      className="bg-blue-600 h-2 rounded-full"
                      style={{
                        width: `${(performanceData.audioToText / 2) * 100}%`,
                      }}
                    ></div>
                  </div>
                </div>
                <div>
                  <div className="flex justify-between text-sm mb-1">
                    <span className="text-gray-600">
                      テキスト分析
                    </span>
                    <span className="text-gray-900">
                      {performanceData.textAnalysis}秒
                    </span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div
                      className="bg-blue-600 h-2 rounded-full"
                      style={{
                        width: `${(performanceData.textAnalysis / 2) * 100}%`,
                      }}
                    ></div>
                  </div>
                </div>
                <div>
                  <div className="flex justify-between text-sm mb-1">
                    <span className="text-gray-600">
                      要約生成
                    </span>
                    <span className="text-gray-900">
                      {performanceData.summaryGeneration}秒
                    </span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div
                      className="bg-blue-600 h-2 rounded-full"
                      style={{
                        width: `${(performanceData.summaryGeneration / 2) * 100}%`,
                      }}
                    ></div>
                  </div>
                </div>
              </div>
            </div>

            {/* CPU/NPU使用率 */}
            <div>
              <h3 className="text-sm text-gray-700 mb-3">
                処理分担
              </h3>
              <div className="space-y-3">
                <div>
                  <div className="flex justify-between text-sm mb-1">
                    <span className="text-gray-600">
                      CPU使用
                    </span>
                    <span className="text-gray-900">
                      {performanceData.cpuUsage}%
                    </span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div
                      className="bg-orange-500 h-2 rounded-full"
                      style={{
                        width: `${performanceData.cpuUsage}%`,
                      }}
                    ></div>
                  </div>
                </div>
                <div>
                  <div className="flex justify-between text-sm mb-1">
                    <span className="text-gray-600">
                      NPU使用
                    </span>
                    <span className="text-gray-900">
                      {performanceData.npuUsage}%
                    </span>
                  </div>
                  <div className="w-full bg-gray-200 rounded-full h-2">
                    <div
                      className="bg-green-600 h-2 rounded-full"
                      style={{
                        width: `${performanceData.npuUsage}%`,
                      }}
                    ></div>
                  </div>
                </div>
              </div>
            </div>

            <div className="p-3 bg-blue-50 border border-blue-200 rounded-lg">
              <p className="text-xs text-blue-800">
                💡
                NPUを活用することで、AI処理の大部分をCPUから分離し、軽快な動作を実現しています。
              </p>
            </div>
          </div>
        )}
      </div>

      {/* プライバシー情報 */}
      <div className="mt-6 bg-green-50 border-2 border-green-600 rounded-lg p-6">
        <h3 className="text-green-900 mb-3">
          プライバシーポリシー
        </h3>
        <ul className="space-y-2 text-sm text-green-800">
          <li className="flex items-start gap-2">
            <span className="text-green-600 mt-0.5">✓</span>
            <span>
              すべての音声データはこのPC内でのみ処理されます
            </span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-green-600 mt-0.5">✓</span>
            <span>
              クラウドサーバーへのアップロードは一切行いません
            </span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-green-600 mt-0.5">✓</span>
            <span>
              インターネット接続なしでも完全に動作します
            </span>
          </li>
          <li className="flex items-start gap-2">
            <span className="text-green-600 mt-0.5">✓</span>
            <span>データはあなたが管理・削除できます</span>
          </li>
        </ul>
      </div>
    </div>
  );
}
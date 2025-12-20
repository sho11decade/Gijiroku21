import { useState } from "react";
import { HelpCircle, Cpu, Zap, BarChart3 } from "lucide-react";
import { ToastType } from "./Toast";
import { Tooltip } from "./Tooltip";

export function SettingsPanel({
  onToast,
}: {
  onToast: (type: ToastType, message: string) => void;
}) {
  const [settings, setSettings] = useState({
    speakerSeparation: true,
    keyPointExtraction: true,
    autoSummary: false,
    npuAcceleration: true,
  });

  const [showPerformance, setShowPerformance] = useState(false);

  const toggleSetting = (key: keyof typeof settings) => {
    setSettings((prev) => ({ ...prev, [key]: !prev[key] }));
    onToast("success", "設定を更新しました");
  };

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
          {/* 発言者分離 */}
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-1">
                <label className="text-gray-900">
                  発言者分離を使う
                </label>
                <button className="text-gray-400 hover:text-gray-600">
                  <HelpCircle className="w-4 h-4" />
                </button>
              </div>
              <p className="text-sm text-gray-600">
                会議中の複数の発言者を自動で識別します。より正確な議事録作成に役立ちます。
              </p>
            </div>
            <button
              onClick={() => toggleSetting("speakerSeparation")}
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ml-4 ${
                settings.speakerSeparation
                  ? "bg-blue-600"
                  : "bg-gray-300"
              }`}
            >
              <span
                className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  settings.speakerSeparation
                    ? "translate-x-6"
                    : "translate-x-1"
                }`}
              />
            </button>
          </div>

          {/* 要点抽出 */}
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-1">
                <label className="text-gray-900">
                  要点抽出を使う
                </label>
                <button className="text-gray-400 hover:text-gray-600">
                  <HelpCircle className="w-4 h-4" />
                </button>
              </div>
              <p className="text-sm text-gray-600">
                会議中に重要な発言を自動でマーキングします。決定事項や要確認事項を見逃しません。
              </p>
            </div>
            <button
              onClick={() =>
                toggleSetting("keyPointExtraction")
              }
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ml-4 ${
                settings.keyPointExtraction
                  ? "bg-blue-600"
                  : "bg-gray-300"
              }`}
            >
              <span
                className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  settings.keyPointExtraction
                    ? "translate-x-6"
                    : "translate-x-1"
                }`}
              />
            </button>
          </div>

          {/* 自動要約 */}
          <div className="flex items-start justify-between">
            <div className="flex-1">
              <div className="flex items-center gap-2 mb-1">
                <label className="text-gray-900">
                  自動要約（会議後）
                </label>
                <button className="text-gray-400 hover:text-gray-600">
                  <HelpCircle className="w-4 h-4" />
                </button>
              </div>
              <p className="text-sm text-gray-600">
                会議終了後、自動で要約を生成します。あとから自由に編集できます。
              </p>
            </div>
            <button
              onClick={() => toggleSetting("autoSummary")}
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ml-4 ${
                settings.autoSummary
                  ? "bg-blue-600"
                  : "bg-gray-300"
              }`}
            >
              <span
                className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  settings.autoSummary
                    ? "translate-x-6"
                    : "translate-x-1"
                }`}
              />
            </button>
          </div>
        </div>
      </div>

      {/* NPU設定 */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6 mb-6">
        <div className="flex items-center gap-2 mb-6">
          <Cpu className="w-5 h-5 text-blue-600" />
          <h2 className="text-gray-900">NPU高速化</h2>
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
          </div>
          <button
            onClick={() => toggleSetting("npuAcceleration")}
            className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ml-4 ${
              settings.npuAcceleration
                ? "bg-blue-600"
                : "bg-gray-300"
            }`}
          >
            <span
              className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                settings.npuAcceleration
                  ? "translate-x-6"
                  : "translate-x-1"
              }`}
            />
          </button>
        </div>

        {settings.npuAcceleration && (
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
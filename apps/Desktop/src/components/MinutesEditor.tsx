import { useState } from 'react';
import { Sparkles, X, Download, Copy } from 'lucide-react';
import { ToastType } from './Toast';
import { ConfirmModal } from './ConfirmModal';

export function MinutesEditor({ onToast }: { onToast: (type: ToastType, message: string) => void }) {
  const [showAiSuggestions, setShowAiSuggestions] = useState(true);
  const [showDeleteModal, setShowDeleteModal] = useState(false);
  const [content, setContent] = useState(`会議日時：2024年12月20日 14:00-15:30
参加者：田中会長、佐藤副会長、鈴木、山田

議題：来月の夏祭りの準備について

---

1. 予算の確定
   - 予算は50万円で確定

2. 会場の予約
   - 来週までに確認が必要

3. 担当者の決定
   - チラシ作成：山田が担当

4. 次回会議
   - 来週の同じ時間に実施
`);

  const aiSuggestions = [
    {
      title: '要約案',
      content: '来月の夏祭りについて協議。予算50万円、会場予約の確認期限は来週。チラシ作成は山田が担当。次回会議は来週同時刻。',
    },
    {
      title: '決定事項',
      content: '• 予算50万円で確定\n• チラシ作成担当：山田\n• 次回会議：来週同時刻',
    },
    {
      title: '要確認事項',
      content: '• 会場予約の確認（期限：来週）\n• チラシデザインの進捗',
    },
  ];

  const removeAllAi = () => {
    setShowDeleteModal(true);
  };

  const handleConfirmDelete = () => {
    setShowDeleteModal(false);
    onToast('success', 'AI生成部分を削除しました');
  };

  const handleCopy = () => {
    navigator.clipboard.writeText(content);
    onToast('success', 'クリップボードにコピーしました');
  };

  const handleExport = () => {
    onToast('info', 'エクスポート機能は開発中です');
  };

  return (
    <div className="max-w-7xl mx-auto p-6">
      <div className="grid grid-cols-1 lg:grid-cols-3 gap-6">
        {/* メインエディタ */}
        <div className="lg:col-span-2">
          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <div className="flex items-center justify-between mb-4">
              <h2 className="text-gray-900">議事録エディタ</h2>
              <div className="flex gap-2">
                <button
                  onClick={removeAllAi}
                  className="px-3 py-2 text-sm text-red-600 hover:bg-red-50 rounded-lg transition-colors flex items-center gap-1"
                >
                  <X className="w-4 h-4" />
                  AI生成部分を全削除
                </button>
                <button
                  onClick={handleCopy}
                  className="px-3 py-2 text-sm text-gray-600 hover:bg-gray-100 rounded-lg transition-colors flex items-center gap-1"
                >
                  <Copy className="w-4 h-4" />
                  コピー
                </button>
                <button
                  onClick={handleExport}
                  className="px-3 py-2 text-sm bg-blue-600 text-white hover:bg-blue-700 rounded-lg transition-colors flex items-center gap-1"
                >
                  <Download className="w-4 h-4" />
                  エクスポート
                </button>
              </div>
            </div>

            {/* エディタ本文 */}
            <div className="mb-4">
              <textarea
                value={content}
                onChange={(e) => setContent(e.target.value)}
                className="w-full h-96 px-4 py-3 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 resize-none"
                placeholder="ここに議事録を編集してください..."
              />
            </div>

            {/* 説明文 */}
            <div className="flex items-start gap-2 p-3 bg-blue-50 border border-blue-200 rounded-lg">
              <Sparkles className="w-5 h-5 text-blue-600 flex-shrink-0 mt-0.5" />
              <div className="text-sm text-blue-800">
                <p>AIは「提案」するだけです。最終的な編集権限はあなたにあります。</p>
                <p className="mt-1">右側のAI要約案を参考に、自由に編集してください。</p>
              </div>
            </div>
          </div>
        </div>

        {/* AI要約案サイドバー */}
        <div className="lg:col-span-1">
          <div className="bg-gradient-to-br from-purple-50 to-blue-50 rounded-lg border-2 border-purple-200 p-6 sticky top-6">
            <div className="flex items-center justify-between mb-4">
              <div className="flex items-center gap-2">
                <Sparkles className="w-5 h-5 text-purple-600" />
                <h3 className="text-gray-900">AI要約案</h3>
              </div>
              <button
                onClick={() => setShowAiSuggestions(!showAiSuggestions)}
                className="text-sm text-gray-600 hover:text-gray-900"
              >
                {showAiSuggestions ? '非表示' : '表示'}
              </button>
            </div>

            {showAiSuggestions && (
              <div className="space-y-4">
                {aiSuggestions.map((suggestion, index) => (
                  <div key={index} className="bg-white rounded-lg p-4 border border-purple-200">
                    <div className="flex items-center justify-between mb-2">
                      <h4 className="text-sm text-purple-700">{suggestion.title}</h4>
                      <button
                        onClick={() => {
                          setContent(content + '\n\n' + suggestion.content);
                        }}
                        className="text-xs text-purple-600 hover:text-purple-800 underline"
                      >
                        挿入
                      </button>
                    </div>
                    <p className="text-sm text-gray-700 whitespace-pre-line">{suggestion.content}</p>
                  </div>
                ))}

                <div className="mt-4 p-3 bg-purple-100 rounded-lg border border-purple-200">
                  <p className="text-xs text-purple-900">
                    💡 これらはAIによる提案です。そのまま使用することも、編集することも、無視することもできます。
                  </p>
                </div>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* 色分け凡例 */}
      <div className="mt-6 bg-white rounded-lg shadow-sm border border-gray-200 p-4">
        <h3 className="text-sm text-gray-900 mb-3">編集モード説明</h3>
        <div className="grid grid-cols-1 md:grid-cols-3 gap-4 text-sm">
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 bg-purple-200 border border-purple-300 rounded"></div>
            <span className="text-gray-600">AI生成部分（編集・削除可能）</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 bg-white border border-gray-300 rounded"></div>
            <span className="text-gray-600">人が編集した部分</span>
          </div>
          <div className="flex items-center gap-2">
            <div className="w-4 h-4 bg-green-200 border border-green-300 rounded"></div>
            <span className="text-gray-600">確定済み</span>
          </div>
        </div>
      </div>

      {/* 削除確認モーダル */}
      <ConfirmModal
        isOpen={showDeleteModal}
        onClose={() => setShowDeleteModal(false)}
        onConfirm={handleConfirmDelete}
        title="AI生成部分を削除しますか？"
        message="人が編集した部分のみ残ります。"
      />
    </div>
  );
}
import { useEffect, useState } from 'react';
import { Command, X } from 'lucide-react';
import { motion, AnimatePresence } from 'motion/react';

interface KeyboardShortcutsProps {
  onNewMeeting: () => void;
  onGoToHistory: () => void;
  onGoToSettings: () => void;
  onToggleRecording: () => void;
}

export function KeyboardShortcuts({
  onNewMeeting,
  onGoToHistory,
  onGoToSettings,
  onToggleRecording,
}: KeyboardShortcutsProps) {
  const [showHelp, setShowHelp] = useState(false);

  useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      // Cmd/Ctrl + K でヘルプを表示
      if ((e.metaKey || e.ctrlKey) && e.key === 'k') {
        e.preventDefault();
        setShowHelp(true);
      }

      // ESCでヘルプを閉じる
      if (e.key === 'Escape') {
        setShowHelp(false);
      }

      // ショートカット（ヘルプが開いてない時のみ）
      if (!showHelp) {
        // Cmd/Ctrl + N で新規会議
        if ((e.metaKey || e.ctrlKey) && e.key === 'n') {
          e.preventDefault();
          onNewMeeting();
        }

        // Cmd/Ctrl + H で履歴
        if ((e.metaKey || e.ctrlKey) && e.key === 'h') {
          e.preventDefault();
          onGoToHistory();
        }

        // Cmd/Ctrl + , で設定
        if ((e.metaKey || e.ctrlKey) && e.key === ',') {
          e.preventDefault();
          onGoToSettings();
        }

        // Cmd/Ctrl + R で録音トグル
        if ((e.metaKey || e.ctrlKey) && e.key === 'r') {
          e.preventDefault();
          onToggleRecording();
        }
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [showHelp, onNewMeeting, onGoToHistory, onGoToSettings, onToggleRecording]);

  const shortcuts = [
    { keys: ['⌘', 'K'], description: 'キーボードショートカットを表示' },
    { keys: ['⌘', 'N'], description: '新規会議を開始' },
    { keys: ['⌘', 'H'], description: '過去の記録を表示' },
    { keys: ['⌘', ','], description: '設定を開く' },
    { keys: ['⌘', 'R'], description: '録音を開始/停止' },
    { keys: ['ESC'], description: 'モーダルを閉じる' },
  ];

  return (
    <>
      <AnimatePresence>
        {showHelp && (
          <div className="fixed inset-0 z-50 flex items-center justify-center p-4">
            {/* オーバーレイ */}
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              onClick={() => setShowHelp(false)}
              className="absolute inset-0 bg-black/50 backdrop-blur-sm"
            />

            {/* モーダル */}
            <motion.div
              initial={{ opacity: 0, scale: 0.95, y: 20 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95, y: 20 }}
              className="relative bg-white rounded-xl shadow-2xl max-w-lg w-full p-6"
            >
              <div className="flex items-center justify-between mb-6">
                <div className="flex items-center gap-2">
                  <Command className="w-5 h-5 text-blue-600" />
                  <h3 className="text-gray-900">キーボードショートカット</h3>
                </div>
                <button
                  onClick={() => setShowHelp(false)}
                  className="text-gray-400 hover:text-gray-600 transition-colors"
                >
                  <X className="w-5 h-5" />
                </button>
              </div>

              <div className="space-y-3">
                {shortcuts.map((shortcut, index) => (
                  <div
                    key={index}
                    className="flex items-center justify-between p-3 bg-gray-50 rounded-lg"
                  >
                    <span className="text-sm text-gray-700">{shortcut.description}</span>
                    <div className="flex gap-1">
                      {shortcut.keys.map((key, keyIndex) => (
                        <kbd
                          key={keyIndex}
                          className="px-2 py-1 text-xs bg-white border border-gray-300 rounded shadow-sm"
                        >
                          {key}
                        </kbd>
                      ))}
                    </div>
                  </div>
                ))}
              </div>

              <div className="mt-6 p-3 bg-blue-50 border border-blue-200 rounded-lg">
                <p className="text-xs text-blue-800">
                  💡 WindowsユーザーはCtrlキーを使用してください（例：Ctrl + N）
                </p>
              </div>
            </motion.div>
          </div>
        )}
      </AnimatePresence>
    </>
  );
}

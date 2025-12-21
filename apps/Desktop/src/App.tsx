import { useState } from 'react';
import { MeetingDashboard } from './components/MeetingDashboard';
import { MinutesEditor } from './components/MinutesEditor';
import { SettingsPanel } from './components/SettingsPanel';
import { OnboardingFlow } from './components/OnboardingFlow';
import { PrivacyStatusBar } from './components/PrivacyStatusBar';
import { MeetingHistory } from './components/MeetingHistory';
import { ToastContainer, ToastType } from './components/Toast';
import { KeyboardShortcuts } from './components/KeyboardShortcuts';
import { Tooltip } from './components/Tooltip';
import { motion } from 'motion/react';
import * as TauriAPI from './api/tauri';

type ViewType = 'meeting' | 'editor' | 'settings' | 'history';

interface Toast {
  id: string;
  type: ToastType;
  message: string;
}

export default function App() {
  const [currentView, setCurrentView] = useState<ViewType>('meeting');
  const [showOnboarding, setShowOnboarding] = useState(true);
  const [isRecording, setIsRecording] = useState(false);
  const [toasts, setToasts] = useState<Toast[]>([]);

  const handleOnboardingComplete = () => {
    setShowOnboarding(false);
    addToast('success', 'ようこそGijiroku21へ');
  };

  const addToast = (type: ToastType, message: string) => {
    const id = Date.now().toString();
    setToasts((prev) => [...prev, { id, type, message }]);
  };

  const removeToast = (id: string) => {
    setToasts((prev) => prev.filter((toast) => toast.id !== id));
  };

  const handleViewChange = (view: ViewType) => {
    setCurrentView(view);
    const messages: Record<ViewType, string> = {
      meeting: '会議画面を表示しました',
      editor: '議事録編集画面を表示しました',
      history: '過去の記録を表示しました',
      settings: '設定画面を表示しました',
    };
    addToast('info', messages[view]);
  };

  const handleToggleRecording = async () => {
    try {
      const status = await TauriAPI.getRecordingStatus();
      if (status.status === 'recording') {
        await TauriAPI.stopRecording();
        setIsRecording(false);
        addToast('warning', '録音を停止しました');
      } else {
        await TauriAPI.startRecording('クイック会議');
        setIsRecording(true);
        addToast('success', '録音を開始しました');
      }
    } catch (e) {
      console.error('[App] toggleRecording failed', e);
      addToast('error', '録音操作に失敗しました');
    }
  };

  if (showOnboarding) {
    return <OnboardingFlow onComplete={handleOnboardingComplete} />;
  }

  return (
    <div className="min-h-screen bg-gray-50 flex flex-col">
      {/* キーボードショートカット */}
      <KeyboardShortcuts
        onNewMeeting={() => handleViewChange('meeting')}
        onGoToHistory={() => handleViewChange('history')}
        onGoToSettings={() => handleViewChange('settings')}
        onToggleRecording={handleToggleRecording}
      />

      {/* ヘッダー */}
      <header className="bg-white border-b border-gray-200 px-6 py-4">
        <div className="max-w-7xl mx-auto flex items-center justify-between">
          <motion.div
            initial={{ opacity: 0, x: -20 }}
            animate={{ opacity: 1, x: 0 }}
            className="flex items-center gap-3"
          >
            <div className="w-10 h-10 bg-blue-600 rounded-lg flex items-center justify-center">
              <svg className="w-6 h-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z" />
              </svg>
            </div>
            <div>
              <h1 className="text-gray-900">Gijiroku21</h1>
              <p className="text-sm text-gray-500">ローカルAI議事録アプリ</p>
            </div>
          </motion.div>

          {/* ナビゲーション */}
          <nav className="flex gap-2">
            <Tooltip content="新規会議 (⌘N)">
              <button
                onClick={() => handleViewChange('meeting')}
                className={`px-4 py-2 rounded-lg transition-all ${
                  currentView === 'meeting'
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-600 hover:bg-gray-100'
                }`}
              >
                会議中
              </button>
            </Tooltip>
            <Tooltip content="議事録を編集">
              <button
                onClick={() => handleViewChange('editor')}
                className={`px-4 py-2 rounded-lg transition-all ${
                  currentView === 'editor'
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-600 hover:bg-gray-100'
                }`}
              >
                議事録編集
              </button>
            </Tooltip>
            <Tooltip content="過去の記録 (⌘H)">
              <button
                onClick={() => handleViewChange('history')}
                className={`px-4 py-2 rounded-lg transition-all ${
                  currentView === 'history'
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-600 hover:bg-gray-100'
                }`}
              >
                過去の記録
              </button>
            </Tooltip>
            <Tooltip content="設定 (⌘,)">
              <button
                onClick={() => handleViewChange('settings')}
                className={`px-4 py-2 rounded-lg transition-all ${
                  currentView === 'settings'
                    ? 'bg-blue-100 text-blue-700'
                    : 'text-gray-600 hover:bg-gray-100'
                }`}
              >
                設定
              </button>
            </Tooltip>
          </nav>
        </div>
      </header>

      {/* メインコンテンツ */}
      <motion.main
        key={currentView}
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        exit={{ opacity: 0, y: -20 }}
        transition={{ duration: 0.2 }}
        className="flex-1 overflow-auto"
      >
        {currentView === 'meeting' && (
          <MeetingDashboard 
            isRecording={isRecording}
            setIsRecording={setIsRecording}
            onToast={addToast}
          />
        )}
        {currentView === 'editor' && <MinutesEditor onToast={addToast} />}
        {currentView === 'history' && <MeetingHistory onToast={addToast} />}
        {currentView === 'settings' && <SettingsPanel onToast={addToast} />}
      </motion.main>

      {/* プライバシーステータスバー */}
      <PrivacyStatusBar isRecording={isRecording} />

      {/* トースト通知 */}
      <ToastContainer toasts={toasts} onClose={removeToast} />
    </div>
  );
}

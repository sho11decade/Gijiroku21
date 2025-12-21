import { useState, useEffect } from 'react';
import { Mic, MicOff, Star, CheckCircle, AlertCircle } from 'lucide-react';
import { motion, AnimatePresence } from 'framer-motion';
import { EmptyState } from './EmptyState';
import { ToastType } from './Toast';
import * as TauriAPI from '../api/tauri';
import { listen, UnlistenFn } from '@tauri-apps/api/event';

interface MeetingDashboardProps {
  isRecording: boolean;
  setIsRecording: (value: boolean) => void;
  onToast: (type: ToastType, message: string) => void;
}

interface TranscriptSegment {
  start: number;
  end: number;
  text: string;
  confidence: number;
  speaker: string | null;
}

interface Transcript {
  id: number;
  speaker: string;
  text: string;
  confidence: number;
  timestamp: string;
  tags?: ('important' | 'decision' | 'confirm')[];
}

export function MeetingDashboard({ isRecording, setIsRecording, onToast }: MeetingDashboardProps) {
  const [transcripts, setTranscripts] = useState<Transcript[]>([]);
  const [meetingTime, setMeetingTime] = useState(0);
  const [meetingTitle, setMeetingTitle] = useState('〇〇自治会 定例会');
  const [currentMeetingId, setCurrentMeetingId] = useState<string | null>(null);

  useEffect(() => {
    if (isRecording) {
      const interval = setInterval(() => {
        setMeetingTime((prev) => prev + 1);
      }, 1000);
      return () => clearInterval(interval);
    }
  }, [isRecording]);

  // Tauri Eventリスナー：文字起こし結果を受信
  useEffect(() => {
    let unlisten: UnlistenFn | null = null;

    const setupListener = async () => {
      unlisten = await listen<TranscriptSegment>('transcript_update', (event) => {
        const segment = event.payload;
        const newTranscript: Transcript = {
          id: Date.now(),
          speaker: segment.speaker || '不明',
          text: segment.text,
          confidence: segment.confidence,
          timestamp: new Date().toLocaleTimeString('ja-JP', { 
            hour: '2-digit', 
            minute: '2-digit', 
            second: '2-digit' 
          }),
        };
        setTranscripts((prev) => [...prev, newTranscript]);
      });
    };

    setupListener();

    return () => {
      if (unlisten) {
        unlisten();
      }
    };
  }, []);

  // モック：録音中は定期的に新しい発言を追加（将来削除予定）
  useEffect(() => {
    if (isRecording) {
      const mockPhrases = [
        { speaker: '山田', text: 'その件については私が担当します。', tags: [] as ('important' | 'decision' | 'confirm')[] },
        { speaker: '田中会長', text: '次回の会議は来週の同じ時間でお願いします。', tags: ['decision'] as ('important' | 'decision' | 'confirm')[] },
        { speaker: '佐藤副会長', text: 'チラシのデザインはどうなっていますか？', tags: ['confirm'] as ('important' | 'decision' | 'confirm')[] },
      ];

      const interval = setInterval(() => {
        const randomPhrase = mockPhrases[Math.floor(Math.random() * mockPhrases.length)];
        const newTranscript: Transcript = {
          id: Date.now(),
          speaker: randomPhrase.speaker,
          text: randomPhrase.text,
          confidence: 0.85 + Math.random() * 0.15,
          timestamp: new Date().toLocaleTimeString('ja-JP', { hour: '2-digit', minute: '2-digit', second: '2-digit' }),
          tags: randomPhrase.tags.length > 0 ? randomPhrase.tags : undefined,
        };
        setTranscripts((prev) => [...prev, newTranscript]);
      }, 8000);

      return () => clearInterval(interval);
    }
  }, [isRecording]);

  const formatTime = (seconds: number) => {
    const mins = Math.floor(seconds / 60);
    const secs = seconds % 60;
    return `${mins.toString().padStart(2, '0')}:${secs.toString().padStart(2, '0')}`;
  };

  const toggleRecording = async () => {
    try {
      if (!isRecording) {
        // 録音開始
        const meetingId = await TauriAPI.startRecording(meetingTitle);
        setCurrentMeetingId(meetingId);
        setIsRecording(true);
        setMeetingTime(0);
        setTranscripts([]);
        onToast('success', '録音を開始しました');
      } else {
        // 録音停止
        await TauriAPI.stopRecording();
        setIsRecording(false);
        setCurrentMeetingId(null);
        onToast('success', '録音を停止しました');
      }
    } catch (error) {
      onToast('error', `録音の操作に失敗しました: ${error}`);
    }
  };

  const addTag = (id: number, tag: 'important' | 'decision' | 'confirm') => {
    setTranscripts((prev) =>
      prev.map((t) =>
        t.id === id
          ? { ...t, tags: t.tags ? [...t.tags, tag] : [tag] }
          : t
      )
    );
  };

  return (
    <div className="max-w-7xl mx-auto p-6">
      {/* 会議情報カード */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6 mb-6">
        <div className="grid grid-cols-1 md:grid-cols-3 gap-6">
          <div>
            <label className="text-sm text-gray-500 mb-1 block">会議名</label>
            <input
              type="text"
              value={meetingTitle}
              onChange={(e) => setMeetingTitle(e.target.value)}
              disabled={isRecording}
              className="w-full px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:bg-gray-100 disabled:cursor-not-allowed"
            />
          </div>
          <div>
            <label className="text-sm text-gray-500 mb-1 block">状態</label>
            <div className="flex items-center gap-2 px-3 py-2 bg-gray-50 rounded-lg">
              {isRecording ? (
                <>
                  <span className="relative flex h-3 w-3">
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-red-400 opacity-75"></span>
                    <span className="relative inline-flex rounded-full h-3 w-3 bg-red-600"></span>
                  </span>
                  <span className="text-gray-900">録音中（ローカル処理）</span>
                </>
              ) : (
                <>
                  <span className="inline-flex rounded-full h-3 w-3 bg-gray-400"></span>
                  <span className="text-gray-600">停止中</span>
                </>
              )}
            </div>
          </div>
          <div>
            <label className="text-sm text-gray-500 mb-1 block">AI処理</label>
            <div className="flex items-center gap-2 px-3 py-2 bg-gray-50 rounded-lg">
              <span className={`inline-flex rounded-full h-3 w-3 ${isRecording ? 'bg-green-600' : 'bg-gray-400'}`}></span>
              <span className="text-gray-900">NPU {isRecording ? '使用中' : '待機中'}</span>
            </div>
          </div>
        </div>

        {/* 録音コントロール */}
        <div className="mt-6 flex items-center justify-between">
          <div className="text-2xl text-gray-900">{formatTime(meetingTime)}</div>
          <button
            onClick={toggleRecording}
            className={`px-8 py-3 rounded-lg transition-all flex items-center gap-2 ${
              isRecording
                ? 'bg-red-600 hover:bg-red-700 text-white'
                : 'bg-blue-600 hover:bg-blue-700 text-white'
            }`}
          >
            {isRecording ? (
              <>
                <MicOff className="w-5 h-5" />
                録音停止
              </>
            ) : (
              <>
                <Mic className="w-5 h-5" />
                録音開始
              </>
            )}
          </button>
        </div>
      </div>

      {/* 文字起こしエリア */}
      <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6 mb-6">
        <div className="flex items-center justify-between mb-4">
          <h2 className="text-gray-900">リアルタイム文字起こし</h2>
          <div className="flex gap-2">
            <button className="px-3 py-1 text-sm bg-yellow-100 text-yellow-800 rounded-full flex items-center gap-1">
              <Star className="w-4 h-4" />
              重要
            </button>
            <button className="px-3 py-1 text-sm bg-blue-100 text-blue-800 rounded-full flex items-center gap-1">
              <CheckCircle className="w-4 h-4" />
              決定事項
            </button>
            <button className="px-3 py-1 text-sm bg-orange-100 text-orange-800 rounded-full flex items-center gap-1">
              <AlertCircle className="w-4 h-4" />
              要確認
            </button>
          </div>
        </div>

        <div className="space-y-4 max-h-96 overflow-y-auto">
          {transcripts.map((transcript) => (
            <div
              key={transcript.id}
              className={`p-4 rounded-lg border transition-colors ${
                transcript.confidence < 0.9
                  ? 'bg-gray-50 border-gray-200 opacity-70'
                  : 'bg-white border-gray-300'
              }`}
            >
              <div className="flex items-start justify-between mb-2">
                <div className="flex items-center gap-2">
                  <span className="px-2 py-1 bg-blue-100 text-blue-800 rounded text-sm">
                    {transcript.speaker}
                  </span>
                  <span className="text-xs text-gray-500">{transcript.timestamp}</span>
                  {transcript.confidence < 0.9 && (
                    <span className="text-xs text-gray-400">（確信度: {Math.round(transcript.confidence * 100)}%）</span>
                  )}
                </div>
                {transcript.tags && (
                  <div className="flex gap-1">
                    {transcript.tags.includes('important') && (
                      <Star className="w-4 h-4 text-yellow-600 fill-yellow-600" />
                    )}
                    {transcript.tags.includes('decision') && (
                      <CheckCircle className="w-4 h-4 text-blue-600 fill-blue-600" />
                    )}
                    {transcript.tags.includes('confirm') && (
                      <AlertCircle className="w-4 h-4 text-orange-600 fill-orange-600" />
                    )}
                  </div>
                )}
              </div>
              <p className="text-gray-800">{transcript.text}</p>
            </div>
          ))}
        </div>

        {transcripts.length === 0 && (
          <div className="text-center text-gray-400 py-12">
            録音を開始すると、ここに文字起こしが表示されます
          </div>
        )}
      </div>

      {/* プライバシー通知 */}
      <div className="bg-green-50 border-2 border-green-600 rounded-lg p-4 flex items-center gap-3">
        <div className="w-12 h-12 bg-green-600 rounded-full flex items-center justify-center flex-shrink-0">
          <svg className="w-6 h-6 text-white" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" />
          </svg>
        </div>
        <div>
          <p className="text-green-900">この音声・文字は外部送信されません</p>
          <p className="text-sm text-green-700">すべての処理はこのPC内で完結しています</p>
        </div>
      </div>
    </div>
  );
}
import { useState } from 'react';
import { Search, Calendar, Trash2, Eye, Download, Lock, Plus, X, Tag } from 'lucide-react';
import { TagEditor } from './TagEditor';
import { ConfirmModal } from './ConfirmModal';
import { ToastType } from './Toast';
import { motion } from 'motion/react';
import { EmptyState } from './EmptyState';

interface MeetingRecord {
  id: string;
  title: string;
  date: string;
  duration: string;
  participants: string[];
  tags: string[];
  summary: string;
  transcript: string;
  decisions: string[];
  confirmItems: string[];
}

export function MeetingHistory({ onToast }: { onToast: (type: ToastType, message: string) => void }) {
  const [searchQuery, setSearchQuery] = useState('');
  const [selectedFilter, setSelectedFilter] = useState<'all' | 'thisMonth' | 'lastMonth' | 'thisYear'>('all');
  const [selectedRecord, setSelectedRecord] = useState<MeetingRecord | null>(null);
  const [selectedTags, setSelectedTags] = useState<string[]>([]);
  const [showTagEditor, setShowTagEditor] = useState(false);
  const [deleteConfirm, setDeleteConfirm] = useState<{ id: string; title: string } | null>(null);

  // モックデータ
  const [meetingRecords, setMeetingRecords] = useState<MeetingRecord[]>([
    {
      id: '1',
      title: '〇〇自治会 定例会',
      date: '2024-12-20 14:00',
      duration: '1時間30分',
      participants: ['田中会長', '佐藤副会長', '鈴木', '山田'],
      tags: ['定例会', '夏祭り'],
      summary: '来月の夏祭りについて協議。予算50万円、会場予約の確認期限は来週。チラシ作成は山田が担当。',
      transcript: '会議の文字起こし全文...',
      decisions: ['予算50万円で確定', 'チラシ作成担当：山田'],
      confirmItems: ['会場予約の確認（期限：来週）'],
    },
    {
      id: '2',
      title: '〇〇自治会 臨時会議',
      date: '2024-12-15 10:00',
      duration: '45分',
      participants: ['田中会長', '佐藤副会長', '高橋'],
      tags: ['臨時会議', '防災'],
      summary: '防災訓練の日程調整。12月28日に実施予定。参加者への連絡方法を検討。',
      transcript: '会議の文字起こし全文...',
      decisions: ['防災訓練：12月28日実施'],
      confirmItems: ['参加者への連絡担当を決定'],
    },
    {
      id: '3',
      title: '〇〇自治会 11月定例会',
      date: '2024-11-20 14:00',
      duration: '2時間',
      participants: ['田中会長', '佐藤副会長', '鈴木', '山田', '高橋'],
      tags: ['定例会', '予算'],
      summary: '来年度予算案の検討。各委員会からの要望を集約。次回までに予算案を作成。',
      transcript: '会議の文字起こし全文...',
      decisions: ['予算案作成担当：佐藤副会長'],
      confirmItems: ['各委員会からの要望提出期限：11月末'],
    },
    {
      id: '4',
      title: '〇〇自治会 10月定例会',
      date: '2024-10-20 14:00',
      duration: '1時間15分',
      participants: ['田中会長', '佐藤副会長', '鈴木'],
      tags: ['定例会', '清掃活動'],
      summary: '秋の清掃活動の報告。参加者50名。次回は春に実施予定。',
      transcript: '会議の文字起こし全文...',
      decisions: ['次回清掃活動：来春実施'],
      confirmItems: [],
    },
  ]);

  // すべてのタグを収集
  const allTags = Array.from(new Set(meetingRecords.flatMap(record => record.tags)));

  const updateRecordTags = (recordId: string, newTags: string[]) => {
    setMeetingRecords(prev =>
      prev.map(record =>
        record.id === recordId ? { ...record, tags: newTags } : record
      )
    );
    if (selectedRecord && selectedRecord.id === recordId) {
      setSelectedRecord({ ...selectedRecord, tags: newTags });
    }
  };

  const filteredRecords = meetingRecords.filter((record) => {
    // 検索クエリフィルター
    const matchesSearch = 
      record.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
      record.summary.toLowerCase().includes(searchQuery.toLowerCase()) ||
      record.participants.some(p => p.toLowerCase().includes(searchQuery.toLowerCase()));

    if (!matchesSearch) return false;

    // タグフィルター
    if (selectedTags.length > 0) {
      const hasSelectedTag = selectedTags.some(tag => record.tags.includes(tag));
      if (!hasSelectedTag) return false;
    }

    // 日付フィルター
    const recordDate = new Date(record.date);
    const now = new Date();
    
    switch (selectedFilter) {
      case 'thisMonth':
        return recordDate.getMonth() === now.getMonth() && recordDate.getFullYear() === now.getFullYear();
      case 'lastMonth':
        const lastMonth = new Date(now.getFullYear(), now.getMonth() - 1);
        return recordDate.getMonth() === lastMonth.getMonth() && recordDate.getFullYear() === lastMonth.getFullYear();
      case 'thisYear':
        return recordDate.getFullYear() === now.getFullYear();
      default:
        return true;
    }
  });

  const deleteRecord = (id: string) => {
    setMeetingRecords(prev => prev.filter(record => record.id !== id));
    setSelectedRecord(null);
    onToast('success', '会議記録を削除しました');
  };

  const exportRecord = (record: MeetingRecord) => {
    alert(`「${record.title}」をエクスポートします（デモ）`);
  };

  const toggleTagFilter = (tag: string) => {
    setSelectedTags(prev =>
      prev.includes(tag) ? prev.filter(t => t !== tag) : [...prev, tag]
    );
  };

  return (
    <div className="max-w-7xl mx-auto p-6">
      {selectedRecord ? (
        /* 詳細表示 */
        <div>
          <button
            onClick={() => setSelectedRecord(null)}
            className="mb-4 text-blue-600 hover:text-blue-700 flex items-center gap-1"
          >
            ← 一覧に戻る
          </button>

          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-6">
            <div className="flex items-start justify-between mb-6">
              <div>
                <h2 className="text-gray-900 mb-2">{selectedRecord.title}</h2>
                <div className="flex flex-wrap gap-3 text-sm text-gray-600">
                  <span className="flex items-center gap-1">
                    <Calendar className="w-4 h-4" />
                    {selectedRecord.date}
                  </span>
                  <span>時間：{selectedRecord.duration}</span>
                </div>
              </div>
              <div className="flex gap-2">
                <button
                  onClick={() => exportRecord(selectedRecord)}
                  className="px-3 py-2 text-sm bg-blue-600 text-white hover:bg-blue-700 rounded-lg flex items-center gap-1"
                >
                  <Download className="w-4 h-4" />
                  エクスポート
                </button>
                <button
                  onClick={() => setDeleteConfirm({ id: selectedRecord.id, title: selectedRecord.title })}
                  className="px-3 py-2 text-sm text-red-600 hover:bg-red-50 rounded-lg flex items-center gap-1"
                >
                  <Trash2 className="w-4 h-4" />
                  削除
                </button>
              </div>
            </div>

            {/* タグ編集エリア */}
            <div className="mb-6">
              <div className="flex items-center justify-between mb-2">
                <h3 className="text-sm text-gray-700">タグ</h3>
                <button
                  onClick={() => setShowTagEditor(!showTagEditor)}
                  className="text-sm text-blue-600 hover:text-blue-700 flex items-center gap-1"
                >
                  <Tag className="w-4 h-4" />
                  {showTagEditor ? 'タグ編集を閉じる' : 'タグを編集'}
                </button>
              </div>

              {showTagEditor ? (
                <TagEditor
                  recordId={selectedRecord.id}
                  currentTags={selectedRecord.tags}
                  allTags={allTags}
                  onUpdateTags={updateRecordTags}
                  onClose={() => setShowTagEditor(false)}
                />
              ) : (
                <div className="flex flex-wrap gap-2">
                  {selectedRecord.tags.length > 0 ? (
                    selectedRecord.tags.map((tag, index) => (
                      <span
                        key={index}
                        className="px-3 py-1 bg-blue-100 text-blue-800 rounded-full text-sm"
                      >
                        {tag}
                      </span>
                    ))
                  ) : (
                    <span className="text-sm text-gray-400">タグが設定されていません</span>
                  )}
                </div>
              )}
            </div>

            {/* 参加者 */}
            <div className="mb-6">
              <h3 className="text-sm text-gray-700 mb-2">参加者</h3>
              <div className="flex flex-wrap gap-2">
                {selectedRecord.participants.map((participant, index) => (
                  <span
                    key={index}
                    className="px-3 py-1 bg-gray-100 text-gray-800 rounded-lg text-sm"
                  >
                    {participant}
                  </span>
                ))}
              </div>
            </div>

            {/* 要約 */}
            <div className="mb-6">
              <h3 className="text-sm text-gray-700 mb-2">要約</h3>
              <p className="text-gray-800 bg-gray-50 p-4 rounded-lg">{selectedRecord.summary}</p>
            </div>

            {/* 決定事項 */}
            {selectedRecord.decisions.length > 0 && (
              <div className="mb-6">
                <h3 className="text-sm text-gray-700 mb-2">決定事項</h3>
                <ul className="space-y-2">
                  {selectedRecord.decisions.map((decision, index) => (
                    <li
                      key={index}
                      className="flex items-start gap-2 bg-blue-50 p-3 rounded-lg"
                    >
                      <span className="text-blue-600 mt-0.5">✓</span>
                      <span className="text-gray-800">{decision}</span>
                    </li>
                  ))}
                </ul>
              </div>
            )}

            {/* 要確認事項 */}
            {selectedRecord.confirmItems.length > 0 && (
              <div className="mb-6">
                <h3 className="text-sm text-gray-700 mb-2">要確認事項</h3>
                <ul className="space-y-2">
                  {selectedRecord.confirmItems.map((item, index) => (
                    <li
                      key={index}
                      className="flex items-start gap-2 bg-orange-50 p-3 rounded-lg"
                    >
                      <span className="text-orange-600 mt-0.5">!</span>
                      <span className="text-gray-800">{item}</span>
                    </li>
                  ))}
                </ul>
              </div>
            )}

            {/* 文字起こし全文 */}
            <div>
              <h3 className="text-sm text-gray-700 mb-2">文字起こし全文</h3>
              <div className="bg-gray-50 p-4 rounded-lg border border-gray-200">
                <p className="text-gray-600 text-sm">
                  {selectedRecord.transcript}
                </p>
              </div>
            </div>
          </div>

          {/* プライバシー通知 */}
          <div className="mt-6 bg-green-50 border border-green-600 rounded-lg p-4 flex items-center gap-3">
            <Lock className="w-5 h-5 text-green-600 flex-shrink-0" />
            <div className="text-sm text-green-800">
              この会議記録はあなたのPC内にのみ保存されています。削除するとデータは完全に消去されます。
            </div>
          </div>
        </div>
      ) : (
        /* 一覧表示 */
        <div>
          <div className="mb-6">
            <h2 className="text-gray-900 mb-2">過去の会議記録</h2>
            <p className="text-gray-600">すべての記録はこのPC内に保存されています</p>
          </div>

          {/* 検索とフィルター */}
          <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-4 mb-4">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              {/* 検索バー */}
              <div className="relative">
                <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 text-gray-400 w-5 h-5" />
                <input
                  type="text"
                  placeholder="会議名、参加者、内容で検索..."
                  value={searchQuery}
                  onChange={(e) => setSearchQuery(e.target.value)}
                  className="w-full pl-10 pr-4 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500"
                />
              </div>

              {/* 期間フィルター */}
              <div className="flex gap-2">
                <button
                  onClick={() => setSelectedFilter('all')}
                  className={`flex-1 px-3 py-2 rounded-lg transition-colors text-sm ${
                    selectedFilter === 'all'
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  すべて
                </button>
                <button
                  onClick={() => setSelectedFilter('thisMonth')}
                  className={`flex-1 px-3 py-2 rounded-lg transition-colors text-sm ${
                    selectedFilter === 'thisMonth'
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  今月
                </button>
                <button
                  onClick={() => setSelectedFilter('lastMonth')}
                  className={`flex-1 px-3 py-2 rounded-lg transition-colors text-sm ${
                    selectedFilter === 'lastMonth'
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  先月
                </button>
                <button
                  onClick={() => setSelectedFilter('thisYear')}
                  className={`flex-1 px-3 py-2 rounded-lg transition-colors text-sm ${
                    selectedFilter === 'thisYear'
                      ? 'bg-blue-600 text-white'
                      : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                  }`}
                >
                  今年
                </button>
              </div>
            </div>
          </div>

          {/* タグフィルター */}
          {allTags.length > 0 && (
            <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-4 mb-4">
              <div className="flex items-center gap-2 mb-3">
                <Tag className="w-4 h-4 text-gray-600" />
                <h3 className="text-sm text-gray-700">タグで絞り込み</h3>
                {selectedTags.length > 0 && (
                  <button
                    onClick={() => setSelectedTags([])}
                    className="text-xs text-blue-600 hover:text-blue-700 ml-auto"
                  >
                    クリア
                  </button>
                )}
              </div>
              <div className="flex flex-wrap gap-2">
                {allTags.map((tag, index) => (
                  <button
                    key={index}
                    onClick={() => toggleTagFilter(tag)}
                    className={`px-3 py-1 rounded-full text-sm transition-colors ${
                      selectedTags.includes(tag)
                        ? 'bg-blue-600 text-white'
                        : 'bg-gray-100 text-gray-700 hover:bg-gray-200'
                    }`}
                  >
                    {tag}
                  </button>
                ))}
              </div>
            </div>
          )}

          {/* 検索結果件数 */}
          <div className="mb-4 text-sm text-gray-600">
            {filteredRecords.length}件の会議記録が見つかりました
            {selectedTags.length > 0 && (
              <span className="ml-2">
                （タグ: {selectedTags.map(tag => `「${tag}」`).join(', ')} で絞り込み中）
              </span>
            )}
          </div>

          {/* 会議記録一覧 */}
          <div className="space-y-4">
            {filteredRecords.map((record) => (
              <div
                key={record.id}
                className="bg-white rounded-lg shadow-sm border border-gray-200 p-6 hover:shadow-md transition-shadow"
              >
                <div className="flex items-start justify-between mb-3">
                  <div className="flex-1">
                    <h3 className="text-gray-900 mb-2">{record.title}</h3>
                    <div className="flex flex-wrap gap-3 text-sm text-gray-600 mb-3">
                      <span className="flex items-center gap-1">
                        <Calendar className="w-4 h-4" />
                        {record.date}
                      </span>
                      <span>時間：{record.duration}</span>
                      <span>参加者：{record.participants.length}名</span>
                    </div>
                    <div className="flex flex-wrap gap-2 mb-3">
                      {record.tags.map((tag, index) => (
                        <span
                          key={index}
                          className="px-2 py-1 bg-blue-100 text-blue-800 rounded-full text-xs"
                        >
                          {tag}
                        </span>
                      ))}
                    </div>
                    <p className="text-gray-600 text-sm line-clamp-2">{record.summary}</p>
                  </div>
                  <div className="flex gap-2 ml-4">
                    <button
                      onClick={() => setSelectedRecord(record)}
                      className="px-3 py-2 text-sm bg-blue-600 text-white hover:bg-blue-700 rounded-lg flex items-center gap-1 whitespace-nowrap"
                    >
                      <Eye className="w-4 h-4" />
                      詳細
                    </button>
                  </div>
                </div>

                {/* 決定事項プレビュー */}
                {record.decisions.length > 0 && (
                  <div className="mt-3 pt-3 border-t border-gray-200">
                    <div className="text-xs text-gray-600 mb-1">決定事項：{record.decisions.length}件</div>
                    <div className="text-sm text-gray-700">
                      {record.decisions[0]}
                      {record.decisions.length > 1 && <span className="text-gray-500"> ほか</span>}
                    </div>
                  </div>
                )}
              </div>
            ))}

            {filteredRecords.length === 0 && (
              <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-12 text-center">
                <Search className="w-12 h-12 text-gray-300 mx-auto mb-3" />
                <p className="text-gray-500">検索条件に一致する会議記録が見つかりませんでした</p>
              </div>
            )}
          </div>

          {/* ストレージ情報 */}
          <div className="mt-6 bg-gray-50 border border-gray-200 rounded-lg p-4">
            <div className="flex items-center justify-between text-sm">
              <div className="flex items-center gap-2 text-gray-700">
                <Lock className="w-4 h-4" />
                <span>ローカルストレージ使用状況</span>
              </div>
              <div className="text-gray-900">
                {meetingRecords.length}件の会議記録 / 約 2.4MB
              </div>
            </div>
            <div className="mt-2 w-full bg-gray-200 rounded-full h-2">
              <div className="bg-blue-600 h-2 rounded-full" style={{ width: '15%' }}></div>
            </div>
            <p className="mt-2 text-xs text-gray-600">
              すべてのデータはこのPC内にのみ保存されています。クラウドには一切送信されません。
            </p>
          </div>
        </div>
      )}

      {/* 削除確認モーダル */}
      {deleteConfirm && (
        <ConfirmModal
          isOpen={true}
          title="会議記録の削除"
          message={`「${deleteConfirm.title}」を削除しますか？\nこの操作は取り消せません。`}
          confirmText="削除"
          variant="danger"
          onConfirm={() => {
            deleteRecord(deleteConfirm.id);
            setDeleteConfirm(null);
          }}
          onClose={() => setDeleteConfirm(null)}
        />
      )}
    </div>
  );
}
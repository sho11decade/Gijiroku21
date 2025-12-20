import { useState } from 'react';
import { X, Plus, Tag as TagIcon, Sparkles } from 'lucide-react';

interface TagEditorProps {
  recordId: string;
  currentTags: string[];
  allTags: string[];
  onUpdateTags: (recordId: string, newTags: string[]) => void;
  onClose: () => void;
}

export function TagEditor({ recordId, currentTags, allTags, onUpdateTags, onClose }: TagEditorProps) {
  const [tags, setTags] = useState<string[]>(currentTags);
  const [newTagInput, setNewTagInput] = useState('');

  // よく使うタグの候補（既存のタグから）
  const suggestedTags = allTags.filter(tag => !tags.includes(tag));

  // AI提案タグ（モック）
  const aiSuggestedTags = ['重要', '緊急', 'フォローアップ必要'];

  const addTag = (tag: string) => {
    if (tag.trim() && !tags.includes(tag.trim())) {
      setTags([...tags, tag.trim()]);
    }
  };

  const removeTag = (tagToRemove: string) => {
    setTags(tags.filter(tag => tag !== tagToRemove));
  };

  const handleAddNewTag = () => {
    if (newTagInput.trim() && !tags.includes(newTagInput.trim())) {
      setTags([...tags, newTagInput.trim()]);
      setNewTagInput('');
    }
  };

  const handleSave = () => {
    onUpdateTags(recordId, tags);
    onClose();
  };

  const handleKeyPress = (e: React.KeyboardEvent) => {
    if (e.key === 'Enter') {
      e.preventDefault();
      handleAddNewTag();
    }
  };

  return (
    <div className="bg-gradient-to-br from-blue-50 to-purple-50 border-2 border-blue-200 rounded-lg p-6">
      {/* 現在のタグ */}
      <div className="mb-6">
        <h4 className="text-sm text-gray-700 mb-3 flex items-center gap-2">
          <TagIcon className="w-4 h-4" />
          現在のタグ
        </h4>
        <div className="flex flex-wrap gap-2 min-h-[2.5rem]">
          {tags.length > 0 ? (
            tags.map((tag, index) => (
              <span
                key={index}
                className="px-3 py-1 bg-blue-600 text-white rounded-full text-sm flex items-center gap-2"
              >
                {tag}
                <button
                  onClick={() => removeTag(tag)}
                  className="hover:bg-blue-700 rounded-full p-0.5 transition-colors"
                >
                  <X className="w-3 h-3" />
                </button>
              </span>
            ))
          ) : (
            <span className="text-sm text-gray-400 py-1">タグを追加してください</span>
          )}
        </div>
      </div>

      {/* 新しいタグを追加 */}
      <div className="mb-6">
        <h4 className="text-sm text-gray-700 mb-3">新しいタグを追加</h4>
        <div className="flex gap-2">
          <input
            type="text"
            value={newTagInput}
            onChange={(e) => setNewTagInput(e.target.value)}
            onKeyPress={handleKeyPress}
            placeholder="タグ名を入力..."
            className="flex-1 px-3 py-2 border border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 text-sm"
          />
          <button
            onClick={handleAddNewTag}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors flex items-center gap-1 text-sm"
          >
            <Plus className="w-4 h-4" />
            追加
          </button>
        </div>
      </div>

      {/* よく使うタグ */}
      {suggestedTags.length > 0 && (
        <div className="mb-6">
          <h4 className="text-sm text-gray-700 mb-3">よく使うタグ</h4>
          <div className="flex flex-wrap gap-2">
            {suggestedTags.map((tag, index) => (
              <button
                key={index}
                onClick={() => addTag(tag)}
                className="px-3 py-1 bg-gray-100 text-gray-700 rounded-full text-sm hover:bg-gray-200 transition-colors flex items-center gap-1"
              >
                <Plus className="w-3 h-3" />
                {tag}
              </button>
            ))}
          </div>
        </div>
      )}

      {/* AI提案タグ */}
      <div className="mb-6 p-4 bg-purple-50 border border-purple-200 rounded-lg">
        <h4 className="text-sm text-purple-900 mb-3 flex items-center gap-2">
          <Sparkles className="w-4 h-4" />
          AI提案タグ
        </h4>
        <div className="flex flex-wrap gap-2 mb-3">
          {aiSuggestedTags.filter(tag => !tags.includes(tag)).map((tag, index) => (
            <button
              key={index}
              onClick={() => addTag(tag)}
              className="px-3 py-1 bg-purple-100 text-purple-700 rounded-full text-sm hover:bg-purple-200 transition-colors flex items-center gap-1"
            >
              <Plus className="w-3 h-3" />
              {tag}
            </button>
          ))}
        </div>
        <p className="text-xs text-purple-700">
          会議の内容からAIが提案したタグです。必要に応じて追加してください。
        </p>
      </div>

      {/* タグのヒント */}
      <div className="mb-6 p-3 bg-blue-50 border border-blue-200 rounded-lg">
        <p className="text-xs text-blue-800">
          💡 <strong>タグ活用のヒント：</strong> 
          会議の種類（定例会、臨時会議）、議題のカテゴリ（予算、イベント、防災など）、
          重要度（重要、緊急）などでタグ付けすると、後から検索しやすくなります。
        </p>
      </div>

      {/* アクションボタン */}
      <div className="flex gap-3">
        <button
          onClick={onClose}
          className="flex-1 px-4 py-2 text-gray-700 bg-white border border-gray-300 rounded-lg hover:bg-gray-50 transition-colors"
        >
          キャンセル
        </button>
        <button
          onClick={handleSave}
          className="flex-1 px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
        >
          保存
        </button>
      </div>

      {/* プライバシー通知 */}
      <div className="mt-4 p-3 bg-green-50 border border-green-200 rounded-lg">
        <p className="text-xs text-green-800">
          🔒 タグ情報もすべてこのPC内に保存されます。外部には送信されません。
        </p>
      </div>
    </div>
  );
}

import { Lock, Cpu, WifiOff } from 'lucide-react';

interface PrivacyStatusBarProps {
  isRecording: boolean;
}

export function PrivacyStatusBar({ isRecording }: PrivacyStatusBarProps) {
  return (
    <div className="bg-green-50 border-t-2 border-green-600 px-6 py-3">
      <div className="max-w-7xl mx-auto flex items-center justify-between">
        <div className="flex items-center gap-6">
          <div className="flex items-center gap-2 text-green-800">
            <Lock className="w-4 h-4" />
            <span className="text-sm">処理場所：このPC</span>
          </div>
          <div className="flex items-center gap-2 text-green-800">
            <Cpu className="w-4 h-4" />
            <span className="text-sm">AI：ローカルLLM {isRecording && <span className="inline-block w-2 h-2 bg-green-600 rounded-full ml-1 animate-pulse" />}</span>
          </div>
          <div className="flex items-center gap-2 text-green-800">
            <WifiOff className="w-4 h-4" />
            <span className="text-sm">通信：なし</span>
          </div>
        </div>
        
        {isRecording && (
          <div className="flex items-center gap-2 px-3 py-1 bg-green-100 rounded-full">
            <span className="relative flex h-3 w-3">
              <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-green-400 opacity-75"></span>
              <span className="relative inline-flex rounded-full h-3 w-3 bg-green-600"></span>
            </span>
            <span className="text-sm text-green-800">NPU使用中</span>
          </div>
        )}
      </div>
    </div>
  );
}

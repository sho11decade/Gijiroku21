import { useState } from 'react';
import { Shield, Server, Edit3 } from 'lucide-react';

interface OnboardingFlowProps {
  onComplete: () => void;
}

export function OnboardingFlow({ onComplete }: OnboardingFlowProps) {
  const [currentStep, setCurrentStep] = useState(0);

  const steps = [
    {
      icon: Shield,
      title: 'このアプリは\n音声を外部に送信しません',
      description: '録音した音声データは、あなたのパソコンの中だけで処理されます。クラウドサーバーにアップロードされることはありません。',
      color: 'text-green-600',
      bgColor: 'bg-green-100',
    },
    {
      icon: Server,
      title: 'AIはこのPCの中だけで動きます\n（インターネット不要）',
      description: 'NPU（AI専用チップ）を活用し、高速かつプライベートに処理を行います。オフラインでも完全に動作します。',
      color: 'text-blue-600',
      bgColor: 'bg-blue-100',
    },
    {
      icon: Edit3,
      title: '会議に集中してください\n議事録は後から整えられます',
      description: 'AIが自動で要点を抽出しますが、最終的な編集権限はあなたにあります。AIは提案するだけです。',
      color: 'text-purple-600',
      bgColor: 'bg-purple-100',
    },
  ];

  const currentStepData = steps[currentStep];
  const Icon = currentStepData.icon;

  const handleNext = () => {
    if (currentStep < steps.length - 1) {
      setCurrentStep(currentStep + 1);
    } else {
      onComplete();
    }
  };

  const handleSkip = () => {
    onComplete();
  };

  return (
    <div className="min-h-screen bg-gradient-to-br from-blue-50 to-indigo-100 flex items-center justify-center p-6">
      <div className="max-w-2xl w-full bg-white rounded-2xl shadow-2xl p-12">
        {/* アイコン */}
        <div className={`w-24 h-24 ${currentStepData.bgColor} rounded-full flex items-center justify-center mx-auto mb-8`}>
          <Icon className={`w-12 h-12 ${currentStepData.color}`} />
        </div>

        {/* タイトル */}
        <h1 className="text-center text-gray-900 mb-6 whitespace-pre-line">
          {currentStepData.title}
        </h1>

        {/* 説明文 */}
        <p className="text-center text-gray-600 mb-12 leading-relaxed">
          {currentStepData.description}
        </p>

        {/* ステップインジケーター */}
        <div className="flex justify-center gap-2 mb-8">
          {steps.map((_, index) => (
            <div
              key={index}
              className={`h-2 rounded-full transition-all ${
                index === currentStep
                  ? 'w-8 bg-blue-600'
                  : 'w-2 bg-gray-300'
              }`}
            />
          ))}
        </div>

        {/* ボタン */}
        <div className="flex gap-4">
          <button
            onClick={handleSkip}
            className="flex-1 px-6 py-3 text-gray-600 hover:bg-gray-100 rounded-lg transition-colors"
          >
            スキップ
          </button>
          <button
            onClick={handleNext}
            className="flex-1 px-6 py-3 bg-blue-600 text-white hover:bg-blue-700 rounded-lg transition-colors"
          >
            {currentStep < steps.length - 1 ? '次へ' : '始める'}
          </button>
        </div>
      </div>
    </div>
  );
}

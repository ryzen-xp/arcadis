import { cn } from '@/lib/utils';
import { ReactNode } from 'react';

interface FeatureCardProps {
  icon?: ReactNode;
  title?: string;
  description?: string;
  buttonText?: string;
  buttonAction?: () => void;
  className?: string;
  btnClassNames?: string;
}

const FeatureCard = ({
  icon,
  title,
  description,
  buttonText,
  buttonAction,
  className,
  btnClassNames,
}: FeatureCardProps) => {
  return (
    <div
      className={cn(
        'bg-[#1A1B4B] border border-[#2A2B5B] rounded-xl p-6',
        className
      )}
    >
      <div className="flex flex-col h-full">
        <div className="flex items-center gap-2">
          {icon && <div className="mb-4 text-[#22C55E] h-6 w-6">{icon}</div>}

          {title && (
            <h3
              className={
                'text-xl font-semibold text-white mb-3 text-[0.9769rem]'
              }
            >
              {title}
            </h3>
          )}
        </div>

        {description && (
          <p className="text-[#D1D5DB] mb-6 text-[0.8475rem] leading-5">
            {description}
          </p>
        )}

        {buttonText && (
          <div>
            <button
              onClick={buttonAction}
              className={cn(
                'bg-[#16A34A] hover:bg-green-600 text-white font-medium py-2 px-4 rounded transition-colors text-[0.8544rem]',
                btnClassNames
              )}
            >
              {buttonText}
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default FeatureCard;

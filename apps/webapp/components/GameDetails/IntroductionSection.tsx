// components/IntroductionSection.tsx
import React from 'react';

interface IntroductionSectionProps {
  title?: string;
  content: string;
}

const IntroductionSection = ({ title = "Introduction", content }: IntroductionSectionProps) => {
  return (
    <div className="my-4 sm:my-6 lg:my-8">
      <h3 className="text-xl sm:text-2xl lg:text-3xl font-bold mb-2 sm:mb-3 lg:mb-4">{title}</h3>
      <p className="text-[#D1D5DB] text-sm sm:text-base font-normal leading-relaxed max-w-4xl">
        {content}
      </p>
    </div>
  );
};

export default IntroductionSection;
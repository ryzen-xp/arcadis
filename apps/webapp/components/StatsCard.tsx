import React from 'react';

interface StatsCardProps {
  value: string;
  label: string;
  isCurrency?: boolean;
  isPercentage?: boolean;
  className?: string;
}

const StatsCard: React.FC<StatsCardProps> = ({
  value,
  label,
  isCurrency = false,
  isPercentage = false,
  className = '',
}) => {

  const formatValue = () => {
    if (isCurrency) {
      return `$${value}`;
    } else if (isPercentage) {
      return `${value}%`;
    }
    return value;
  };

  return (
    <div
      className={`bg-[#2A2B5B] rounded-[12px] p-5 hover:scale-105 transition-transform duration-200 ${className}`}
    >
      <p className="font-bold text-[29.3px] leading-[36px] text-white mb-1">
        {formatValue()}
      </p>
      <p className="font-normal text-[15.13px] leading-[24px] text-[#D1D5DB]">
        {label}
      </p>
    </div>
  );
};

// Main component to display a group of stats cards
const StatsCardGroup: React.FC = () => {
  return (
    <div className="grid grid-cols-1 md:grid-cols-3 gap-5">
      <StatsCard value="15,000+" label="Active Players" />
      <StatsCard value="500+" label="Teams Registered" />
      <StatsCard value="50K+" label="Price Pool" isCurrency />
    </div>
  );
};

export default StatsCardGroup;
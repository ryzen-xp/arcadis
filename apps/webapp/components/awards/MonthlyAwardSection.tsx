import AwardCard from './AwardCard';
import { CalendarIcon } from 'lucide-react';

type Award = {
  type: string;
  name: string;
  category: string;
  icon: 'trophy' | 'star';
};

type MonthlyAwardsSectionProps = {
  month: string;
  awards: Award[];
};

export default function MonthlyAwardsSection({ month, awards }: MonthlyAwardsSectionProps) {
  return (
    <div className="bg-[#111827] md:shadow-xl rounded-lg p-6 shadow-lg">
      <div className="flex items-center mb-6">
        <CalendarIcon className="w-6 h-6 text-blue-400 mr-2" />
        <h2 className="text-xl font-semibold text-white">{month}</h2>
      </div>
      
      <div className="space-y-4">
        {awards.map((award, index) => (
          <AwardCard 
            key={index} 
            type={award.type} 
            name={award.name} 
            category={award.category} 
            icon={award.icon} 
          />
        ))}
      </div>
    </div>
  );
}
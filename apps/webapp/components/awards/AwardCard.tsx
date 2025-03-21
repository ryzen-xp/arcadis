import { TrophyIcon, StarIcon } from "lucide-react";

type AwardCardProps = {
  type: string;
  name: string;
  category: string;
  icon: "trophy" | "star";
};

export default function AwardCard({
  type,
  name,
  category,
  icon,
}: AwardCardProps) {
  return (
    <>
      <div
        className={`rounded-full w-[3rem] h-[3rem] flex justify-center items-center p-2 mr-3 bg-gradient-to-r ${icon === "trophy" ? "from-[#FACC15] to-[#D97706]" : "from-[#60A5FA] to-[#4F46E5]"}`}
      >
        {icon === "trophy" ? (
          <TrophyIcon className="w-6 h-6 text-[#0A0A0A]" />
        ) : (
          <StarIcon className="w-6 h-6 text-[#0A0A0A]" />
        )}
      </div>
      <div
        className="bg-[#1F2937] border border-gray-800 rounded-lg p-4 cursor-pointer hover:bg-[#283446] transition-colors"
        onClick={() => console.log(`Navigating to ${name} details...`)}
      >
        <div className="text-[#9CA3AF] text-sm mb-2">{type}</div>
        <div className="flex items-start">
          <div>
            <h3 className="font-semibold text-base text-white">{name}</h3>
            <p className="text-[#9CA3AF] text-sm">{category}</p>
          </div>
        </div>
      </div>
    </>
  );
}

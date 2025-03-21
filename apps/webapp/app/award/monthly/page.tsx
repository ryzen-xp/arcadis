"use client";
import MonthlyAwardsSection from "@/components/awards/MonthlyAwardSection";

export default function MonthlyAwardsPage() {
  const awardsData = {
    december2024: {
      month: "December 2024",
      awards: [
        {
          type: "Game of the Month",
          name: "Crypto Legends",
          category: "Epic Quest",
          icon: "trophy" as const,
        },
        {
          type: "Rising Star",
          name: "New Horizons",
          category: "Future Quest",
          icon: "star" as const,
        },
      ],
    },
    november2024: {
      month: "November 2024",
      awards: [
        {
          type: "Game of the Month",
          name: "StarkWorld",
          category: "Virtual Reality",
          icon: "trophy" as const,
        },
        {
          type: "Rising Star",
          name: "Tech Titans",
          category: "Code Warriors",
          icon: "star" as const,
        },
      ],
    },
  };

  return (
    <div className="min-h-screen p-6 md:p-8 lg:p-12  bg-gradient-to-tr from-[#164E63] via-[#060A32] to-[#040726]">
      <div className="max-w-7xl mx-auto">
        <div className="text-center mb-12">
          <h1 className="text-4xl md:text-5xl font-bold mb-3 bg-gradient-to-r from-[#2563EB] to-[#9333EA] text-transparent bg-clip-text">
            Monthly Awards
          </h1>
          <p className="text-gray-300">
            Recognizing monthly achievements in Arcadis gaming
          </p>
        </div>

        <div className="grid grid-cols-1 lg:grid-cols-2 gap-8">
          <MonthlyAwardsSection
            month={awardsData.december2024.month}
            awards={awardsData.december2024.awards}
          />
          <MonthlyAwardsSection
            month={awardsData.november2024.month}
            awards={awardsData.november2024.awards}
          />
        </div>
      </div>
    </div>
  );
}

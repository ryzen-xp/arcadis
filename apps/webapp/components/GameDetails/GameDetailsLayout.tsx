"use client"
import React, { useState, ReactNode } from 'react';
import Image from 'next/image';
import TabNavigation from './TabNavigation';
import GameHeroBanner from './GameHeroBanner';

interface GameDetailsLayoutProps {
  gameTitle: string;
  studioName: string;
  bannerImage: string;
  stellarImage: string;
  bannerTagline: string;
  introductionText: string;
  children: ReactNode;
  initialActiveTab?: string;
}

const GameDetailsLayout = ({ 
  gameTitle,
  studioName,
  bannerImage,
  stellarImage,
  introductionText,
  children,
  initialActiveTab = 'Overview'
}: GameDetailsLayoutProps) => {
  const [activeTab, setActiveTab] = useState(initialActiveTab);
  
  const tabs = [
    { name: 'Overview', icon: 'home' },
    { name: 'Review', icon: 'star' },
    { name: 'Guides', icon: 'book' },
    { name: 'News', icon: 'newspaper' },
    { name: 'Analytics', icon: 'chart-bar' },
    { name: 'Events', icon: 'calendar' }
  ];

  const handleTabChange = (tabName: string) => {
    setActiveTab(tabName);
  };

  return (
    <div className="min-h-screen bg-[linear-gradient(45deg,#164E63_0%,#060A32_50%,#040726_100%)] text-white">
      <div className="container mx-auto px-4 sm:px-6 lg:px-12 py-4 sm:py-6 lg:py-8">
        {/* Game Title and Studio Section */}
        <div className="flex flex-col md:flex-row items-start md:items-center justify-between mb-4 sm:mb-6 lg:mb-8">
          <div className="flex items-center gap-4 md:gap-0">
            <div className="w-20 h-20 sm:w-28 sm:h-28 lg:w-[112px] lg:h-[112px] mb-4 sm:mb-0 sm:mr-4 rounded-lg overflow-hidden">
              <Image 
                src={stellarImage} 
                alt={gameTitle} 
                width={112}
                height={112}
                className="w-full h-full object-contain"
                priority
              />
            </div>
            <div>
              <h1 className="text-2xl sm:text-3xl lg:text-[44px] font-bold">{gameTitle}</h1>
              <p className="text-[#9CA3AF] font-normal text-xs sm:text-sm">{studioName}</p>
            </div>
          </div>
          <div className="flex mt-4 md:mt-0">
            <div className="bg-[#10B981] w-8 h-8 rounded-lg flex items-center justify-center mr-2">
              <span className="font-semibold">2</span>
            </div>
            <button className="bg-white w-8 h-8 rounded-full text-black flex items-center justify-center">
              <svg xmlns="http://www.w3.org/2000/svg" className="h-[13px] w-[13px]" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M11.049 2.927c.3-.921 1.603-.921 1.902 0l1.519 4.674a1 1 0 00.95.69h4.915c.969 0 1.371 1.24.588 1.81l-3.976 2.888a1 1 0 00-.363 1.118l1.518 4.674c.3.922-.755 1.688-1.538 1.118l-3.976-2.888a1 1 0 00-1.176 0l-3.976 2.888c-.783.57-1.838-.197-1.538-1.118l1.518-4.674a1 1 0 00-.363-1.118l-3.976-2.888c-.784-.57-.38-1.81.588-1.81h4.914a1 1 0 00.951-.69l1.519-4.674z" />
              </svg>
            </button>
          </div>
        </div>
        
        {/* Tab Navigation - Scrollable on mobile */}
        <div className="overflow-x-auto -mx-4 sm:mx-0">
          <div className="min-w-max sm:min-w-0 px-4 sm:px-0">
            <TabNavigation 
              tabs={tabs} 
              activeTab={activeTab} 
              onTabChange={handleTabChange} 
            />
          </div>
        </div>
        
        {/* Hero Banner */}
        <GameHeroBanner 
          bannerImage={bannerImage}
          gameTitle={gameTitle}
          introductionText={introductionText}
        />
        
        {/* Main Content Area */}
        <div className="mt-4 sm:mt-6 lg:mt-8">
          {children}
        </div>
      </div>
    </div>
  );
};

export default GameDetailsLayout;
"use client";

// components/TabNavigation.tsx
import React, { useEffect, useState } from 'react';
import { ReactNode } from 'react';

interface Tab {
  name: string;
  icon: string;
}

interface TabNavigationProps {
  tabs: Tab[];
  activeTab?: string;
  onTabChange: (tabName: string) => void;
}

const TabNavigation = ({ tabs, activeTab, onTabChange }: TabNavigationProps) => {
  const [isMobile, setIsMobile] = useState(false);
  
  // Handle responsive behavior safely
  useEffect(() => {
    const checkIfMobile = () => {
      setIsMobile(window.innerWidth < 640);
    };
    
    // Initial check
    checkIfMobile();
    
    // Add event listener
    window.addEventListener('resize', checkIfMobile);
    
    // Cleanup
    return () => window.removeEventListener('resize', checkIfMobile);
  }, []);

  // Determine spacing based on screen size
  const getSpacing = () => isMobile ? 80 : 100;

  // Map of icon names to their JSX representations
  const iconMap: { [key: string]: ReactNode } = {
    'home': (
      <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 sm:h-5 sm:w-5 mr-1 sm:mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
      </svg>
    ),
    'star': (
        <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 sm:h-5 sm:w-5 mr-1 sm:mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 6.253v13m0-13C10.832 5.477 9.246 5 7.5 5S4.168 5.477 3 6.253v13C4.168 18.477 5.754 18 7.5 18s3.332.477 4.5 1.253m0-13C13.168 5.477 14.754 5 16.5 5c1.747 0 3.332.477 4.5 1.253v13C19.832 18.477 18.247 18 16.5 18c-1.746 0-3.332.477-4.5 1.253" />
        </svg>
    ),
    'book': (
      <svg className="h-4 w-4 sm:h-5 sm:w-5 mr-1 sm:mr-2" viewBox="0 0 17 17" fill="none" xmlns="http://www.w3.org/2000/svg">
      <path d="M4.46002 8.75H7.12669" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
      <path d="M5.79337 7.41669V10.0834" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
      <path d="M10.46 9.41669H10.4667" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
      <path d="M12.46 8.08331H12.4667" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
      <path d="M13.7934 4.75H3.1267C2.39032 4.75 1.79337 5.34695 1.79337 6.08333V11.4167C1.79337 12.153 2.39032 12.75 3.1267 12.75H13.7934C14.5297 12.75 15.1267 12.153 15.1267 11.4167V6.08333C15.1267 5.34695 14.5297 4.75 13.7934 4.75Z" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
  </svg>
    ),
    'newspaper': (
        <svg className="h-4 w-4 sm:h-5 sm:w-5 mr-1 sm:mr-2" viewBox="0 0 17 17" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M3.23668 15.4166H13.9034C14.257 15.4166 14.5961 15.2762 14.8462 15.0261C15.0962 14.7761 15.2367 14.4369 15.2367 14.0833V3.41665C15.2367 3.06302 15.0962 2.72389 14.8462 2.47384C14.5961 2.22379 14.257 2.08331 13.9034 2.08331H5.90335C5.54973 2.08331 5.21059 2.22379 4.96054 2.47384C4.71049 2.72389 4.57002 3.06302 4.57002 3.41665V14.0833C4.57002 14.4369 4.42954 14.7761 4.17949 15.0261C3.92944 15.2762 3.59031 15.4166 3.23668 15.4166ZM3.23668 15.4166C2.88306 15.4166 2.54392 15.2762 2.29388 15.0261C2.04383 14.7761 1.90335 14.4369 1.90335 14.0833V8.08331C1.90335 7.34998 2.50335 6.74998 3.23668 6.74998H4.57002" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M12.57 10.0833H7.23666" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M10.57 12.75H7.23666" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M7.23666 4.75H12.57V7.41667H7.23666V4.75Z" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
    ),
    'chart-bar': (
        <svg className="h-4 w-4 sm:h-5 sm:w-5 mr-1 sm:mr-2" viewBox="0 0 17 17" fill="none" xmlns="http://www.w3.org/2000/svg">
            <path d="M2.32999 2.75V13.4167C2.32999 13.7703 2.47046 14.1094 2.72051 14.3595C2.97056 14.6095 3.3097 14.75 3.66332 14.75H14.33" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M4.99664 11.4167H10.33" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M4.99664 8.08331H12.9966" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
            <path d="M4.99664 4.75H6.99664" stroke="#9CA3AF" stroke-width="1.33333" stroke-linecap="round" stroke-linejoin="round"/>
        </svg>
    ),
    'calendar': (
      <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4 sm:h-5 sm:w-5 mr-1 sm:mr-2" fill="none" viewBox="0 0 24 24" stroke="currentColor">
        <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8 7V3m8 4V3m-9 8h10M5 21h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v12a2 2 0 002 2z" />
      </svg>
    )
  };

  return (
    <div className="flex overflow-x-auto scrollbar-hide pb-2 mb-4 sm:mb-6 border-t border-[#202448] relative">
      {/* Animated indicator for active tab */}
      <div className="absolute bottom-0 left-0 h-[3px] transform transition-all duration-300 ease-in-out" 
           style={{ 
             width: '36px', 
             transform: `translateX(${tabs.findIndex(tab => tab.name === activeTab) * getSpacing()}px)`,
             opacity: activeTab ? 1 : 0
           }} />
      
      {tabs.map((tab) => (
        <button
          key={tab.name}
          className={`relative flex items-center px-2 sm:px-4 py-1 sm:py-2 mt-4 sm:mt-6 mr-2 sm:mr-4 font-medium text-xs sm:text-sm whitespace-nowrap transition-all duration-300 ease-in-out ${
            activeTab === tab.name
              ? 'text-white bg-[#171717] rounded-[6px] sm:rounded-[8px] transform scale-105'
              : 'text-gray-400 hover:text-gray-200 hover:bg-[#171717]/30 hover:rounded-[6px] sm:hover:rounded-[8px]'
          }`}
          onClick={() => onTabChange(tab.name)}
        >
          <span className={`transition-transform duration-300 ease-in-out ${
            activeTab === tab.name ? 'transform scale-110' : ''
          }`}>
            {iconMap[tab.icon]}
          </span>
          
          <span className={`transition-all duration-300 ease-in-out ${
            activeTab === tab.name ? 'font-semibold' : ''
          }`}>
            {tab.name}
          </span>
          
          {/* Subtle glow effect for active tab */}
          {activeTab === tab.name && (
            <span className="absolute inset-0 rounded-[6px] sm:rounded-[8px] animate-pulse bg-blue-500/10 z-[-1]"></span>
          )}
        </button>
      ))}
    </div>
  );
};

export default TabNavigation;
// components/GameHeroBanner.tsx
import React from 'react';
import IntroductionSection from './IntroductionSection';
import { motion } from 'framer-motion';
import Image from 'next/image';

interface GameHeroBannerProps {
  bannerImage: string;
  gameTitle: string;
  introductionText: string;
}

const GameHeroBanner = ({ bannerImage, gameTitle, introductionText }: GameHeroBannerProps) => {
  // Game info data for sidebar
  const gameInfo = {
    gameType: "An epic space exploration RPG where your choices shape the fate of the galaxy.",
    tags: ["RPG", "SPACE", "EXPLORATION"],
    developer: "Cosmic Games Studio",
    developerLink: "#",
    status: "upcoming",
    community: "750K+",
    players: "250,000+",
    platforms: ["PC", "Console", "Mobile"]
  };

  const fadeInUp = {
    hidden: { opacity: 0, y: 20 },
    visible: { opacity: 1, y: 0 }
  };

  const staggerChildren = {
    visible: {
      transition: {
        staggerChildren: 0.1
      }
    }
  };

  return (
    <motion.div 
      initial="hidden"
      animate="visible"
      variants={staggerChildren}
      className="flex flex-col md:flex-row items-center justify-between w-full h-full rounded-lg overflow-hidden mb-8 gap-16"
    >
       <motion.div variants={fadeInUp} className="w-full md:basis-9/12">
            {/* Introduction Section */}
            <IntroductionSection title="Introduction" content={introductionText} />
            {/* Main Banner Image */}
            <motion.div 
              className="w-full relative aspect-[16/9]"
              initial={{ scale: 0.95, opacity: 0 }}
              animate={{ scale: 1, opacity: 1 }}
              transition={{ duration: 0.5 }}
            >
                <Image
                src={bannerImage}
                alt={gameTitle}
                fill
                className="object-contain"
                priority
                sizes="(max-width: 768px) 100vw, (max-width: 1200px) 75vw"
                />
            </motion.div>
       </motion.div>

        {/* Game Info Sidebar */}
        <motion.div 
          className="w-full md:basis-3/12 md:min-w-[348px] mt-6 md:mt-0 px-4 md:px-0"
          variants={fadeInUp}
          initial={{ opacity: 0, x: 50 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ duration: 0.6, ease: "easeOut" }}
        >
        
        <motion.button 
          whileHover={{ scale: 1.02, backgroundColor: "#f8f8f8" }}
          whileTap={{ scale: 0.98 }}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.4, delay: 0.2 }}
          className="w-full bg-white text-[#171717] font-medium text-sm py-2 rounded-md mb-4 transition-all duration-300 hover:bg-gray-200 hover:shadow-lg"
        >
            Write Your <br className="hidden md:block" /> Review
        </motion.button>

        <motion.div 
          className="bg-[#1a1e3d] border border-[#E5E5E5] rounded-lg p-6"
          initial={{ opacity: 0, scale: 0.95 }}
          animate={{ opacity: 1, scale: 1 }}
          transition={{ duration: 0.5, delay: 0.3 }}
        >
            <div className="mb-6">
            <motion.p 
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.4, delay: 0.4 }}
              className="text-[#9CA3AF] font-normal text-sm mb-4"
            >
              {gameInfo.gameType}
            </motion.p>
            <motion.div 
              className="flex flex-wrap gap-2"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.4, delay: 0.5 }}
            >
                {gameInfo.tags.map((tag, index) => (
                <motion.span 
                  key={tag}
                  initial={{ opacity: 0, scale: 0.8 }}
                  animate={{ opacity: 1, scale: 1 }}
                  transition={{ delay: 0.6 + index * 0.1 }}
                  whileHover={{ scale: 1.1, backgroundColor: "rgba(16, 185, 129, 0.3)" }}
                  className="bg-green-500/20 text-green-400 px-3 py-1 text-xs font-semibold rounded"
                >
                    {tag}
                </motion.span>
                ))}
            </motion.div>
            </div>
            
            <motion.div 
              className="space-y-6 pt-4"
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              transition={{ duration: 0.4, delay: 0.7 }}
            >
              {[
                { label: "Developer", value: gameInfo.developer, isLink: true },
                { label: "Status", value: gameInfo.status },
                { label: "Community", value: gameInfo.community },
                { label: "Players", value: gameInfo.players },
                { label: "Platforms", value: gameInfo.platforms.join(", ") }
              ].map((item, index) => (
                <motion.div 
                  key={item.label}
                  initial={{ opacity: 0, x: -20 }}
                  animate={{ opacity: 1, x: 0 }}
                  transition={{ delay: 0.8 + index * 0.1 }}
                  className="flex justify-between items-center"
                >
                  <span className="text-[#9CA3AF] font-normal text-base">{item.label}</span>
                  {item.isLink ? (
                    <motion.a 
                      href={gameInfo.developerLink}
                      whileHover={{ scale: 1.05, color: "#60A5FA" }}
                      className="text-[#3B82F6] font-normal text-base hover:underline"
                    >
                      {item.value}
                    </motion.a>
                  ) : (
                    <span className="text-[#3B82F6] font-normal text-base">{item.value}</span>
                  )}
                </motion.div>
              ))}
            </motion.div>
            
            <motion.button 
              whileHover={{ scale: 1.02, backgroundColor: "#f8f8f8" }}
              whileTap={{ scale: 0.98 }}
              initial={{ opacity: 0, y: 20 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ duration: 0.4, delay: 1.3 }}
              className="w-full bg-white text-[#171717] text-sm font-medium py-2 rounded-md mt-6 flex items-center justify-center transition-all duration-300 hover:bg-gray-200 hover:shadow-lg"
            >
              <motion.svg 
                xmlns="http://www.w3.org/2000/svg" 
                className="h-[16px] w-[16px] mr-4"
                whileHover={{ rotate: 180 }}
                transition={{ duration: 0.3 }}
                fill="none" 
                viewBox="0 0 24 24" 
                stroke="currentColor"
              >
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M8.684 13.342C8.886 12.938 9 12.482 9 12c0-.482-.114-.938-.316-1.342m0 2.684a3 3 0 110-2.684m0 2.684l6.632 3.316m-6.632-6l6.632-3.316m0 0a3 3 0 105.367-2.684 3 3 0 00-5.367 2.684zm0 9.316a3 3 0 105.368 2.684 3 3 0 00-5.368-2.684z" />
              </motion.svg>
              Share
            </motion.button>
        </motion.div>
        
        <motion.button 
          whileHover={{ scale: 1.02, backgroundColor: "rgba(31, 41, 55, 0.5)" }}
          whileTap={{ scale: 0.98 }}
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.4, delay: 1.4 }}
          className="w-full bg-transparent border border-transparent text-white font-medium text-xs py-2 rounded-md mt-4 flex items-center justify-center transition-all duration-300 hover:border-gray-500"
        >
            <motion.svg 
              className="h-[16px] w-[16px] mr-2"
              whileHover={{ scale: 1.2 }}
              transition={{ duration: 0.2 }}
              viewBox="0 0 17 16" 
              fill="none" 
              xmlns="http://www.w3.org/2000/svg"
            >
                <g clipPath="url(#clip0_1_1096)">
                <path d="M14.71 10C14.71 10.3536 14.5695 10.6928 14.3195 10.9428C14.0694 11.1929 13.7303 11.3333 13.3767 11.3333H5.37669L2.71002 14V3.33333C2.71002 2.97971 2.8505 2.64057 3.10055 2.39052C3.35059 2.14048 3.68973 2 4.04336 2H13.3767C13.7303 2 14.0694 2.14048 14.3195 2.39052C14.5695 2.64057 14.71 2.97971 14.71 3.33333V10Z" stroke="white" strokeWidth="1.33333" strokeLinecap="round" strokeLinejoin="round"/>
                </g>
                <defs>
                <clipPath id="clip0_1_1096">
                <rect width="16" height="16" fill="white" transform="translate(0.710022)"/>
                </clipPath>
                </defs>
            </motion.svg>
            Feedback
        </motion.button>
        </motion.div>
    </motion.div>
  );
};

export default GameHeroBanner;
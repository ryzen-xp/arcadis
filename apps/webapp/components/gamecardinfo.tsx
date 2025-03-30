import React from "react";

// TypeScript Interface for component props
interface GameInfoCardProps {
  // Required game data
  title: string;
  description: string;
  developer: {
    name: string;
    url: string;
    logo?: string;
  };
  status: "upcoming" | "active" | "completed";
  
  // Optional game data
  image?: string;
  rating?: number;
  players?: number;
  community?: number;
  categories?: string[];
  platforms?: string[];
  
  // Optional display settings
  showFeedback?: boolean;
  showReviewButton?: boolean;
  cardWidth?: string;
  customActions?: React.ReactNode;
}

const GameInfoCard: React.FC<GameInfoCardProps> = ({
  title,
  description,
  developer,
  status,
  image,
  rating,
  players,
  community,
  categories = [],
  platforms = [],
  showFeedback = false,
  showReviewButton = true,
  cardWidth = "360px",
  customActions,
}) => {
  // Format numbers to K/M notation
  const formatNumber = (num?: number) => {
    if (!num) return "N/A";
    if (num >= 1_000_000) return (num / 1_000_000).toFixed(1) + "M+";
    if (num >= 1_000) return (num / 1_000).toFixed(0) + "K+";
    return num.toString();
  };

  // Get status color
  const getStatusColor = () => {
    switch (status) {
      case "active": return "bg-green-500";
      case "upcoming": return "bg-blue-500";
      case "completed": return "bg-gray-500";
      default: return "bg-gray-500";
    }
  };

  return (
    <div className="p-4"> {/* Added outer container with padding */}
      <div 
        className="bg-[#162436] text-white rounded-xl shadow-lg overflow-hidden flex flex-col m-auto"
        style={{ width: cardWidth, maxWidth: "100%" }} /* Added max-width */
      >
        {/* Review Button */}
        {showReviewButton && (
          <div className="px-5 pt-5 pb-2">
            <button className="bg-white text-gray-900 font-semibold py-2 px-4 rounded-lg w-full hover:bg-gray-100 transition">
              Write Your Review
            </button>
          </div>
        )}
        
        {/* Game Image with Status */}
        {image && (
          <div className="relative px-5 pb-3">
            <img 
              src={image} 
              alt={title} 
              className="rounded-lg w-full h-44 object-cover"
            />
            <div className="absolute top-3 left-8">
              <span className={`${getStatusColor()} text-white text-xs px-3 py-1 rounded-md font-semibold`}>
                {status.charAt(0).toUpperCase() + status.slice(1)}
              </span>
            </div>
          </div>
        )}
        
        {/* Game Info */}
        <div className="px-5 pb-4 flex-grow flex flex-col gap-3">
          {/* Title, Developer & Rating */}
          <div className="flex justify-between items-start">
            <div className="flex items-start gap-3">
              {developer.logo && (
                <img 
                  src={developer.logo} 
                  alt={developer.name} 
                  className="w-10 h-10 rounded-lg object-cover"
                />
              )}
              <div>
                <h3 className="text-xl font-bold">{title}</h3>
                <a 
                  href={developer.url} 
                  className="text-blue-400 text-sm hover:underline"
                >
                  {developer.name}
                </a>
              </div>
            </div>
            
            {rating !== undefined && (
              <div className="bg-green-500 text-white w-8 h-8 flex items-center justify-center rounded-md font-bold">
                {rating}
              </div>
            )}
          </div>
          
          {/* Description */}
          <p className="text-gray-400 text-sm">{description}</p>
          
          {/* Stats */}
          <div className="space-y-1">
            {players !== undefined && (
              <div className="flex justify-between items-center">
                <span className="text-gray-400">Players</span>
                <span className="font-medium">{formatNumber(players)}</span>
              </div>
            )}
            
            {community !== undefined && (
              <div className="flex justify-between items-center">
                <span className="text-gray-400">Community</span>
                <span className="font-medium">{formatNumber(community)}</span>
              </div>
            )}
          </div>
          
          {/* Categories */}
          {categories.length > 0 && (
            <div className="flex flex-wrap gap-2">
              {categories.map((category, index) => (
                <span 
                  key={index} 
                  className="bg-[#1E3A50] text-green-500 text-xs px-3 py-1 rounded-md uppercase"
                >
                  {category}
                </span>
              ))}
            </div>
          )}
          
          {/* Platforms */}
          {platforms.length > 0 && (
            <div>
              <span className="text-gray-400 text-sm">Platforms</span>
              <div className="flex flex-wrap gap-2 mt-1">
                {platforms.map((platform, index) => (
                  <span 
                    key={index} 
                    className="bg-[#1E3A50] text-blue-400 text-xs px-3 py-1 rounded-md"
                  >
                    {platform}
                  </span>
                ))}
              </div>
            </div>
          )}
          
          {/* Custom Actions */}
          {customActions}
        </div>
        
        {/* Action Buttons - Bottom Row */}
        <div className="flex items-center px-5 pb-5 pt-2 gap-2">
          {/* Home Button */}
          <button className="w-10 h-10 border border-gray-700 rounded-lg flex items-center justify-center hover:bg-gray-800 transition" aria-label="Home">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M3 9L12 2L21 9V20C21 20.5304 20.7893 21.0391 20.4142 21.4142C20.0391 21.7893 19.5304 22 19 22H5C4.46957 22 3.96086 21.7893 3.58579 21.4142C3.21071 21.0391 3 20.5304 3 20V9Z" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
              <path d="M9 22V12H15V22" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
          </button>
          
          {/* Plus Button */}
          <button className="w-10 h-10 border border-gray-700 rounded-lg flex items-center justify-center hover:bg-gray-800 transition" aria-label="Add">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 2C6.48 2 2 6.48 2 12C2 17.52 6.48 22 12 22C17.52 22 22 17.52 22 12C22 6.48 17.52 2 12 2ZM17 13H13V17H11V13H7V11H11V7H13V11H17V13Z" fill="white"/>
            </svg>
          </button>
          
          {/* Paper Plane Button */}
          <button className="w-10 h-10 border border-gray-700 rounded-lg flex items-center justify-center hover:bg-gray-800 transition" aria-label="Send">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M22 2L11 13" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
              <path d="M22 2L15 22L11 13L2 9L22 2Z" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
          </button>
          
          <div className="flex-grow"></div>
          
          {/* Globe Button */}
          <button className="w-10 h-10 border border-gray-700 rounded-lg flex items-center justify-center hover:bg-gray-800 transition" aria-label="Global">
            <svg width="20" height="20" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
              <path d="M12 22C17.5228 22 22 17.5228 22 12C22 6.47715 17.5228 2 12 2C6.47715 2 2 6.47715 2 12C2 17.5228 6.47715 22 12 22Z" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
              <path d="M2 12H22" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
              <path d="M12 2C14.5013 4.73835 15.9228 8.29203 16 12C15.9228 15.708 14.5013 19.2616 12 22C9.49872 19.2616 8.07725 15.708 8 12C8.07725 8.29203 9.49872 4.73835 12 2V2Z" stroke="white" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"/>
            </svg>
          </button>
        </div>
      </div>
    </div>
  );
};

export default GameInfoCard;
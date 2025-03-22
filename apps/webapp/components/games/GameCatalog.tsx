import React from 'react'
import GameCard from '../ui/GameCard'

const GameCatalog = () => {
    const stellarOdysseyProps = {
        rating: 9,
        statusLabel: "Active",
        statusColor: "#10B981", // Green color for the active status
        bannerImage: "/image 2.png", // Path to the game banner image
        gameIcon: "/gameAvater.png", // Path to the zkops publisher icon
        title: "zKops",
        developer: "Korporations",
        description: "zKops is an engaging puzzle game that puts players' strategic thinking to the test. Set within a dynamic",
        xLink: "https://x.com/zkops", // Twitter/X link
        tgLink: "https://t.me/zkops", // Telegram link
        webLink: "https://zkops.game", // Website link
        tags: ["PUZZLE", "STRATEGIC"], // Game categories/tags
        stats: {
          players: "500,000+",
          community: "1.2" // Will be displayed as "1.2M+"
        },
        hasWebsiteLink: true // Controls whether to show the website globe icon
      };
  return (
    <div className='grid sm:grid-cols-2 lg:grid-cols-3 gap-3 md:gap-6 py-6 '>
      <GameCard {...stellarOdysseyProps}/>
      <GameCard {...stellarOdysseyProps}/>
      <GameCard {...stellarOdysseyProps}/>
      <GameCard {...stellarOdysseyProps}/>
      <GameCard {...stellarOdysseyProps}/>
      <GameCard {...stellarOdysseyProps}/>
    </div>
  )
}

export default GameCatalog

import GameDetailsLayout from '../../components/GameDetails/GameDetailsLayout';

export default function StellarOdysseyPage() {
  // Game details data
  const gameData = {
    title: "Stellar Odyssey",
    studioName: "Cosmic Games Studio", 
    bannerImage: "/stellar-odyssey-banner.png",
    stellarImage: "/gamer-img.png",
    bannerTagline: "EMBARK ON A COSMIC ADVENTURE!",
    introductionText: "Embark on an interstellar journey across a procedurally generated universe. As a deep space explorer, you'll discover ancient artifacts, engage in diplomatic relations with alien civilizations, and uncover the mysteries of a dying universe.",
  };

  return (
    <>
      <GameDetailsLayout
        gameTitle={gameData.title}
        studioName={gameData.studioName}
        bannerImage={gameData.bannerImage}
        stellarImage={gameData.stellarImage}
        bannerTagline={gameData.bannerTagline}
        introductionText={gameData.introductionText}
        initialActiveTab="Overview"
      >
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-4 sm:gap-6 lg:gap-8">
          <div className="lg:col-span-2 w-full lg:w-[895px] lg:max-w-[895px]">
            {/* Recent Transactions Section */}
            <div className="bg-[#1a1b4b] rounded-lg p-4 sm:p-6 mb-4 sm:mb-6 lg:mb-8">
              <h3 className="text-lg sm:text-xl font-bold mb-2">Recent Transactions</h3>
              <p className="text-[#9CA3AF] font-normal text-xs sm:text-sm mb-4">Latest transactions on the network</p>
              
              <div className="overflow-x-auto -mx-4 sm:mx-0">
                <div className="min-w-max sm:min-w-0 px-4 sm:px-0">
                  <table className="w-full">
                    <thead>
                      <tr className="text-left text-gray-400 text-xs sm:text-sm border-b border-[#2a2b5b]">
                        <th className="pb-2 font-medium text-[#9CA3AF]">Hash</th>
                        <th className="pb-2 font-medium text-[#9CA3AF]">Block</th>
                        <th className="pb-2 font-medium text-[#9CA3AF]">Type</th>
                        <th className="pb-2 font-medium text-[#9CA3AF]">Status</th>
                      </tr>
                    </thead>
                    <tbody>
                      <tr className="">
                        <td className="py-2 sm:py-3 pr-2 sm:pr-4 text-xs sm:text-sm font-normal">
                          <div className="text-blue-400 truncate max-w-[120px] sm:max-w-xs text-[#60A5FA]">0x63cf80c89e9bd0655688854aed482f09434f256b0439f44fc2beb620cf625815</div>
                        </td>
                        <td className="py-2 sm:py-3 pr-2 sm:pr-4 text-xs sm:text-sm font-normal">293739</td>
                        <td className="py-2 sm:py-3 pr-2 sm:pr-4 text-xs sm:text-sm font-normal">INVOKE</td>
                        <td className="py-2 sm:py-3">
                          <span className="bg-green-500/20 text-green-400 text-[10px] sm:text-xs font-semibold px-1.5 sm:px-2 py-0.5 sm:py-1 rounded">ACCEPTED_ON_L1</span>
                        </td>
                      </tr>
                    </tbody>
                  </table>
                </div>
              </div>
            </div>
          </div>
          
        </div>
      </GameDetailsLayout>
    </>
  );
}
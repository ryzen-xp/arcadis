'use client';

import FeatureCard from '@/components/ui/featureCard';
import GameCard from '@/components/ui/GameCard';
import { Users } from 'lucide-react';

const Page = () => {
  return (
    // <TestGameCard />

    <TeamsAndGuildsCard />
  );
};

export default Page;
const TeamsAndGuildsCard = () => {
  return (
    <div className=" bg-[#070B1D] h-screen">
      <div className="grid grid-cols-2 gap-6 max-w-[80%] mx-auto pt-20 relative">
        <FeatureCard
          icon={<Users />}
          title="Teams & Guilds"
          description="Join forces with other players or create your own team. Compete in tournaments and climb the rankings."
          buttonText="Register Your Team"
          buttonAction={() => console.log('Team registration clicked')}
        />
      </div>
    </div>
  );
};

const TestGameCard = () => {
  return (
    <div className="w-full min-h-screen flex items-center justify-center p-[300px] bg-[black] ">
      <GameCard
        rating={9}
        statusLabel="Active"
        statusColor="#22C55E"
        bannerImage="/eternal-legends.svg"
        gameIcon="/publisherImage.svg"
        title="Eternal Legends"
        developer="Blockchain Gaming Inc"
        description="A revolutionary MMORPG that combines traditional fantasy with blockchain technology."
        xLink="https://app.onlydust.com/discover"
        tgLink="https://app.onlydust.com/discover"
        webLink="https://app.onlydust.com/discover"
        tags={['MMORPG', 'BLOCKCHAIN', 'FANTASY']}
        hasWebsiteLink={true}
        stats={{
          players: '800,000+',
          community: '1.5M',
        }}
      />
    </div>
  );
};

'use client'
import GameCard from '@/components/ui/GameCard'
import { PageWrapper } from '@/components/PageWrapper'

const Page = () => {
	return (
		<PageWrapper>
			<div className="w-full min-h-screen flex items-center justify-center p-[300px]">
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
		</PageWrapper>
	)
}

// const TestGameCard = () => {
//   return (
//     <div className="w-full min-h-screen flex items-center justify-center p-[300px] bg-[black] ">
//       <GameCard
//         rating={9}
//         statusLabel="Active"
//         statusColor="#22C55E"
//         bannerImage="/eternal-legends.svg"
//         gameIcon="/publisherImage.svg"
//         title="Eternal Legends"
//         developer="Blockchain Gaming Inc"
//         description="A revolutionary MMORPG that combines traditional fantasy with blockchain technology."
//         xLink="https://app.onlydust.com/discover"
//         tgLink="https://app.onlydust.com/discover"
//         webLink="https://app.onlydust.com/discover"
//         tags={['MMORPG', 'BLOCKCHAIN', 'FANTASY']}
//         hasWebsiteLink={true}
//         stats={{
//           players: '800,000+',
//           community: '1.5M',
//         }}
//       />
//     </div>
//   );
// };

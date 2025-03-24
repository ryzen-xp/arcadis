import Image from 'next/image'
import AwardCategoryCard from './AwardCategoryCard'

const HallOfFameGrid = () => {
	const categories = [
		{
			title: 'Game of the Year',
			description: 'The most outstanding game that pushed boundaries and captivated players',
			color: 'bg-yellow-500',
			games: [
				{ game_title: 'Cyber Legends', genre: 'Epic Quest', year: 2024 },
				{ game_title: 'StarkWorld', genre: 'Virtual Reality', year: 2023 },
			],
			icon: <Image src={'/cup.svg'} alt="cup-image" width={64} height={64} />,
		},
		{
			title: 'Innovation Excellence',
			description: 'Games that introduced groundbreaking mechanics or technology',
			color: 'bg-blue-500',
			games: [
				{ game_title: 'NFT Warriors', genre: 'Battle Arena', year: 2024 },
				{ game_title: 'Blockchain Heroes', genre: 'Crypto Quest', year: 2023 },
			],
			icon: <Image src={'/star.svg'} alt="star-image" width={64} height={64} />,
		},
		{
			title: 'Community Choice',
			description: 'Voted by the Arcadis community as their favorite game',
			color: 'bg-green-500',
			games: [
				{ game_title: 'Metaverse Explorers', genre: 'Open World', year: 2024 },
				{ game_title: 'Crypto Racers', genre: 'Speed Champions', year: 2023 },
			],
			icon: <Image src={'/thumbs-up.svg'} alt="star-image" width={64} height={64} />,
		},
		{
			title: 'Best Visual Design',
			description: 'Excellence in artistic direction and visual presentation',
			color: 'bg-purple-500',
			games: [
				{ game_title: 'Digital Dreams', genre: 'Art World', year: 2024 },
				{ game_title: 'Pixel Masters', genre: 'Visual Journey', year: 2023 },
			],
			icon: <Image src={'/badge.svg'} alt="star-image" width={64} height={64} />,
		},
		{
			title: 'Technical Achievement',
			description: 'Outstanding technical implementation and performance',
			color: 'bg-red-500',
			games: [
				{ game_title: 'Tech Titans', genre: 'Code Warriors', year: 2024 },
				{
					game_title: 'Blockchain Builders',
					genre: 'Smart Contract Wars',
					year: 2023,
				},
			],
			icon: <Image src={'/medal.svg'} alt="star-image" width={64} height={64} />,
		},
		{
			title: 'Rising Star',
			description: 'Most promising new game or developer on the platform',
			color: 'bg-teal-500',
			games: [
				{ game_title: 'New Horizons', genre: 'Future Quest ', year: 2024 },
				{ game_title: 'Emerging Worlds', genre: 'Genesis', year: 2023 },
			],
			icon: <Image src={'/crown.svg'} alt="star-image" width={64} height={64} />,
		},
	]
	return (
		<>
			<div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 gap-6 px-6">
				{categories.map((category, index) => (
					<AwardCategoryCard key={Math.random() * 1000} {...category} />
				))}
			</div>
		</>
	)
}

export default HallOfFameGrid

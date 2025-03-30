import { Dot } from 'lucide-react'

interface WinningGame {
	game_title: string
	genre: string
	year: string | number
}
interface AwardCategoryCardProps {
	title: string
	description: string
	icon: React.ReactNode
	games: WinningGame[]
}
const AwardCategoryCard = ({ title, description, icon, games }: AwardCategoryCardProps) => {
	return (
		<>
			<div className=" w-full p-[24px] rounded-xl bg-[#111827] shadow-[#0000001A]">
				<div className="w-full">{icon}</div>

				<div className="w-full mt-4">
					<p className=" text-base text-white text-left lg:text-lg font-bold capitalize">{title}</p>
					<p className=" text-sm mt-4 text-[#9CA3AF] text-left lg:text-base font-normal leading-6 ">
						{description}
					</p>
				</div>

				<div className="w-full mt-4">
					{games.map(({ game_title, genre, year }) => (
						<>
							<div
								className="w-full p-3 rounded-[8px] bg-[#1F2937] mb-4"
								key={Math.floor(Math.random()) * 1000}
							>
								<p className=" text-sm lg:text-base font-semibold capitalize text-white">
									{game_title}
								</p>
								<p className="text-xs flex items-center justify-start text-[#9CA3AF] font-normal mt-1 capitalize">
									{genre} <Dot color="#9CA3AF" size={8} />
									{year}
								</p>
							</div>
						</>
					))}
				</div>
			</div>
		</>
	)
}

export default AwardCategoryCard

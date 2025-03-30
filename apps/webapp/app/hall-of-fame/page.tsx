import HallOfFameGrid from '@/components/hall-of-fame/HallOfFameGrid'

const HallOfFame = () => {
	return (
		<>
			<div className="w-full min-h-screen h-full bg-[#050e29] pb-20 lg:px-[116px] bg-gradient-to-tr from-[#164E63] via-[#060A32] to-[#040726] ">
				<div className="w-full pt-[63.75px]">
					<h1 className=" font-bold text-center text-[36px] lg:text-[56.13px] bg-clip-text text-transparent bg-gradient-to-r  from-[#2563EB] to-[#9333EA] ">
						Hall of Fame
					</h1>

					<p className="mt-[16.25px] font-normal text-center text-[16px] lg:text-lg text-[#9CA3AF]">
						Celebrating excellence in Arcadis gaming
					</p>
				</div>

				<div className="w-full mt-[64px]">
					<HallOfFameGrid />
				</div>
			</div>
		</>
	)
}

export default HallOfFame

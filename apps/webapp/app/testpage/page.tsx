import GameCard from "@/components/ui/GameCard";



const Page = () => {
    return (
        <div className="w-full min-h-screen flex items-center justify-center p-[300px] bg-black " >
        <GameCard 
        rating = {9}
        status = "Active" 
        bannerImage = "/eternal-legends.svg"
        gameIcon = "/publisherImage.svg"
        title = "Eternal Legends"
        developer = "Blockchain Gaming Inc"
        description = "A revolutionary MMORPG that combines traditional fantasy with blockchain technology."
        players = {800000}
        comMembers = {1.5}
        xLink = "https://x.com/home"
        tgLink = "https://x.com/home"
        webLink = "https://x.com/home"
        tags={["MMORPG", "BLOCKCHAIN", "FANTASY"]}
        />
        </div>
    )
}


export default Page;
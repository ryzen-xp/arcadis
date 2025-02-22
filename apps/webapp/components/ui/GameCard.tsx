import { Globe } from "lucide-react";
import Image from "next/image";
import { FaTelegramPlane } from "react-icons/fa";
import { FaXTwitter } from "react-icons/fa6";

const Badge = ({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) => (
  <span className={`px-3 py-1 rounded-[6px] text-xs font-semibold ${className}`}>
    {children}
  </span>
);

const Card = ({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) => (
  <div
    className={`bg-[#BFDBFE1A] border-[1px] border-[#F3F4F626] p-5 rounded-xl min-w-[300px] md:w-[403.11px] h-[603.87px]  text-white ${className}`}
  >
    {children}
  </div>
);

const CardContent = ({
  children,
  className,
}: {
  children: React.ReactNode;
  className?: string;
}) => <div className={`mt-4 ${className}`}>{children}</div>;

interface GameCardProps {
  rating: number;
  status: string;
  bannerImage: string;
  gameIcon: string;
  title: string;
  developer: string;
  description: string;
  players: number;
  comMembers: number;
  xLink: string;
  tgLink: string;
  webLink: string;
  tags: string[];
}

const GameCard: React.FC<GameCardProps> = ({
  rating,
  status,
  bannerImage,
  gameIcon,
  title,
  developer,
  description,
  players,
  comMembers,
  xLink,
  tgLink,
  webLink,
  tags,
}) => {
  return (
    <Card  >
      <div className="relative font-sans flex items-center justify-center mb-5 h-[33%] ">
        <img
          src={bannerImage}
          alt="Game Banner"
          className="rounded-lg w-full h-full object-cover"
        />
        <Badge className="absolute top-2 left-2 bg-[#22C55E] text-white px-3 py-1 rounded-full text-sm">
          {status}
        </Badge>
      </div>

      <CardContent>
        <div className="flex items-start gap-4">
          <Image
            src={gameIcon}
            alt="Game Avatar"
            height={100}
            width={100}
            className="w-16 h-16 rounded-lg object-cover"
          />
          <div className="flex-1 flex flex-col gap-4 mb-10 ">
            <h3 className="text-[19.06px] text-white font-bold">{title} </h3>
            <p className="text-[#9CA3AF] font-normal text-[13.63px] text-sm">{developer} </p>
          </div>



          <div className="bg-[#10B981]  shadow-[0px_1px_3px_0px_#0000001A] text-white w-8 h-8 flex items-center justify-center rounded-md text-lg font-bold">
            {rating}
          </div>
        </div>

        <p className=" text-[13.67px] text-[#D1D5DB] font-normal mt-3 mb-6">{description}</p>

        <div className="flex flex-col items-start gap-2 w-full justify-between ">
          <div className="w-full flex justify-between text-gray-400 text-sm">
            <span className="text-[#9CA3AF] text-[13.56px] font-normal " >Players</span>
            <span className="text-white text-[15.5px] font-medium">{players} </span>
          </div>
          <div className="w-full flex justify-between text-gray-400 text-sm mt-1">
            <span className="text-[#9CA3AF] text-[13.56px] font-normal ">Community</span>
            <span className="text-white text-[15.5px] font-medium">{comMembers}M+</span>
          </div>
        </div>

        <div className="flex gap-2 mt-6">
          {tags.map((tag, index) => (
            <Badge key={index} className="bg-[#064E3B] text-[#34D399] ">
              {" "}
              {tag}{" "}
            </Badge>
          ))}
        </div>

        <div className="flex items-center gap-4 mt-9 text-[#0A0A0A]   text-xl">
          <a href={xLink} className="w-9 h-9 bg-white rounded-[6px] flex justify-center items-center " >
            {" "}
            <FaXTwitter className="cursor-pointer " />{" "}
          </a>
          <a href={tgLink} className="w-9 h-9 bg-white rounded-[6px] flex justify-center items-center " >
            <FaTelegramPlane className="cursor-pointer " />
          </a>
          <a href={webLink} className="ml-auto w-9 h-9 bg-white rounded-[6px] flex justify-center items-center " >
            <Globe className="cursor-pointer " />
          </a>


        </div>
      </CardContent>
    </Card>
  );
};

export default GameCard;

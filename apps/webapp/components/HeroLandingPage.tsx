// pages/index.tsx
import { NextPage } from "next";
import Head from "next/head";
import {
  Search,
  ChevronDown,
  Gamepad2,
  Users,
  Lightbulb,
  Globe,
  LayoutGrid,
  Wallet,
  Gift,
  Network,
  MoveHorizontal,
  BarChart3,
  MonitorPlay,
} from "lucide-react";

const HomeHeroPage: NextPage = () => {
  return (
    <div className="min-h-screen bg-[#080B27] text-white">
      <Head>
        <title>Stellar Gaming Platform | Discover. Play. Earn.</title>
        <meta
          name="description"
          content="Shape the future of blockchain gaming"
        />
        <link rel="icon" href="/favicon.ico" />
      </Head>

      <main className="container mx-auto px-4 py-12 max-w-7xl">
        {/* Hero Section */}
        <section className="flex flex-col items-center justify-center text-center mt-8 mb-20 relative">
          {/* Decorative dots */}
          <div className="absolute top-0 left-1/4 w-2 h-2 rounded-full bg-[#060A32] opacity-60"></div>
          <div className="absolute bottom-1/3 right-1/4 w-2 h-2 rounded-full bg-blue-300 opacity-60"></div>

          <h1 className="text-5xl md:text-6xl font-bold mb-6">
            Discover. Play. Earn.
          </h1>
          <h2 className="text-4xl md:text-5xl font-bold mb-12">
            <span className="bg-gradient-to-r from-blue-500 to-cyan-400 bg-clip-text text-transparent">
              Shape the Future of Blockchain
            </span>
            <br />
            <span className="bg-gradient-to-r from-cyan-400 to-blue-300 bg-clip-text text-transparent">
              Gaming
            </span>
          </h2>

          <div className="flex flex-col sm:flex-row gap-4 z-10">
            <button className="bg-gradient-to-r from-[#3dadd6] to-[#164E63]   text-white py-3 px-8 rounded-lg transition-all duration-300">
              Explore Games Now
            </button>
            <button className="bg-transparent border border-gray-600 hover:border-gray-400 text-white py-3 px-8 rounded-lg transition-all duration-300">
              Register Your Game
            </button>
          </div>
        </section>

        {/* Features Section */}
        <section className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-20">
          <div className="bg-[#3c07646A]  bg-[linear-gradient(90deg,rgba(255,255,255,0)_10%,rgba(255,255,255,0.08)_50%,rgba(255,255,255,0)_90%)]
							shadow-[0px_2px_5px_rgba(0,0,0,0.1)] p-6 rounded-xl">
            <div className="bg-teal-500 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
              <Gamepad2 className="text-white" size={24} />
            </div>
            <h3 className="text-xl font-bold mb-2">Curated Games</h3>
            <p className="text-gray-300">
              Discover and explore handpicked Stellar games with in-depth
              reviews, ratings, and personalized recommendations tailored for
              you.
            </p>
          </div>

          <div className="bg-[#3c07646A]  bg-[linear-gradient(90deg,rgba(255,255,255,0)_10%,rgba(255,255,255,0.08)_50%,rgba(255,255,255,0)_90%)]
							shadow-[0px_2px_5px_rgba(0,0,0,0.1)] p-6 rounded-xl">
            <div className="bg-purple-500 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
              <Users className="text-white" size={24} />
            </div>
            <h3 className="text-xl font-bold mb-2">Community Rewards</h3>
            <p className="text-gray-300">
              Earn tokens, NFTs, and reputation for playing, reviewing, voting,
              and contributing.
            </p>
          </div>

          <div className="bg-[#3c07646A]  bg-[linear-gradient(90deg,rgba(255,255,255,0)_10%,rgba(255,255,255,0.08)_50%,rgba(255,255,255,0)_90%)]
							shadow-[0px_2px_5px_rgba(0,0,0,0.1)] p-6 rounded-xl">
            <div className="bg-pink-500 w-12 h-12 rounded-lg flex items-center justify-center mb-4">
              <Lightbulb className="text-white" size={24} />
            </div>
            <h3 className="text-xl font-bold mb-2">Support Innovation</h3>
            <p className="text-gray-300">
              Help developers crowdfund and grow their games with your
              engagement.
            </p>
          </div>
        </section>

        <section className="mb-20">
          <div className="bg-white rounded-full p-2 flex flex-wrap items-center gap-2 md:gap-0">
            {/* Search Input */}
            <input
              type="text"
              placeholder="Search Games"
              className="bg-transparent border-none outline-none pl-4 flex-grow text-black w-full md:w-auto"
            />

            {/* Buttons and Filters */}
            <div className="flex flex-wrap items-center gap-2 md:gap-0">
              {/* Category Dropdown */}
              <div className="relative mx-2">
                <button className="flex items-center text-black px-3 py-2">
                  <span>Choose category</span>
                  <ChevronDown size={16} className="ml-2" />
                </button>
              </div>

              {/* Divider */}
              <div className="h-8 w-px bg-gray-700 mx-1 hidden md:block"></div>

              {/* State Dropdown */}
              <div className="relative mx-2">
                <button className="flex items-center text-black px-3 py-2">
                  <span>Choose state</span>
                  <ChevronDown size={16} className="ml-2" />
                </button>
              </div>

              {/* Divider */}
              <div className="h-8 w-px bg-gray-700 mx-1 hidden md:block"></div>

              {/* Framework Dropdown */}
              <div className="relative mx-2">
                <button className="flex items-center text-black px-3 py-2">
                  <span>Framework</span>
                  <ChevronDown size={16} className="ml-2" />
                </button>
              </div>

              {/* Search Button */}
              <button className="bg-blue-500 hover:bg-blue-600 p-3 rounded-full text-white ml-2">
                <Search size={20} />
              </button>
            </div>
          </div>
        </section>

        {/* Categories Section */}
        <section className="grid grid-cols-2 sm:grid-cols-4 md:grid-cols-8 gap-6 text-center">
          <div className="flex flex-col items-center space-y-2">
            <div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
              <Gift className="text-white" size={20} />
            </div>
            <span className="text-sm">Play-to-Earn</span>
          </div>

          <div className="flex flex-col items-center space-y-2">
            <div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
              <Globe className="text-white" size={20} />
            </div>
            <span className="text-sm">Gaming Metaverses</span>
          </div>

          <div className="flex flex-col items-center space-y-2">
            <div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
              <LayoutGrid className="text-white" size={20} />
            </div>
            <span className="text-sm">DeFi Gaming</span>
          </div>

          <div className="flex flex-col items-center space-y-2">
            <div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
              <Wallet className="text-white" size={20} />
            </div>
            <span className="text-sm">Trading Card Games</span>
          </div>

          <div className="flex flex-col items-center space-y-2">
            <div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
              <Network className="text-white" size={20} />
            </div>
            <span className="text-sm">Blockchain RPGs</span>
          </div>

          <div className="flex flex-col items-center space-y-2">
            <div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
              <MoveHorizontal className="text-white" size={20} />
            </div>
            <span className="text-sm">Move-to-Earn</span>
          </div>

          <div className="flex flex-col items-center space-y-2">
            <div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
              <BarChart3 className="text-white" size={20} />
            </div>
            <span className="text-sm">Strategy Games</span>
          </div>

          <div className="flex flex-col items-center space-y-2">
            <div className="bg-blue-900 bg-opacity-50 w-12 h-12 rounded-full flex items-center justify-center">
              <MonitorPlay className="text-white" size={20} />
            </div>
            <span className="text-sm">Simulators</span>
          </div>
        </section>
      </main>
    </div>
  );
};

export default HomeHeroPage;

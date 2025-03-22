import React from 'react'
import GameCatalog from '@/components/games/GameCatalog'

const page = () => {
  return (
    <div className='bg-gradient-to-bl from-[#040726] via-[#060A32] to-[#164E63] h-full px-4 sm:px-12 md:px-6 lg:px-24 pt-5'>
      <h2 className='font-bold text-4xl text-white leading-10 ml-3 md:ml-0' >Games</h2>
      <GameCatalog/>
    </div>
  )
}

export default page

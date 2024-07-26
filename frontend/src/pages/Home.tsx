import React from 'react'
import LoginButton from '../components/LoginButton'

export const Home: React.FC = () => {
  return (
    <div className='text-white bg-home-bg h-dvh'>
        <header className='py-6'>
            <div className='container mx-auto flex justify-between items-center px-8 md:px-14'>
                <div className='text-lg font-bold'>IDS</div>
                <div className='space-x-12 hidden md:flex items-center z-30'>
                    <a href='#home' className='hover:text-selected-text transition-all duration-300'>home</a>
                    <LoginButton /> 
                </div>
                <div className='md:hidden'>
                    <i className="fa-solid fa-bars"></i>
                </div>
            </div>
        </header>

        <div id='home' className='container mt-16 flex justify-between items-center mx-auto px-8 md:px-14 lg:px-24 w-full'
        >
            <div className='flex flex-wrap md:flex-nowrap'>
                <div className='md:my-36 lg:ml-20 justify-center md:justify-start max-w-xl flex flex-wrap z-10'>
                    <h1 className='font-bold text-5xl text-center md:text-left md:text-6xl lg:text-7xl'>
                        TailWindCSSで <br />
                        美しいサイトを <br />
                        作ります
                    </h1>
                    <button className='px-6 py-4 mt-10 bg-theme font-bold rounded-lg hover:bg-purple-600 transition-all duration-300'>
                        <i className="fa-solid fa-rocket mr-2"></i>
                        <span>もっと見る</span>
                    </button>
                </div>
                <img src="/human.jpg" alt="image" className='md:absolute lg:top-2 lg:right-52 md:w-3/5 w-10/12 mt-12 md:mt-0 right-6 mx-auto z-0 lg:w-3/6'/>
            </div>
        </div>
    </div>
  )
}

import React from 'react'
// import LoginButton from '../components/auth/LoginButton'
// import { SignupButton } from '../components/auth/SignupButton'
import Auth0NavButtons from '../components/auth/Auth0NavButtons'

export const Home: React.FC = () => {
  return (
    <div className='text-white bg-home-bg h-screen w-screen overscroll-none'>
        <header className='py-6'>
            <div className='container mx-auto flex justify-between items-center px-8 md:px-14'>
                <div className='text-3xl font-bold'>IDS</div>
                <div className='space-x-12 hidden md:flex items-center z-30'>
                    <Auth0NavButtons /> 
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
                        Great things together
                    </h1>
                </div>
                <img src="/DALL·E 2024-09-26 13.25.13 - A peaceful scene of a tree surrounded by a calm lake. The tree stands on a small island or by the water's edge, its branches gently hanging over the s.webp" alt="image" className='md:absolute lg:top-2 lg:right-52 md:w-3/5 w-10/12 mt-12 md:mt-0 right-6 mx-auto z-0 lg:w-3/6'/>
            </div>
        </div>

        <div 
            id='home' 
            className='container mt-16 flex justify-between items-center mx-auto px-8 md:px-14 lg:px-24 w-full'
        >
        </div>
    </div>
  )
}

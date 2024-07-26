import React from 'react'
import { Information } from '../pages/Root';
import { format } from 'date-fns';

type InfoListProps = {
    infos: Information[]; 
}

const InfoList: React.FC<InfoListProps> = ({ infos }) => {
    
    return(
        <>
            {/* <!-- Timeline --> */}
            <div>
            {
                infos.map(info => {
                    return (
                        <div className="flex gap-x-3 pt-10">
                            <div className="w-16 text-end">
                                <span className="text-xl text-gray-500">{format(info.date_time, "hh:mma")}</span>
                                </div>

                                <div className="relative last:after:hidden after:absolute after:top-7 after:bottom-0 after:start-3.5 after:w-px after:-translate-x-[0.5px] after:bg-gray-200">
                                <div className="relative z-10 size-7 flex justify-center items-center">
                                    <div className="size-2 rounded-full bg-gray-400"></div>
                                </div>
                            </div>

                            <div className="grow pt-0.5 pb-8">
                                <h3 className="flex gap-x-1.5 font-semibold text-3xl text-gray-500">
                                    <svg className="flex-shrink-0 size-7 mt-1" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                                    <path d="M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z"></path>
                                    <polyline points="14 2 14 8 20 8"></polyline>
                                    <line x1="16" x2="8" y1="13" y2="13"></line>
                                    <line x1="16" x2="8" y1="17" y2="17"></line>
                                    <line x1="10" x2="8" y1="9" y2="9"></line>
                                    </svg>
                                    {info.summary}
                                </h3>
                                <p className="mt-1 text-lg text-gray-600">
                                    {info.memo}
                                </p>
                                <div className='flex items-center mt-2'>
                                <span className=' mr-2 text-sm text-gray-500'>園児</span>
                                <button type="button" className=" -ms-1  inline-flex items-center gap-x-2 text-sm rounded-lg border border-transparent bg-sky-50 text-gray-500 hover:bg-sky-100 disabled:opacity-50 disabled:pointer-events-none">
                                    <img className="flex-shrink-0 size-4 rounded-full" src="https://images.unsplash.com/photo-1659482633369-9fe69af50bfb?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8auto=format&fit=facearea&facepad=3&w=320&h=320&q=80" alt="Image Description"/>
                                    {info.name}
                                </button>
                                </div>
                                <div className='flex items-center mt-0.5'>
                                <span className=' mr-2 text-sm text-gray-500'>担当</span>
                                <button type="button" className=" -ms-1  inline-flex items-center gap-x-2 text-sm rounded-lg border border-transparent bg-pink-50 text-gray-500 hover:bg-pink-100 disabled:opacity-50 disabled:pointer-events-none">
                                    <img className="flex-shrink-0 size-4 rounded-full" src="https://images.unsplash.com/photo-1659482633369-9fe69af50bfb?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8auto=format&fit=facearea&facepad=3&w=320&h=320&q=80" alt="Image Description"/>
                                    {info.manager}
                                </button>
                                </div>
                            </div>
                        </div>
                    )
                })
            }
            

            
            </div>
            {/* <!-- End Timeline --> */}
        </>
    )
};
export default InfoList;

// {/* <!-- Item --> */}
// <div className="flex gap-x-3">
// {/* <!-- Left Content --> */}
// <div className="w-16 text-end">
// <span className="text-xs text-gray-500">12:05PM</span>
// </div>
// {/* <!-- End Left Content --> */}

// {/* <!-- Icon --> */}
// <div className="relative last:after:hidden after:absolute after:top-7 after:bottom-0 after:start-3.5 after:w-px after:-translate-x-[0.5px] after:bg-gray-200">
// <div className="relative z-10 size-7 flex justify-center items-center">
//     <div className="size-2 rounded-full bg-gray-400"></div>
// </div>
// </div>
// {/* <!-- End Icon --> */}

// {/* <!-- Right Content --> */}
// <div className="grow pt-0.5 pb-8">
// <h3 className="flex gap-x-1.5 font-semibold text-gray-800">
//     Release v5.2.0 quick bug fix 🐞
// </h3>
// <button type="button" className="mt-1 -ms-1 p-1 inline-flex items-center gap-x-2 text-xs rounded-lg border border-transparent text-gray-500 hover:bg-gray-100 disabled:opacity-50 disabled:pointer-events-none">
//     <span className="flex flex-shrink-0 justify-center items-center size-4 bg-white border border-gray-200 text-[10px] font-semibold uppercase text-gray-600 rounded-full">
//     A
//     </span>
//     Alex Gregarov
// </button>
// </div>
// {/* <!-- End Right Content --> */}
// </div>
// {/* <!-- End Item --> */}

// {/* <!-- Item --> */}
// <div className="flex gap-x-3">
// {/* <!-- Left Content --> */}
// <div className="w-16 text-end">
// <span className="text-xs text-gray-500">12:05PM</span>
// </div>
// {/* <!-- End Left Content --> */}

// {/* <!-- Icon --> */}
// <div className="relative last:after:hidden after:absolute after:top-7 after:bottom-0 after:start-3.5 after:w-px after:-translate-x-[0.5px] after:bg-gray-200">
// <div className="relative z-10 size-7 flex justify-center items-center">
//     <div className="size-2 rounded-full bg-gray-400"></div>
// </div>
// </div>
// {/* <!-- End Icon --> */}

// {/* <!-- Right Content --> */}
// <div className="grow pt-0.5 pb-8">
// <h3 className="flex gap-x-1.5 font-semibold text-gray-800">
//     Marked "Install Charts" completed
// </h3>
// <p className="mt-1 text-sm text-gray-600">
//     Finally! You can check it out here.
// </p>
// <button type="button" className="mt-1 -ms-1 p-1 inline-flex items-center gap-x-2 text-xs rounded-lg border border-transparent text-gray-500 hover:bg-gray-100 disabled:opacity-50 disabled:pointer-events-none">
//     <img className="flex-shrink-0 size-4 rounded-full" src="https://images.unsplash.com/photo-1659482633369-9fe69af50bfb?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=facearea&facepad=3&w=320&h=320&q=80" alt="Image Description"/>
//     James Collins
// </button>
// </div>
// {/* <!-- End Right Content --> */}
// </div>
// {/* <!-- End Item --> */}

// {/* <!-- Item --> */}
// <div className="flex gap-x-3">
// {/* <!-- Left Content --> */}
// <div className="w-16 text-end">
// <span className="text-xs text-gray-500">12:05PM</span>
// </div>
// {/* <!-- End Left Content --> */}

// {/* <!-- Icon --> */}
// <div className="relative last:after:hidden after:absolute after:top-7 after:bottom-0 after:start-3.5 after:w-px after:-translate-x-[0.5px] after:bg-gray-200">
// <div className="relative z-10 size-7 flex justify-center items-center">
//     <div className="size-2 rounded-full bg-gray-400"></div>
// </div>
// </div>
// {/* <!-- End Icon --> */}

// {/* <!-- Right Content --> */}
// <div className="grow pt-0.5 pb-8">
// <h3 className="flex gap-x-1.5 font-semibold text-gray-800">
//     Take a break ⛳️
// </h3>
// <p className="mt-1 text-sm text-gray-600">
//     Just chill for now... 😉
// </p>
// </div>
// {/* <!-- End Right Content --> */}
// </div>
// {/* <!-- End Item --> */}
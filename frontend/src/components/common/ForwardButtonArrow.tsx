import React from 'react';
import { IoArrowForward } from "react-icons/io5";
import { useNavigate } from 'react-router-dom';


const ForwardButtonArrow: React.FC = () => {
    // 戻るボタン React-Router 必須
    const navigate = useNavigate();

    const handleClick = () => navigate(+1);

    return(
        <>
            <div className='flex border-none rounded-full items-center border-inherit
            justify-center cursor-pointer text-xl size-10 p-0 duration-200 hover:bg-gray-300 bg-inherit'
            onClick={handleClick}
            >
                <IoArrowForward className='text-lg block select-none border-0'/>
            </div>
        </>
    )
};
export default ForwardButtonArrow;
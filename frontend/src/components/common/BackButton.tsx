import React from 'react';
import { MdArrowBackIos } from "react-icons/md";
import { useNavigate } from 'react-router-dom';


const BackButton: React.FC = () => {
    // 戻るボタン React-Router 必須
    const navigate = useNavigate();

    const handleClick = () => navigate(-1);

    return(
        <>
            <div className='border-2 rounded-md w-14 text-center 
            align-middle mt-1 ml-1 bg-gray-50 hover:bg-gray-100'
            onClick={handleClick}
            >
                <MdArrowBackIos className='text-md inline pl-1 mb-0.5 select-none'/>
                <span className='text-sm pr-1 m-0 select-none text-gray-700'>戻る</span>
            </div>
        </>
    )
};
export default BackButton;
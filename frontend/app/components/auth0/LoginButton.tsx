import { useAuth0 } from '@auth0/auth0-react';
import React from 'react'

export const LoginButton: React.FC = () => {
    const { loginWithRedirect } = useAuth0();

    return (
        <button className='px-1 py-2 bg-black font-bold rounded-lg hover:bg-gray-500 
        transition-all duration-300 ml-3' onClick={() => loginWithRedirect()}
        >
            ログイン
        </button>
    );
};

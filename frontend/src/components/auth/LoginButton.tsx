import { useAuth0 } from '@auth0/auth0-react';
import React from 'react'

const LoginButton: React.FC = () => {
    const { loginWithRedirect } = useAuth0();

    const handleLogin = async () => {
        await loginWithRedirect({
            appState: {
            returnTo: "/user/users",
            },
        });
    };
    return (
        <button className='px-6 py-2 bg-theme font-bold rounded-lg hover:bg-purple-600 
        transition-all duration-300 ml-3' onClick={handleLogin}
        >
            Login
        </button>
    );
};

export default LoginButton
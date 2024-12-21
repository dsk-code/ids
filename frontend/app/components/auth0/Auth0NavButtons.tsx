import { useAuth0 } from '@auth0/auth0-react';
import React from 'react'
import { SignupButton } from './SignupButton';
import { LogoutButton } from './ LogoutButton';
import { LoginButton } from './LoginButton';

const Auth0NavButtons: React.FC = () => {
    const { isAuthenticated } = useAuth0();

    return (
        <div className="mobile-nav-bar__buttons">
        {!isAuthenticated && (
            <>
            <SignupButton />
            <LoginButton />
            </>
        )}
        {isAuthenticated && (
            <>
            <LogoutButton />
            </>
        )}
        </div>
    )
}

export default Auth0NavButtons
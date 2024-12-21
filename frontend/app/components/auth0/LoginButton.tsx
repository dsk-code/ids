import { useAuth0 } from '@auth0/auth0-react';
import React from 'react'

export const LoginButton: React.FC = () => {
    const { loginWithRedirect } = useAuth0();

    // const handleLogin = async () => {
    //     await loginWithRedirect({
    //         // appState: {
    //         // returnTo: "/user",
    //         // },
    //         authorizationParams: {
    //             redirect_uri: window.location.origin,
    //         },
    //     });
    //     await loginWithRedirect();
    // };

    return (
        <button className='px-6 py-2 bg-theme font-bold rounded-lg hover:bg-purple-600 
        transition-all duration-300 ml-3' onClick={() => loginWithRedirect()}
        >
            Login
        </button>
        // <button className='px-6 py-2 bg-theme font-bold rounded-lg hover:bg-purple-600 
        // transition-all duration-300 ml-3' onClick={handleLogin}
        // >
        //     Login
        // </button>
    );
};

// import React from "react";
// import { useAuth0 } from "@auth0/auth0-react";

// const LoginButton: React.FC = () => {
//   const { loginWithRedirect } = useAuth0();

//   return (
//     <button className='px-6 py-2 bg-theme font-bold rounded-lg hover:bg-purple-600 transition-all duration-300 ml-3' onClick={() => loginWithRedirect()}>
//         Log In
//     </button>
//   );
// };

// export default LoginButton;
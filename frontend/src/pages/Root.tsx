// import React, { useEffect, useState } from 'react'
import { useAuth0 } from "@auth0/auth0-react";
// import useEnv from '../hooks/useEnv';

const Root: React.FC = () => {
    // https://auth0.com/docs/quickstart/spa/react/02-calling-an-api
    // const { user, isAuthenticated, getAccessTokenSilently } = useAuth0();
    const { user, isAuthenticated } = useAuth0();
    // const [userMetadata, setUserMetadata] = useState(null);
    // const { domain, audience } = useEnv();
    
    // // userが未取得の場合、ロード中の状態を表示する
    // if (user === undefined) {
    //     return <div>Loading...</div>;
    // }

    // useEffect(() => {
    //     const getUserMetadata = async () => {   
    //         // if (!user) return; // userがない場合は処理を中断     
    //         try {
    //             const accessToken = await getAccessTokenSilently({
    //                 authorizationParams: {
    //                     audience: audience,
    //                     scope: "read:current_user",
    //                 },
    //             });
        
    //             const userDetailsByIdUrl = `https://${domain}/api/v2/users/${user.sub}`;
        
    //             const metadataResponse = await fetch(userDetailsByIdUrl, {
    //                 headers: {
    //                     Authorization: `Bearer ${accessToken}`,
    //                 },
    //             });
        
    //             const { user_metadata } = await metadataResponse.json();
        
    //             setUserMetadata(user_metadata);
    //         } catch (e) {
    //             if (e instanceof Error) {
    //                 console.log(e.message);
    //             } else {
    //                 console.log("An unknown error occurred");
    //             }
    //         }
    //     };
        
    //     getUserMetadata();
    // }, [getAccessTokenSilently, user.sub]);

    // if (user == undefined) {
    //     return null;
    // }
    if (!user) {
        return (
            <h3>No Data</h3>
        );
    }

    return (
        isAuthenticated && (
            <div>
                <p>1</p>
                <img src={user.picture} alt={user.name} />
                <h2>{user.name}</h2>
                <p>{user.email}</p>
                <h3>User Metadata</h3>
                {/* {userMetadata ? (
                <pre>{JSON.stringify(userMetadata, null, 2)}</pre>
                ) : (
                "No user metadata defined"
                )} */}
            </div>
        )
    );
};

export default Root;
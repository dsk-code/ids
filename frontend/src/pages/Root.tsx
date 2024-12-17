import React, { useEffect } from 'react'
import { useAuth0 } from "@auth0/auth0-react";
import useEnv from '../hooks/useEnv';
import { AuthUser, ResponseAuthUser } from '../types/userTypes';
import { postData } from '../api/api';
import { useRecoilState } from 'recoil';
import { authUserState } from '../recoil/atoms';

const Root: React.FC = () => {
    // https://auth0.com/docs/quickstart/spa/react/02-calling-an-api
    const { user, isAuthenticated, getAccessTokenSilently } = useAuth0();
    const [authUser, setAuthUser] = useRecoilState(authUserState);
    const { audience } = useEnv();
    
    useEffect(() => {
        if (user) {
          console.log("User retrieved"); // ユーザーが取得された時にログを出力
        }
      }, [user]);

    useEffect(() => {
        const sendUserData = async () => {   
            if (!user) return; // userがない場合は処理を中断   

            try {
                // アクセストークンの取得
                console.log("リクエスト開始");
                const accessToken = await getAccessTokenSilently({
                    authorizationParams: {
                        audience: audience,
                        // scope: "read:current_user update:current_user_metadata",
                        // prompt: "consent", // 同意を強制する
                    },
                });
                
                const payload: AuthUser = {
                    userName: user.name || "",
                    userEmail: user.email || "",
                };
        
                // APIにPOSTリクエスト
                console.log("リクエスト開始");
                const response = await postData<ResponseAuthUser>(
                    "http://127.0.0.1:8000/api/v1/me",
                    payload,
                    accessToken
                );
            

                setAuthUser(response);
                
                console.log("Response:", response);
            } catch (e) {
                if (e instanceof Error) {
                    console.log(e.message);
                } else {
                    console.log("An unknown error occurred");
                }
            }
        };
        
        sendUserData();
    }, [getAccessTokenSilently, user, audience]);

    // if (!user) {
    //     return (
    //         <h3>No Data</h3>
    //     );
    // }

    return (
        isAuthenticated && (
            <div>
                <p>1</p>
                <img src={user?.picture} alt={user?.name} />
                <h2>{user?.name}</h2>
                <p>{user?.email}</p>
                <h3>User Metadata</h3>
                <p>{authUser.id}</p>
                <p>{authUser.userName}</p>
            </div>
        )
    );
};

export default Root;
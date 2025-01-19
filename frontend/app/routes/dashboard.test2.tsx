import React, { useEffect, useState } from 'react'
import { useAuth0 } from "@auth0/auth0-react";
import useEnv from '../hooks/useEnv';
import { AuthUser, ResponseAuthUser } from '../types/userTypes';
import { postData } from '../api/api';
import { useRecoilState } from 'recoil';
import { classState, RequestPostClassState } from '../recoil/atoms';

export default function ClassForm() {
    // https://auth0.com/docs/quickstart/spa/react/02-calling-an-api
    const { isAuthenticated, getAccessTokenSilently } = useAuth0();
    const [ form , setForm ] = useRecoilState(RequestPostClassState);

    // useEffect(() => {
    //     const sendUserData = async () => {   
    //         if (!user) return; // userがない場合は処理を中断   

    //         try {
    //             // アクセストークンの取得
    //             console.log("リクエスト開始");
    //             const accessToken = await getAccessTokenSilently({
    //                 authorizationParams: {
    //                     audience: audience,
    //                 },
    //             });
                
    //             const payload: AuthUser = {
    //                 auth0UserName: user.name || "",
    //                 auth0UserEmail: user.email || "",
    //             };
        
    //             // APIにPOSTリクエスト
    //             console.log("リクエスト開始");
    //             const response = await postData<ResponseAuthUser>(
    //                 "http://127.0.0.1:8000/api/v1/me",
    //                 // "https://ids.shuttleapp.rs/api/v1/me",
    //                 payload,
    //                 accessToken
    //             );
            

    //             setAuthUser(response);
                
    //             console.log("Response:", response);
    //         } catch (e) {
    //             if (e instanceof Error) {
    //                 console.log(e.message);
    //             } else {
    //                 console.log("An unknown error occurred");
    //             }
    //         }
    //     };
        
    //     sendUserData();
    // }, [getAccessTokenSilently, user, audience]);

    const handleForm = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { name, value } = e.target
        setForm((prev) => ({
            ...prev,
            [name]: value,
        }));
    }

    const show = () => {
        console.log(`こんにちは、${form.className}(${form.age}歳)さん`);
    }

    return (
        isAuthenticated && (
            <form>
                <div className="max-w-sm">
                    <label className="block text-sm font-medium mb-2 dark:text-white">クラス名</label>
                    <input type="text" id='name' name='className' onChange={handleForm} value={form.className} className="py-3 px-4 block w-full border-gray-200 rounded-lg text-sm focus:border-blue-500 focus:ring-blue-500 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900 dark:border-neutral-700 dark:text-neutral-400 dark:placeholder-neutral-500 dark:focus:ring-neutral-600" />
                </div>
                <div>
                    <label className="block text-sm font-medium mb-2 dark:text-white">年齢</label>
                    <input id='age' name='age' type='number' onChange={handleForm} value={form.age}/>
                </div>
                <div className="bg-white border border-gray-200 rounded-lg dark:bg-neutral-700 dark:border-neutral-700" data-hs-input-number="">
                <div className="w-full flex justify-between items-center gap-x-1">
                    <div className="grow py-2 px-3">
                    <input className="w-full p-0 bg-transparent border-0 text-gray-800 focus:ring-0 [&::-webkit-inner-spin-button]:appearance-none [&::-webkit-outer-spin-button]:appearance-none dark:text-white" type="number" aria-roledescription="Number field" value="1" data-hs-input-number-input=""/>
                    </div>
                    <div className="flex items-center -gap-y-px divide-x divide-gray-200 border-s border-gray-200 dark:divide-neutral-700 dark:border-neutral-700">
                    <button type="button" className="size-10 inline-flex justify-center items-center gap-x-2 text-sm font-medium last:rounded-e-lg bg-white text-gray-800 hover:bg-gray-50 focus:outline-none focus:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900 dark:text-white dark:hover:bg-neutral-800 dark:focus:bg-neutral-800" aria-label="Decrease" data-hs-input-number-decrement="">
                        <svg className="shrink-0 size-3.5" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M5 12h14"></path>
                        </svg>
                    </button>
                    <button type="button" className="size-10 inline-flex justify-center items-center gap-x-2 text-sm font-medium last:rounded-e-lg bg-white text-gray-800 hover:bg-gray-50 focus:outline-none focus:bg-gray-50 disabled:opacity-50 disabled:pointer-events-none dark:bg-neutral-900 dark:text-white dark:hover:bg-neutral-800 dark:focus:bg-neutral-800" aria-label="Increase" data-hs-input-number-increment="">
                        <svg className="shrink-0 size-3.5" xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                        <path d="M5 12h14"></path>
                        <path d="M12 5v14"></path>
                        </svg>
                    </button>
                    </div>
                </div>
                </div>
                <div>
                    <button className='py-3 px-4 inline-flex items-center gap-x-2 text-sm font-medium rounded-lg border border-transparent bg-teal-500 text-white hover:bg-teal-600 focus:outline-none focus:bg-teal-600 disabled:opacity-50 disabled:pointer-events-none' type='button' onClick={show}>
                        送信
                    </button>
                </div>
                <p>こんにちは, {form.className}({form.age}歳)さん</p>
            </form>
        )
    );
};

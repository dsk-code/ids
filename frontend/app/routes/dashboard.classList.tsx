import { useAuth0 } from "@auth0/auth0-react";
import { Card, Center, Container, Space, Text } from "@mantine/core";
import { useHover } from "@mantine/hooks";
import { useEffect } from "react";
import { useRecoilState } from "recoil";
import { getData } from "~/api/api";
import useEnv from "~/hooks/useEnv";
import { classListState } from "~/recoil/atoms";
import { Class } from "~/types/classTypes";

export default function ClassList() {
    const { isAuthenticated, getAccessTokenSilently } = useAuth0();
    const { audience, backendApiUrl } = useEnv();
    const [classList, setClassList] = useRecoilState(classListState);

    useEffect(() => {
        const fetchClassList = async () => {
            try {
                // アクセストークンの取得
                console.log("アクセストークンのリクエスト開始");
                const accessToken = await getAccessTokenSilently({
                    authorizationParams: {
                        audience: audience,
                    },
                }).catch((error) => {
                    console.error('アクセストークンの取得に失敗しました:', error);
                });
                console.log(accessToken);
        
                if (accessToken) {
                    // APIにPOSTリクエスト
                    console.log("クラスリストのリクエスト開始");
                    const response = await getData<Class[]>(
                        `${backendApiUrl}/classes`,
                        accessToken
                    );
                    console.log("Response:", response);
                    setClassList(response);
                }
            } catch (e) {
                if (e instanceof Error) {
                    console.log(e.message);
                } else {
                    console.log("An unknown error occurred");
                }
            }
        };
        fetchClassList();
    }, []);


    return (
        isAuthenticated && (
            classList.length > 0 ? (
                <>
                    <Space h="md" />
                    <Container size="xs">
                        <h1>クラスリスト</h1>
                        <ul>
                            {classList && (
                                classList.map((cls) => (
                                        <Card className="border-2 border-gray-300 transition translate-y-4 hover:bg-gray-50 hover:shadow-lg" padding="lg" m="sm" radius="md" withBorder >
                                            <p>{cls.className}: {cls.age}歳</p>
                                        </Card>
                                ))
                            ) }          
                        </ul>
                    </Container>
                </>
            ) : (
                <>
                    <Space h="md" />
                    <Center>
                      <Text c="red">クラスが登録されていません</Text>
                    </Center>
                </>
            )
      
        )
    );
  }



// import React, { useState } from 'react'
// import DatePicker from "react-datepicker";

// import "react-datepicker/dist/react-datepicker.css";
// import SearchBox from '../components/SeachBox';
// import { useAuth0 } from '@auth0/auth0-react';

// export default function Test1() {
//     const [startDate, setStartDate] = useState(new Date());
//     const { isAuthenticated, user } = useAuth0();

//     if (!user) {
//         return null;
//     }

//     if (isAuthenticated) {
//         return(
//             <>
//                 <div className='flex flex-col'>
//                     <div>
//                         <SearchBox />
//                     </div>
                    
//                     <DatePicker
//                         selected={startDate}
//                         onChange={(date) => date && setStartDate(date)}
//                         showTimeSelect
//                         timeFormat="HH:mm"
//                         timeIntervals={15}
//                         timeCaption="time"
//                         dateFormat="yyyy-MM-dd HH:mm"
//                     />
//                 </div>
//                 <h2>{user.name}</h2>
//                 <h2>{user.email}</h2>
//             </>
//         )
//     }
// };


import { useAuth0 } from '@auth0/auth0-react';
import { Button, Container, Text } from '@mantine/core';
import { useParams } from '@remix-run/react';
import { useEffect, useState } from 'react';
import { useRecoilState } from 'recoil';
import { deleteData, getData } from '~/api/api';
import useEnv from '~/hooks/useEnv';
import { classState } from '~/recoil/atoms';
import { Class } from '~/types/classTypes';

export default function ClassDetails() {
    const { isAuthenticated, getAccessTokenSilently } = useAuth0();
    const params = useParams();
    const [classDetails, setClassDatails] = useRecoilState(classState);
    const [isDeleted, setIsDeleted] = useState(false);
    const { audience, backendApiUrl } = useEnv();

    useEffect(() => {
        const fetchClass = async () => {
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
                    const response = await getData<Class>(
                        `${backendApiUrl}/classes/${params.classId}`,
                        accessToken
                    );
                    console.log("Response:", response);
                    setClassDatails(response);
                }
            } catch (e) {
                if (e instanceof Error) {
                    console.log(e.message);
                } else {
                    console.log("An unknown error occurred");
                }
            }
        };
        if (!isDeleted) {
            fetchClass();
        }
    }, [isDeleted, params.classId]);

    const formatDate = (dateStr: string) => {
        const date = new Date(dateStr);
        return date.toLocaleDateString();
    }

    const handleDelete = async () => {
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
                const response = await deleteData(
                    `${backendApiUrl}/classes/${params.classId}`,
                    accessToken
                );
                if (response.status === 204) {
                    setIsDeleted(true);
                    setClassDatails(undefined);
                }
                console.log("Response:", response);
            }
        } catch (e) {
            if (e instanceof Error) {
                console.log(e.message);
            } else {
                console.log("An unknown error occurred");
            }
        }
    }

    if (!isAuthenticated) {
        return <Text c="red">ログインが必要です。</Text>
    }
    
    if (isDeleted) {
        return (
            <Container size="xs">
                <Text c="red">クラスの詳細データが存在しません</Text>
            </Container>
        )
    }

    return (
        classDetails ? (
            <Container size="xs">
                <p>クラス名: {classDetails.className}</p>
                <p>年齢: {classDetails.age}歳</p>
                <p>作成日: {formatDate(classDetails.createdAt)}</p>
                <p>更新日: {formatDate(classDetails.updatedAt)}</p>
                <Button variant="filled" color="red" size="xs" radius="md" onClick={handleDelete}>削除</Button>
            </Container>          
        ) : (
            <Container size="xs">
                <Text c="red">クラスの詳細データが存在しません</Text>
            </Container>
        )       
    )
}


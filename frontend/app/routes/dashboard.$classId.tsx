import { useAuth0 } from '@auth0/auth0-react';
import { Button, Container, Flex, Group, Notification, Text } from '@mantine/core';
import { useForm } from '@mantine/form';
import { useDisclosure } from '@mantine/hooks';
import { useNavigate, useParams } from '@remix-run/react';
import { useEffect, useState } from 'react';
import { useRecoilState } from 'recoil';
import { deleteData, getData, putData } from '~/api/api';
import { DeleteClassModal } from '~/components/class/DeleteClassModal';
import { EditClassModal } from '~/components/class/EditClassModal';
import { PageLoader } from '~/components/common/PageLoader';
import { useDeleteClass } from '~/hooks/class/useDeleteClass';
import { usePutClass } from '~/hooks/class/usePutClass';
import useEnv from '~/hooks/useEnv';
import { classState } from '~/recoil/atoms';
import { Class, RequestPutClass } from '~/types/classTypes';
import { notifications } from '@mantine/notifications';

export default function ClassDetails() {
    const { isAuthenticated, getAccessTokenSilently } = useAuth0();
    const params = useParams();
    const [classDetails, setClassDatails] = useRecoilState(classState);
    const [isLoading, setIsLoading] = useState(false);
    const [isDeleted, setIsDeleted] = useState(false);
    const { putClass } = usePutClass();
    const { deleteClass } = useDeleteClass();
    const [opened, handlers] = useDisclosure(false);
    const navigate = useNavigate();
    const form = useForm({
        mode: 'uncontrolled',
        initialValues: {
            className: "",
            age: "0",
        }
    });
    
    const { audience, backendApiUrl } = useEnv();

    useEffect(() => {
        const fetchClass = async () => {
            setIsLoading(true);
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
            } finally {
                setIsLoading(false);
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

    // const handleDelete = async () => {
    //     setIsLoading(true);
    //     try {
    //         // アクセストークンの取得
    //         console.log("アクセストークンのリクエスト開始");
    //         const accessToken = await getAccessTokenSilently({
    //             authorizationParams: {
    //                 audience: audience,
    //             },
    //         }).catch((error) => {
    //             console.error('アクセストークンの取得に失敗しました:', error);
    //         });
    //         console.log(accessToken);
    
    //         if (accessToken) {
    //             // APIにPOSTリクエスト
    //             console.log("クラスリストのリクエスト開始");
    //             const response = await deleteData(
    //                 `${backendApiUrl}/classes/${params.classId}`,
    //                 accessToken
    //             );
    //             if (response.status === 204) {
    //                 setIsDeleted(true);
    //                 setClassDatails(undefined);
    //                 navigate("/dashboard/classList");
    //             }
    //             console.log("Response:", response);
    //         }
    //     } catch (e) {
    //         if (e instanceof Error) {
    //             console.log(e.message);
    //         } else {
    //             console.log("An unknown error occurred");
    //         }
    //     } finally {
    //         setIsLoading(false);
    //     }
    // }

    const handleEdit = async (values: typeof form.values) => {
        const age = parseInt(values.age, 10) 
        const payload: RequestPutClass = {
            className: values.className,
            age,
        } 
        if (params.classId) {
            const response = await putClass(params.classId, payload);
            if (response?.data) {
                setClassDatails(response.data);
                handlers.close();
            } else {
                console.log(response?.message);
            }
        }
    }

    const handleDelete = async () => {
        if (params.classId) {
            const response = await deleteClass(params.classId);
            
            if (response?.success) {
                navigate("/dashboard/classList");
                handlers.close();
                return notifications.show({
                    title: "通知", 
                    message: `${classDetails?.className} ${classDetails?.age}は削除されました。`
                });
            } else {
                console.log(response?.message);
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

    if (isLoading) {
        return ( <PageLoader /> );
    }

    return (
        classDetails ? (
            <Container size="xs">
                <Notification color="red" />
                <Flex
                mih={50}
                gap="md"
                justify="center"
                align="center"
                direction="column"
                wrap="wrap"
                >
                    <p>クラス名: {classDetails.className}</p>
                    <p>年齢: {classDetails.age}歳</p>
                    <p>作成日: {formatDate(classDetails.createdAt)}</p>
                    <p>更新日: {formatDate(classDetails.updatedAt)}</p>
                    <Group>
                        {/* {params.classId && ( */}
                            <EditClassModal className={classDetails.className} age={classDetails.age} opened={opened} close={handlers.close} open={handlers.open} handleEdit={handleEdit}/>
                        {/* )} */}
                        <DeleteClassModal className={classDetails.className} age={classDetails.age} handleDelete={handleDelete}/>
                    </Group>
                </Flex>
            </Container>          
        ) : (
            <Container size="xs">
                <Text c="red">クラスの詳細データが存在しません</Text>
            </Container>
        )       
    )
}


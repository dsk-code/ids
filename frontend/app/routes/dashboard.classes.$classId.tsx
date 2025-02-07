import { useAuth0 } from '@auth0/auth0-react';
import { Container, Flex, Group, Text } from '@mantine/core';
import { useForm, UseFormReturnType } from '@mantine/form';
import { useDisclosure } from '@mantine/hooks';
import { useNavigate, useParams } from '@remix-run/react';
import { useEffect, useState } from 'react';
import { useRecoilState } from 'recoil';
import { DeleteClassModal } from '~/components/class/DeleteClassModal';
import { EditClassModal } from '~/components/class/EditClassModal';
import { PageLoader } from '~/components/common/PageLoader';
import useEnv from '~/hooks/useEnv';
import { classState } from '~/recoil/atoms';
import { Class, RequestPutClass } from '~/types/classTypes';
import { notifications } from '@mantine/notifications';
import { useGetRequest } from '~/hooks/request/useGetRequest';
import { DeleteRequestParts, GetRequestParts, PutRequestParts } from '~/types/requestPartsTypes';
import { usePutRequest } from '~/hooks/request/usePutRequest';
import { useDeleteRequest } from '~/hooks/request/userDeleteRequest';

// todo: isDeletedが必要なのかを検討
export default function ClassDetails() {
    const { isAuthenticated } = useAuth0();
    const params = useParams();
    const [classDetails, setClassDatails] = useRecoilState(classState);
    const { getRequest, isLoading, setIsLoading } = useGetRequest();
    const { putRequest } = usePutRequest();
    const { deleteRequest } = useDeleteRequest();
    const [isDeleted, setIsDeleted] = useState(false);
    const [opened, handlers] = useDisclosure(false);
    const navigate = useNavigate();
    const form = useForm({
        mode: 'uncontrolled',
        initialValues: {
            className: "",
            age: "0",
        }
    });
    
    const { backendApiUrl } = useEnv();

    useEffect(() => {
        const fetchClass = async () => {
            setIsLoading(true);
            const parts: GetRequestParts = {
                apiPath: `${backendApiUrl}/classes/${params.classId}`,
            }
            console.log(parts);
            const response = await getRequest<Class>(parts);
            if (response?.success && response?.data) {
                setClassDatails(response.data);
            } else {
                console.error(response?.message);
            }
        };
        if (!isDeleted) {
            fetchClass();
            setIsLoading(false);
        }
    }, [isDeleted, params.classId]);

    const formatDate = (dateStr: string) => {
        const date = new Date(dateStr);
        return date.toLocaleDateString();
    }

    const handleEdit = async (values: typeof form.values) => {
        const age = parseInt(values.age, 10) 
        const payload: RequestPutClass = {
            className: values.className,
            age,
        } 

        if (params.classId) {
            const parts: PutRequestParts = {
                apiPath: `${backendApiUrl}/classes/${params.classId}`,
                payload,
            }
            const response = await putRequest<Class>(parts);
            // const response = await putClass(params.classId, payload);
            if (response?.data) {
                setClassDatails(response.data);
                handlers.close();
            } else {
                console.error(response?.message);
            }
        }
    }

    const handleDelete = async () => {
        if (params.classId) {
            const parts: DeleteRequestParts = {
                apiPath: `${backendApiUrl}/classes/${params.classId}`,
            }
            const response = await deleteRequest(parts);
            
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
                        <EditClassModal className={classDetails.className} age={classDetails.age} opened={opened} handlers={handlers} handleEdit={handleEdit}/>
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


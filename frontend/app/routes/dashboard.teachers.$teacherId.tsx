import { useAuth0 } from '@auth0/auth0-react';
import { Container, Flex, Group, Text } from '@mantine/core';
import { useForm } from '@mantine/form';
import { useDisclosure } from '@mantine/hooks';
import { useNavigate, useParams } from '@remix-run/react';
import { useEffect, useState } from 'react';
import { PageLoader } from '~/components/common/PageLoader';
import useEnv from '~/hooks/useEnv';
import { Teacher, RequestPutTeacher } from '~/types/teachersTypes';
import { notifications } from '@mantine/notifications';
import { useGetRequest } from '~/hooks/request/useGetRequest';
import { DeleteRequestParts, GetRequestParts, PutRequestParts } from '~/types/requestPartsTypes';
import { usePutRequest } from '~/hooks/request/usePutRequest';
import { useDeleteRequest } from '~/hooks/request/useDeleteRequest';
import { useFrontPageLinkPath } from '~/hooks/useFrontPageLinkPath';
import { EditTeacherModal } from '~/components/teachers/EditTeacherModal';
import { DeleteTeacherModal } from '~/components/teachers/DeleteTeacherModal';

export default function TeacherDetails() {
    const { isAuthenticated } = useAuth0();
    const params = useParams();
    const [teacherDetails, setTeacherDetails] = useState<Teacher | null>(null);
    const { getRequest, isLoading, setIsLoading } = useGetRequest();
    const { putRequest } = usePutRequest();
    const { deleteRequest } = useDeleteRequest();
    const [opened, handlers] = useDisclosure(false);
    const navigate = useNavigate();
    const { frontPageLinkPath } = useFrontPageLinkPath();
    const { backendApiUrl } = useEnv();

    useEffect(() => {
        const fetchTeacher = async () => {
            setIsLoading(true);
            const parts: GetRequestParts = {
                apiPath: `${backendApiUrl}/teachers/${params.teacherId}`,
            }
            const response = await getRequest<Teacher>(parts);
            if (response?.success && response?.data) {
                setTeacherDetails(response.data);
            } else {
                console.error(response?.message);
            }
            setIsLoading(false);
        };
        fetchTeacher();
    }, [params.teacherId]);

    const formatDate = (dateStr: string) => {
        const date = new Date(dateStr);
        return date.toLocaleDateString();
    }

    const handleEdit = async (values: RequestPutTeacher) => {
        if (params.teacherId) {
            const parts: PutRequestParts = {
                apiPath: `${backendApiUrl}/teachers/${params.teacherId}`,
                payload: values,
            }
            const response = await putRequest<Teacher>(parts);
            if (response?.data) {
                setTeacherDetails(response.data);
                handlers.close();
                notifications.show({
                    title: "通知", 
                    message: "教職員情報が更新されました。"
                });
            } else {
                console.error(response?.message);
            }
        }
    }

    const handleDelete = async () => {
        if (params.teacherId && teacherDetails) {
            const parts: DeleteRequestParts = {
                apiPath: `${backendApiUrl}/teachers/${params.teacherId}`,
            }
            const response = await deleteRequest(parts);
            
            if (response?.success) {
                navigate(frontPageLinkPath.teachersPath);
                handlers.close();
                return notifications.show({
                    title: "通知", 
                    message: `${teacherDetails.lastName} ${teacherDetails.firstName}先生が削除されました。`
                });
            } else {
                console.log(response?.message);
            }
        }
    }

    if (!isAuthenticated) {
        return <Text c="red">ログインが必要です。</Text>
    }

    if (isLoading) {
        return ( <PageLoader /> );
    }

    return (
        teacherDetails ? (
            <Container size="xs">
                <Flex
                mih={50}
                gap="md"
                justify="center"
                align="center"
                direction="column"
                wrap="wrap"
                >
                    <p>氏名: {teacherDetails.lastName} {teacherDetails.firstName}</p>
                    <p>フリガナ: {teacherDetails.lastNameKana} {teacherDetails.firstNameKana}</p>
                    {teacherDetails.phone && <p>電話番号: {teacherDetails.phone}</p>}
                    {teacherDetails.mobilePhone && <p>携帯電話: {teacherDetails.mobilePhone}</p>}
                    {teacherDetails.email && <p>メールアドレス: {teacherDetails.email}</p>}
                    <p>郵便番号: {teacherDetails.postCode1}-{teacherDetails.postCode2}</p>
                    <p>住所: {teacherDetails.prefecture}{teacherDetails.city}{teacherDetails.streetAddress}</p>
                    {teacherDetails.building && <p>建物: {teacherDetails.building}</p>}
                    <p>入社日: {formatDate(teacherDetails.hireDate)}</p>
                    <p>ステータス: {teacherDetails.status}</p>
                    <p>作成日: {formatDate(teacherDetails.createdAt)}</p>
                    <p>更新日: {formatDate(teacherDetails.updatedAt)}</p>
                    <Group>
                        <EditTeacherModal teacher={teacherDetails} opened={opened} handlers={handlers} handleEdit={handleEdit}/>
                        <DeleteTeacherModal teacher={teacherDetails} handleDelete={handleDelete}/>
                    </Group>
                </Flex>
            </Container>          
        ) : (
            <Container size="xs">
                <Text c="red">教職員の詳細データが存在しません</Text>
            </Container>
        )       
    )
}

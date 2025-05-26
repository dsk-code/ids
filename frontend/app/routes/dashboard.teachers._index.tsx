import { useAuth0 } from "@auth0/auth0-react";
import { Button, Card, Center, Container, Group, Modal, Space, Text } from "@mantine/core";
import { Form, isNotEmpty, useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { Link } from "@remix-run/react";
import { useEffect } from "react";
import { useRecoilState } from "recoil";
import { CreateClassModal } from "~/components/class/CreateClassModal";
import { PageLoader } from "~/components/common/PageLoader";
import { CreateTeacherModal } from "~/components/teachers/CreateTeacherModal";
import { useGetRequest } from "~/hooks/request/useGetRequest";
import { usePostRequest } from "~/hooks/request/usePostRequest";
import useEnv from "~/hooks/useEnv";
import { classListState, teachersListState } from "~/recoil/atoms";
import { Class, RequestPostClass } from "~/types/classTypes";
import { GetRequestParts, PostRequestParts } from "~/types/requestPartsTypes";
import { Teacher } from "~/types/teachersTypes";

export default function TeachersList() {
    const { isAuthenticated } = useAuth0();
    const { backendApiUrl } = useEnv();
    // const [classList, setClassList] = useRecoilState(classListState);
    const [teachersList, setTeachersList] = useRecoilState(teachersListState);
    const { getRequest, isLoading, setIsLoading } = useGetRequest();
    const { postRequest } = usePostRequest();
    const [opened, handlers] = useDisclosure(false);
    const form = useForm({
        mode: 'uncontrolled',
        initialValues: {
            lastName: "",
            firstName: "",
            lastNameKana: "",
            firstNameKana: "",
            birthdate: "",
            phone1: "",
            phone2: "",
            phone3: "",
            mobilePhone1: "",
            mobilePhone2: "",
            mobilePhone3: "",
            email: "",
            postCode: "",
            prefecture: "",
            city: "",
            streetAddress: "",
            building: "",
            hireDate: "",
        },
        validate: {
            lastName: isNotEmpty("入力してください"),
            firstName: isNotEmpty("入力してください"),
            lastNameKana: (value) => /^[\p{Script=Katakana}ー々]+$/u.test(value) ? null : "カタカナで入力してください",
            firstNameKana: (value) => /^[\p{Script=Katakana}ー々]+$/u.test(value) ? null : "カタカナで入力してください",
            birthdate: isNotEmpty("生年月日を入力してください"),
            phone1: (value) => value === "" || /^[0-9]+$/.test(value) ? null : "半角数字で入力してください",
            phone2: (value) => value === "" || /^[0-9]+$/.test(value) ? null : "半角数字で入力してください",
            phone3: (value) => value === "" || /^[0-9]+$/.test(value) ? null : "半角数字で入力してください",
            mobilePhone1: (value) => value === "" || /^[0-9]+$/.test(value) ? null : "半角数字で入力してください",
            mobilePhone2: (value) => value === "" || /^[0-9]+$/.test(value) ? null : "半角数字で入力してください",
            mobilePhone3: (value) => value === "" || /^[0-9]+$/.test(value) ? null : "半角数字で入力してください",
            email: (value) => value === "" || /^\w+([.-]?\w+)*@\w+([.-]?\w+)*(\.\w{2,})+$/.test(value) ? null : "無効なメールアドレスです",
            postCode: (value) => /^[0-9]{7}$/.test(value) ? null : "7桁の半角数字のみ入力してください",
            prefecture: isNotEmpty("入力してください"),
            city: isNotEmpty("入力してください"),
            streetAddress: isNotEmpty("入力してください"),
            hireDate: isNotEmpty("入園日を入力してください"),
        },
        validateInputOnBlur: true,

    });

    useEffect(() => {
        const fetchTeachersList = async () => {
            setIsLoading(true);
            const parts: GetRequestParts = {
                apiPath: `${backendApiUrl}/teachers?offset=0&limit=50&status=active`,
            }
            const response = await getRequest<PaginatedTeachersList>(parts);
            if (response?.success && response?.data) {
                setTeachersList(response.data.teachers);
            } else {
                console.error(response?.message);
            }
            setIsLoading(false);
        };
        fetchTeachersList();
    }, [teachersList.length]);

    const handleCreate = async (values: typeof form.values) => {
        const payload: RequestPostTeacher = {
            lastName: values.lastName,
            firstName: values.firstName,
            lastNameKana: values.lastNameKana,
            firstNameKana: values.firstNameKana,
            phone: values.phone1 && values.phone2 && values.phone3 ? 
                `${values.phone1}-${values.phone2}-${values.phone3}` : undefined,
            mobilePhone: values.mobilePhone1 && values.mobilePhone2 && values.mobilePhone3 ? 
                `${values.mobilePhone1}-${values.mobilePhone2}-${values.mobilePhone3}` : undefined,
            email: values.email || undefined,
            postCode: values.postCode,
            prefecture: values.prefecture,
            city: values.city,
            streetAddress: values.streetAddress,
            building: values.building || undefined,
            hireDate: values.hireDate,
        }
        const parts: PostRequestParts = {
            apiPath: `${backendApiUrl}/teachers`,
            payload
        }

        const response = await postRequest<Teacher>(parts);
        if (response?.success && response.data) {
            setTeachersList([...teachersList, response.data]);
            handlers.close();
            form.reset();
        } else {
            console.log(response?.message);
        }
    }

    if (isLoading) {
        return ( <PageLoader /> );
    }
    
    return (
        isAuthenticated && (
            teachersList.length > 0 ? (
                <>
                    <Space h="md" />
                    <Container size="xs">
                        <Group justify="space-between">
                            <h1>教職員一覧</h1>
                            <CreateTeacherModal opened={opened} handlers={handlers} handleCreate={handleCreate} form={form}/>
                        </Group>
                        <ul>
                            {teachersList && (
                                teachersList.map((teacher) => (
                                    <Link to={`/dashboard/teachers/${teacher.id}`} key={teacher.id}>
                                        <Card className="border-2 border-gray-300 transition translate-y-4 hover:bg-gray-50 hover:shadow-lg" padding="lg" m="sm" radius="md" withBorder >
                                            <p>{teacher.lastName} {teacher.firstName}先生</p>
                                            <p>ステータス: {teacher.status}</p>
                                        </Card>
                                    </Link>
                                ))
                            ) }          
                        </ul>
                    </Container>
                </>
            ) : (
                <>
                    <Space h="md" />
                    <Center>
                        <Group justify="space-between">
                            <Text c="red">教職員が登録されていません</Text>
                            <CreateTeacherModal opened={opened} handlers={handlers} handleCreate={handleCreate} form={form}/>                                
                        </Group>
                    </Center>
                </>
            )
      
        )
    );
  }

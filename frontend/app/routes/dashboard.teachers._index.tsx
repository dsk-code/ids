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
import { Class, RequestPostClass } from "~/types/classes/classTypes";
import { GetRequestParts, PostRequestParts } from "~/types/request/requestPartsTypes";
import { Teacher } from "~/types/teachers/teachersTypes";
import { AsYouType, isValidPhoneNumber } from 'libphonenumber-js'

export default function TeachersList() {
    const { isAuthenticated } = useAuth0();
    const { backendApiUrl } = useEnv();
    // const [classList, setClassList] = useRecoilState(classListState);
    const [teachersList, setTeachersList] = useRecoilState(teachersListState);
    const { getRequest, isLoading, setIsLoading } = useGetRequest();
    const { postRequest } = usePostRequest();
    const [opened, handlers] = useDisclosure(false);
    const form = useForm({
        mode: 'controlled',
        initialValues: {
            lastName: "",
            firstName: "",
            lastNameKana: "",
            firstNameKana: "",
            birthdate: "",
            phone: "",
            mobilePhone: "",
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
            phone: (value) => value === "" || isValidPhoneNumber(value, 'JP') ? null : "正しい形式で入力してください",
            mobilePhone: (value) => value === "" || isValidPhoneNumber(value, 'JP') ? null : "正しい形式で入力してください",
            email: (value) => value === "" || /^\w+([.-]?\w+)*@\w+([.-]?\w+)*(\.\w{2,})+$/.test(value) ? null : "無効なメールアドレスです",
            postCode: (value) => /^[0-9]{3}-[0-9]{4}$/.test(value) ? null : "正しい形式で入力してください",
            prefecture: isNotEmpty("入力してください"),
            city: isNotEmpty("入力してください"),
            streetAddress: isNotEmpty("入力してください"),
            hireDate: isNotEmpty("入園日を入力してください"),
        },
        validateInputOnBlur: true,
    });

    // useEffect(() => {
        // const fetchClassList = async () => {
        //     setIsLoading(true);
        //     const parts: GetRequestParts = {
        //         apiPath: `${backendApiUrl}/classes`,
        //     }
        //     const response = await getRequest<Class[]>(parts);
        //     if (response?.success && response?.data) {
        //         setClassList(response.data);
        //     } else {
        //         console.error(response?.message);
        //     }
        // };
        // fetchClassList();
        // setIsLoading(false);
    // }, [classList.length]);

    const handleCreate = async (values: typeof form.values) => {
        // const age = parseInt(values.age, 10);
        // const payload: RequestPostClass = {
        //     className: values.className,
        //     age,
        // }
        // const parts: PostRequestParts = {
        //     apiPath: `${backendApiUrl}/classes`,
        //     payload
        // }

        // const response = await postRequest<Teacher>(parts);

        // if (response?.success) {
        //     if (response.data) {
        //         setTeachersList([...teachersList, response.data]);
        //         handlers.close();
        //     }
        // } else {
        //     console.log(response?.message);
        // }
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
                                teachersList.map((cls) => (
                                    <Link to={`/dashboard/classes/${cls.id}`} key={cls.id}>
                                        <Card className="border-2 border-gray-300 transition translate-y-4 hover:bg-gray-50 hover:shadow-lg" padding="lg" m="sm" radius="md" withBorder >
                                            <p>{cls.lastName}: {cls.firstName}歳</p>
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

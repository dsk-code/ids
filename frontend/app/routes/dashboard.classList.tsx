import { useAuth0 } from "@auth0/auth0-react";
import { Card, Center, Container, Group, Space, Text } from "@mantine/core";
import { useForm } from "@mantine/form";
import { useDisclosure } from "@mantine/hooks";
import { Link } from "@remix-run/react";
import { useEffect } from "react";
import { useRecoilState } from "recoil";
import { CreateClassModal } from "~/components/class/CreateClassModal";
import { PageLoader } from "~/components/common/PageLoader";
import { useGetRequest } from "~/hooks/request/useGetRequest";
import { usePostRequest } from "~/hooks/request/usePostRequest";
import useEnv from "~/hooks/useEnv";
import { classListState } from "~/recoil/atoms";
import { Class, RequestPostClass } from "~/types/classTypes";
import { GetRequestParts, PostRequestParts } from "~/types/requestPartsTypes";

export default function ClassList() {
    const { isAuthenticated } = useAuth0();
    const { backendApiUrl } = useEnv();
    const [classList, setClassList] = useRecoilState(classListState);
    const { getRequest, isLoading, setIsLoading } = useGetRequest();
    const { postRequest } = usePostRequest();
    const [opened, handlers] = useDisclosure(false);
    const form = useForm({
        mode: 'uncontrolled',
        initialValues: {
            className: "",
            age: "0",
        }
    });

    useEffect(() => {
        const fetchClassList = async () => {
            setIsLoading(true);
            const parts: GetRequestParts = {
                apiPath: `${backendApiUrl}/classes`,
            }
            const response = await getRequest<Class[]>(parts);
            if (response?.success && response?.data) {
                setClassList(response.data);
            } else {
                console.error(response?.message);
            }
        };
        fetchClassList();
        setIsLoading(false);
    }, [classList.length]);

    const handleCreate = async (values: typeof form.values) => {
        const age = parseInt(values.age, 10);
        const payload: RequestPostClass = {
            className: values.className,
            age,
        }
        const parts: PostRequestParts = {
            apiPath: `${backendApiUrl}/classes`,
            payload
        }

        const response = await postRequest<Class>(parts);

        if (response?.success) {
            if (response.data) {
                setClassList([...classList, response.data]);
                handlers.close();
            }
        } else {
            console.log(response?.message);
        }
    }

    if (isLoading) {
        return ( <PageLoader /> );
    }
    
    return (
        isAuthenticated && (
            classList.length > 0 ? (
                <>
                    <Space h="md" />
                    <Container size="xs">
                        <Group justify="space-between">
                            <h1>クラス一覧</h1>
                            <CreateClassModal opened={opened} handlers={handlers} handleCreate={handleCreate} form={form}/>
                        </Group>
                        <ul>
                            {classList && (
                                classList.map((cls) => (
                                    <Link to={`/dashboard/${cls.id}`} key={cls.id}>
                                        <Card className="border-2 border-gray-300 transition translate-y-4 hover:bg-gray-50 hover:shadow-lg" padding="lg" m="sm" radius="md" withBorder >
                                            <p>{cls.className}: {cls.age}歳</p>
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
                            <Text c="red">クラスが登録されていません</Text>
                            <CreateClassModal opened={opened} handlers={handlers} handleCreate={handleCreate} form={form}/>
                        </Group>
                    </Center>
                </>
            )
      
        )
    );
  }

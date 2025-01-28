import { useAuth0 } from "@auth0/auth0-react";
import { Card, Center, Container, Group, Space, Text } from "@mantine/core";
import { Link } from "@remix-run/react";
import { useEffect, useState } from "react";
import { useRecoilState } from "recoil";
import { getData } from "~/api/api";
import { CreateClassModal } from "~/components/class/CreateClassModal";
import { PageLoader } from "~/components/common/PageLoader";
import useEnv from "~/hooks/useEnv";
import { classListState } from "~/recoil/atoms";
import { Class } from "~/types/classTypes";

export default function ClassList() {
    const { isAuthenticated, getAccessTokenSilently } = useAuth0();
    const [isLoading, setIsLoading] = useState(false);
    const { audience, backendApiUrl } = useEnv();
    const [classList, setClassList] = useRecoilState(classListState);

    useEffect(() => {
        const fetchClassList = async () => {
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
            } finally {
                setIsLoading(false);
            }
        };
        fetchClassList();
    }, [classList.length]);

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
                            <CreateClassModal />
                            {/* <Button
                            variant="gradient"
                            gradient={{ from: 'teal', to: 'gray', deg: 0 }}
                            onClick={handleCreate}
                            >
                            新規作成
                            </Button> */}
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
                            <CreateClassModal />
                        </Group>
                    </Center>
                </>
            )
      
        )
    );
  }

import { useAuth0 } from "@auth0/auth0-react";
import useEnv from '../hooks/useEnv';
import { postData } from '../api/api';
import { Button, Container, Space, TextInput } from '@mantine/core';
import { useForm } from '@mantine/form';
import { Class, RequestPostClass } from '~/types/classTypes';
import { Input, Text } from '@mantine/core';
import { TbChevronDown } from "react-icons/tb";

export default function ClassCreateForm() {
    // https://auth0.com/docs/quickstart/spa/react/02-calling-an-api
    const { isAuthenticated, getAccessTokenSilently } = useAuth0();
    const { audience, backendApiUrl } = useEnv();

    const form = useForm({
        mode: 'uncontrolled',
        initialValues: {
            className: "",
            age: "",
        }
    });

    const show = async (values: typeof form.values) => {
        try {
            // アクセストークンの取得
            console.log("リクエスト開始");
            const accessToken = await getAccessTokenSilently({
                authorizationParams: {
                    audience: audience,
                },
            }).catch((error) => {
                console.error('アクセストークンの取得に失敗しました:', error);
            });

            console.log(accessToken);

            if (accessToken) {
                const age = parseInt(values.age, 10)
                const payload:  RequestPostClass = {
                    className: values.className,
                    age: age,
                };

                console.log(payload);
        
                // APIにPOSTリクエスト
                console.log("リクエスト開始");
                const response = await postData<Class>(
                    `${backendApiUrl}/classes`,
                    payload,
                    accessToken
                );
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

    return (
        isAuthenticated && (
            <div>
                <Space h="md" />
                <Container size="xs">
                    <form onSubmit={form.onSubmit(show)}>
                        <div>
                            <TextInput label="クラス名" placeholder='クラス名' aria-label='className' {...form.getInputProps("className")}/>
                        </div>
                        <div>
                            <TextInput
                                label="年齢"
                                component="select"
                                rightSection={<TbChevronDown size={14} stroke={"1.5"} />}
                                pointer
                                mt="md"
                                aria-label='age'
                                {...form.getInputProps("age")}
                                >
                                <option value="0">0</option>
                                <option value="1">1</option>
                                <option value="2">2</option>
                                <option value="3">3</option>
                                <option value="4">4</option>
                                <option value="5">5</option>
                            </TextInput>
                        </div>
                        <div>
                            <Space h="md" />
                            <Button variant="filled" color="violet" radius="md" type='submit'>送信</Button> 
                        </div>
                    </form>
                </Container>
            </div>
        )
    );
};

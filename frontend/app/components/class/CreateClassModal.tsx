import { Modal, Button, Container, TextInput, Space, Group } from '@mantine/core';
import { TbChevronDown } from 'react-icons/tb';
import { useForm } from '@mantine/form';
import React from 'react';
import { RequestPostClass, RequestPutClass } from '~/types/classTypes';
import { useDisclosure } from '@mantine/hooks';
import { useRecoilState } from 'recoil';
import { classListState } from '~/recoil/atoms';
import { usePostClass } from '~/hooks/class/usePostClass';

export const CreateClassModal: React.FC = () => {
    const { postClass } = usePostClass();
    const [opened, handlers] = useDisclosure(false);
    const [classList, setClassList] = useRecoilState(classListState);
    const form = useForm({
        mode: 'uncontrolled',
        initialValues: {
            className: "",
            age: "0",
        }
    });

    const handleCreate = async (values: typeof form.values) => {
        const age = parseInt(values.age, 10);
        const payload: RequestPostClass = {
            className: values.className,
            age,
        }

        const response = await postClass(payload);

        if (response?.success) {
            if (response.data) {
                setClassList([...classList, response.data]);
                handlers.close();
            }
        } else {
            console.log(response?.message);
        }
    }
    
    return (
        <>
            <Modal opened={opened} onClose={handlers.close} title="クラス編集">
                <Container size="xs">
                    <form onSubmit={form.onSubmit(handleCreate)}>
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
                            <Group justify="flex-end">
                                <Button variant="outline" color="lime" radius="md" onClick={handlers.close} >キャンセル</Button> 
                                <Button variant="filled" color="violet" radius="md" type='submit'>送信</Button> 
                            </Group>
                        </div>
                    </form>
                </Container>
            </Modal>

            <Button
            variant="gradient"
            gradient={{ from: 'teal', to: 'gray', deg: 0 }}
            onClick={handlers.open}
            >
                新規作成
            </Button>
        </>
    );
}
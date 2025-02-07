import { Modal, Button, Container, TextInput, Space, Group } from '@mantine/core';
import { TbChevronDown } from 'react-icons/tb';
import { UseFormReturnType } from '@mantine/form';
import React from 'react';

interface Props {
    opened: boolean;
    handlers: {
        readonly open: () => void;
        readonly close: () => void;
        readonly toggle: () => void;
    };
    handleCreate: (values: {
        className: string;
        age: string;
    }) => Promise<void>;
    form: UseFormReturnType<{
        className: string;
        age: string;
    }, (values: {
        className: string;
        age: string;
    }) => {
        className: string;
        age: string;
    }>;
}

export const CreateClassModal: React.FC<Props> = ({
    opened,
    handlers,
    handleCreate,
    form
}) => {
    
    return (
        <>
            <Modal opened={opened} onClose={handlers.close} title="クラス作成">
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
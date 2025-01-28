import { Button, Container, Group, Modal, Text } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import React from "react";
import { useAccessToken } from "~/hooks/accesstoken/useAccessToken";
import { useDeleteClass } from "~/hooks/class/useDeleteClass";

interface Props {
    className: string;
    age: number;
    handleDelete: () => void;
}

export const DeleteClassModal: React.FC<Props> = ({ className, age, handleDelete}) => {
    const [opened, { open, close }] = useDisclosure(false);
    const { deleteClass } = useDeleteClass();

    // const handleDelete = async () => {
    //     const response = await deleteClass();
    // }

    return (
        <>
        <Modal opened={opened} onClose={close} title="警告">
            <Container>
                <Text size="xs">クラス名 :  {className}</Text>
                <Text size="xs">年齢 : {age}</Text>
                <Text size="xs" c="red">を削除してもよろしいですか？</Text>
                <Group justify="flex-end">
                    <Button variant="outline" color="lime" radius="md" onClick={close} >キャンセル</Button> 
                    <Button variant="filled" color="red" radius="md" onClick={handleDelete}>削除</Button> 
                </Group>
            </Container>
        </Modal>

        <Button variant="filled" color="red" size="xs" onClick={open}>
            削除
        </Button>
        </>
    );
}
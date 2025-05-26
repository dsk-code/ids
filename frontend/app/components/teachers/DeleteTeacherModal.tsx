import { Button, Container, Group, Modal, Text } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import React from "react";
import { Teacher } from "~/types/teachersTypes";

interface Props {
    teacher: Teacher;
    handleDelete: () => void;
}

export const DeleteTeacherModal: React.FC<Props> = ({ teacher, handleDelete }) => {
    const [opened, { open, close }] = useDisclosure(false);

    return (
        <>
        <Modal opened={opened} onClose={close} title="警告">
            <Container>
                <Text size="xs">教職員名 : {teacher.lastName} {teacher.firstName}</Text>
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

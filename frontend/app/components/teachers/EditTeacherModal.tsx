import { Modal, Button, Container, TextInput, Space, Group, Box } from '@mantine/core';
import { useForm } from '@mantine/form';
import React, { useState } from 'react';
import { Teacher, RequestPutTeacher } from '~/types/teachersTypes';
import { useFormInputKeyDown } from '~/hooks/useFormInputKeyDown';
import { useGetRequest } from '~/hooks/request/useGetRequest';
import { ResponseGetAddress } from '~/types/addressTypes';
import { GetRequestParts } from '~/types/requestPartsTypes';
import useEnv from '~/hooks/useEnv';
import { useDisclosure } from '@mantine/hooks';

interface Props {
    teacher: Teacher;
    opened: boolean;
    handlers: {
        readonly open: () => void;
        readonly close: () => void;
        readonly toggle: () => void;
    }
    handleEdit: (values: RequestPutTeacher) => Promise<void>;
}

export const EditTeacherModal: React.FC<Props> = ({ teacher, opened, handlers, handleEdit }) => {
    const [value, setValue] = useState<Date | null>(null);
    const { handleFormInputKeyDown } = useFormInputKeyDown();
    const { getRequest } = useGetRequest();
    const { backendApiUrl } = useEnv();
    const [formKey, setFormKey] = useState(0);
    const [visible, { toggle, close }] = useDisclosure(false);

    const phoneNumbers = teacher.phone ? teacher.phone.split('-') : ['', '', ''];
    const mobilePhoneNumbers = teacher.mobilePhone ? teacher.mobilePhone.split('-') : ['', '', ''];

    const form = useForm({
        mode: 'uncontrolled',
        initialValues: {
            lastName: teacher.lastName,
            firstName: teacher.firstName,
            lastNameKana: teacher.lastNameKana || '',
            firstNameKana: teacher.firstNameKana || '',
            phone1: phoneNumbers[0],
            phone2: phoneNumbers[1],
            phone3: phoneNumbers[2],
            mobilePhone1: mobilePhoneNumbers[0],
            mobilePhone2: mobilePhoneNumbers[1],
            mobilePhone3: mobilePhoneNumbers[2],
            email: teacher.email || '',
            postCode1: teacher.postCode1,
            postCode2: teacher.postCode2,
            prefecture: teacher.prefecture,
            city: teacher.city,
            streetAddress: teacher.streetAddress,
            building: teacher.building || '',
            hireDate: teacher.hireDate,
        },
        validate: {
            lastName: (value) => value.trim() === '' ? '入力してください' : null,
            firstName: (value) => value.trim() === '' ? '入力してください' : null,
            lastNameKana: (value) => value && !/^[\p{Script=Katakana}ー々]+$/u.test(value) ? 'カタカナで入力してください' : null,
            firstNameKana: (value) => value && !/^[\p{Script=Katakana}ー々]+$/u.test(value) ? 'カタカナで入力してください' : null,
            phone1: (value) => value && !/^[0-9]+$/.test(value) ? '半角数字で入力してください' : null,
            phone2: (value) => value && !/^[0-9]+$/.test(value) ? '半角数字で入力してください' : null,
            phone3: (value) => value && !/^[0-9]+$/.test(value) ? '半角数字で入力してください' : null,
            mobilePhone1: (value) => value && !/^[0-9]+$/.test(value) ? '半角数字で入力してください' : null,
            mobilePhone2: (value) => value && !/^[0-9]+$/.test(value) ? '半角数字で入力してください' : null,
            mobilePhone3: (value) => value && !/^[0-9]+$/.test(value) ? '半角数字で入力してください' : null,
            email: (value) => value && !/^\w+([.-]?\w+)*@\w+([.-]?\w+)*(\.\w{2,})+$/.test(value) ? '無効なメールアドレスです' : null,
            postCode1: (value) => !/^[0-9]{3}$/.test(value) ? '3桁の半角数字のみ入力してください' : null,
            postCode2: (value) => !/^[0-9]{4}$/.test(value) ? '4桁の半角数字のみ入力してください' : null,
            prefecture: (value) => value.trim() === '' ? '入力してください' : null,
            city: (value) => value.trim() === '' ? '入力してください' : null,
            streetAddress: (value) => value.trim() === '' ? '入力してください' : null,
            hireDate: (value) => value.trim() === '' ? '入社日を入力してください' : null,
        }
    });

    const handleClick = () => {
        handlers.open();
    }

    const handleGetAddress = async () => {
        const getedValues = form.getValues();
        if (!getedValues.postCode1 || !getedValues.postCode2) {
            return;
        }
        toggle();
        const parts: GetRequestParts = {
            apiPath: `${backendApiUrl}/get_address?postCode=${getedValues.postCode1}${getedValues.postCode2}`,
        }
        console.log(parts);
        const response = await getRequest<ResponseGetAddress>(parts);
        if (response?.success) {
            if (response.data) {
                response.data.results.map((address) => {
                    form.resetDirty();
                    form.setValues((prev) => ({
                        ...prev,
                        prefecture: address.address1,
                        city: `${address.address2}${address.address3}`,
                    }));
                    close();
                    setFormKey((prev) => prev + 1);
                    console.log("Updated form values:", form.values);
                })
            }
        } else {
            console.error(response?.message);
        }
    }

    const handleSubmit = (values: typeof form.values) => {
        const payload: RequestPutTeacher = {
            lastName: values.lastName,
            firstName: values.firstName,
            lastNameKana: values.lastNameKana || undefined,
            firstNameKana: values.firstNameKana || undefined,
            phone: values.phone1 && values.phone2 && values.phone3 ? 
                `${values.phone1}-${values.phone2}-${values.phone3}` : undefined,
            mobilePhone: values.mobilePhone1 && values.mobilePhone2 && values.mobilePhone3 ? 
                `${values.mobilePhone1}-${values.mobilePhone2}-${values.mobilePhone3}` : undefined,
            email: values.email || undefined,
            postCode1: values.postCode1,
            postCode2: values.postCode2,
            prefecture: values.prefecture,
            city: values.city,
            streetAddress: values.streetAddress,
            building: values.building || undefined,
            prefecturesKana: undefined,
            cityKana: undefined,
            buildingKana: undefined,
            hireDate: values.hireDate,
        };
        handleEdit(payload);
    }
    
    return (
        <>
            <Modal opened={opened} onClose={handlers.close} title="教職員編集" size="lg">
                <Container size="md">
                    <form onSubmit={form.onSubmit(handleSubmit)}>
                        <Group grow>
                            <TextInput label="姓" placeholder='姓' {...form.getInputProps("lastName")}/>
                            <TextInput label="名" placeholder='名' {...form.getInputProps("firstName")}/>
                        </Group>
                        <Group grow mt="md">
                            <TextInput label="姓（カナ）" placeholder='セイ' {...form.getInputProps("lastNameKana")}/>
                            <TextInput label="名（カナ）" placeholder='メイ' {...form.getInputProps("firstNameKana")}/>
                        </Group>
                        <Group grow mt="md">
                            <Box>
                                <Group grow>
                                    <TextInput label="電話番号" placeholder='000' {...form.getInputProps("phone1")}/>
                                    <TextInput placeholder='0000' {...form.getInputProps("phone2")}/>
                                    <TextInput placeholder='0000' {...form.getInputProps("phone3")}/>
                                </Group>
                            </Box>
                        </Group>
                        <Group grow mt="md">
                            <Box>
                                <Group grow>
                                    <TextInput label="携帯電話" placeholder='000' {...form.getInputProps("mobilePhone1")}/>
                                    <TextInput placeholder='0000' {...form.getInputProps("mobilePhone2")}/>
                                    <TextInput placeholder='0000' {...form.getInputProps("mobilePhone3")}/>
                                </Group>
                            </Box>
                        </Group>
                        <TextInput label="メールアドレス" placeholder='example@example.com' mt="md" {...form.getInputProps("email")}/>
                        <Group grow mt="md">
                            <Box>
                                <Group grow>
                                    <TextInput label="郵便番号" placeholder='000' {...form.getInputProps("postCode1")}/>
                                    <TextInput placeholder='0000' {...form.getInputProps("postCode2")}/>
                                    <Button onClick={handleGetAddress}>住所検索</Button>
                                </Group>
                            </Box>
                        </Group>
                        <TextInput label="都道府県" placeholder='都道府県' mt="md" {...form.getInputProps("prefecture")}/>
                        <TextInput label="市区町村" placeholder='市区町村' mt="md" {...form.getInputProps("city")}/>
                        <TextInput label="番地" placeholder='番地' mt="md" {...form.getInputProps("streetAddress")}/>
                        <TextInput label="建物名" placeholder='建物名' mt="md" {...form.getInputProps("building")}/>
                        <TextInput label="入社日" placeholder='YYYY-MM-DD' mt="md" {...form.getInputProps("hireDate")}/>
                        <Space h="md" />
                        <Group justify="flex-end">
                            <Button variant="outline" color="lime" radius="md" onClick={handlers.close}>キャンセル</Button> 
                            <Button variant="filled" color="violet" radius="md" type='submit'>更新</Button> 
                        </Group>
                    </form>
                </Container>
            </Modal>

            <Button variant="filled" color="green" size="xs" onClick={handleClick}>
                編集
            </Button>
        </>
    );
}

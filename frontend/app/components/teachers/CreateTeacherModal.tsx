import { Modal, Button, Container, TextInput, Space, Group, Text, Box, LoadingOverlay } from '@mantine/core';
import { UseFormReturnType } from '@mantine/form';
import React, { useState } from 'react';
import { useFormInputKeyDown } from '~/hooks/useFormInputKeyDown';
import { useGetRequest } from '~/hooks/request/useGetRequest';
import { ResponseGetAddress } from '~/types/addressTypes';
import { GetRequestParts } from '~/types/requestPartsTypes';
import useEnv from '~/hooks/useEnv';
import { useDisclosure } from '@mantine/hooks';

interface InputTeacherData {
    lastName: string;
    firstName: string;
    lastNameKana: string;
    firstNameKana: string;
    birthdate: string;
    phone1: string;
    phone2: string;
    phone3: string;
    mobilePhone1: string;
    mobilePhone2: string;
    mobilePhone3: string;
    email: string;
    postCode: string;
    prefecture: string;
    city: string;
    streetAddress: string;
    building: string;
    hireDate: string;
}

interface Props {
    opened: boolean;
    handlers: {
        readonly open: () => void;
        readonly close: () => void;
        readonly toggle: () => void;
    };
    handleCreate: (values: InputTeacherData) => Promise<void>;
    form: UseFormReturnType<InputTeacherData, (values: InputTeacherData) => InputTeacherData>;
}

export const CreateTeacherModal: React.FC<Props> = ({
    opened,
    handlers,
    handleCreate,
    form
}) => {
    const [value, setValue] = useState<Date | null>(null);
    const { handleFormInputKeyDown } = useFormInputKeyDown();
    const { getRequest } = useGetRequest();
    const { backendApiUrl } = useEnv();
    const [formKey, setFormKey] = useState(0); // フォームのキーを管理
    const [visible, { toggle, close }] = useDisclosure(false);

    const handleClick = async () => {
        const getedValues = form.getValues();
        if (form.validateField('postCode').hasError) {
            return
        }
        toggle();
        const parts: GetRequestParts = {
            apiPath: `${backendApiUrl}/get_address?postCode=${getedValues.postCode}`,
        }
        console.log(parts);
        const response = await getRequest<ResponseGetAddress>(parts);
        if (response?.success) {
            console.log(response.data);
            if (response.data) {
                response.data.results.map((address) => {
                    form.resetDirty();
                    form.setValues((prev) => ({
                        ...prev,
                        prefecture: address.address1,
                        city: `${address.address2}${address.address3}`,
                    }));
                    close();
                    setFormKey((prev) => prev + 1); // フォームを再レンダリング
                    console.log("Updated form values:", form.values);
                })
            }
        }
        close();
    }
    
    return (
        <>
            <Modal opened={opened} onClose={handlers.close} size="auto" title="教職員作成">
                <Container size="xs">
                    <Box pos="relative">
                        <LoadingOverlay visible={visible} loaderProps={{ children: '読み込み中...' }} />
                        {/* ...other content */}
                        <form key={formKey} onSubmit={form.onSubmit(handleCreate)}>
                            <Group grow>
                                <TextInput label="姓" placeholder='姓' withAsterisk aria-label='lastName' {...form.getInputProps("lastName")} onKeyDown={handleFormInputKeyDown} />
                                <TextInput label="名" placeholder='名' withAsterisk aria-label='firstName' {...form.getInputProps("firstName")} onKeyDown={handleFormInputKeyDown} />
                            </Group>
                            <Group grow>
                                <TextInput label="セイ" placeholder='セイ' withAsterisk aria-label='lastNameKana' {...form.getInputProps("lastNameKana")} onKeyDown={handleFormInputKeyDown} />
                                <TextInput label="メイ" placeholder='メイ' withAsterisk aria-label='firstNameKana' {...form.getInputProps("firstNameKana")} onKeyDown={handleFormInputKeyDown} />
                            </Group>
                            <div>
                                <Text size="sm">生年月日<span className='text-red-500'> *</span></Text>
                                <input type='date' className='bg-white text-gray-600 dark:text-gray-600 dark:bg-white border rounded-custom border-gray-300' min="1900-01-01" max="2500-12-31" required {...form.getInputProps("birthdate")}/>
                                {form.errors.birthdate && (
                                    <Text size="xs" c="red">{form.errors.birthdate}</Text>
                                )}
                            </div>
                            <Group gap="xs">
                                <TextInput label="固定電話" placeholder='0000' aria-label='phone' size="xs" styles={{ input: { minWidth: 60, maxWidth: 60 } }} {...form.getInputProps("phone1")} onKeyDown={handleFormInputKeyDown} />
                                <div>
                                    <Space h="lg" />
                                    <Text>-</Text>
                                </div>
                                <TextInput label=" " placeholder='0000' aria-label='phone' size="xs" styles={{ input: { minWidth: 60, maxWidth: 60 } }} {...form.getInputProps("phone2")} onKeyDown={handleFormInputKeyDown} />
                                <div>
                                    <Space h="lg" />
                                    <Text>-</Text>
                                </div>
                                <TextInput label=" " placeholder='0000' aria-label='phone' size="xs" styles={{ input: { minWidth: 60, maxWidth: 60 } }} {...form.getInputProps("phone3")} onKeyDown={handleFormInputKeyDown} />
                            </Group>
                            <Group gap="xs">
                                <TextInput label="携帯電話" placeholder='000' aria-label='phone' size="xs" styles={{ input: { minWidth: 60, maxWidth: 60 } }} {...form.getInputProps("mobilePhone1")} onKeyDown={handleFormInputKeyDown} />
                                <div className='ml-0'>
                                    <Space h="lg" />
                                    <Text>-</Text>
                                </div>
                                <TextInput label=" " placeholder='0000' aria-label='phone' size="xs" styles={{ input: { minWidth: 60, maxWidth: 60 } }} {...form.getInputProps("mobilePhone2")} onKeyDown={handleFormInputKeyDown} />
                                <div>
                                    <Space h="lg" />
                                    <Text>-</Text>
                                </div>
                                <TextInput label=" " placeholder='0000' aria-label='phone' size="xs" styles={{ input: { minWidth: 60, maxWidth: 60 } }} {...form.getInputProps("mobilePhone3")} onKeyDown={handleFormInputKeyDown} />
                            </Group>
                            <div>
                                <TextInput label="メールアドレス" placeholder='メールアドレス' aria-label='email' {...form.getInputProps("email")} onKeyDown={handleFormInputKeyDown} />
                            </div>
                            <Group>
                                <TextInput label="郵便番号" placeholder='例) 1600022' description='７桁のハイフンを含まない半角数字' size="xs" withAsterisk aria-label='postCode' styles={{ input: { maxWidth: 100 } }} {...form.getInputProps("postCode")} onKeyDown={handleFormInputKeyDown} />
                                <div className='ml-0 mr-0'>
                                    <Space h="md" />
                                    <Button mb={0} variant="light" color="gray" onClick={handleClick}>住所検索</Button>
                                </div>
                            </Group>
                            <Group>
                                <TextInput label="都道府県" placeholder='都道府県' withAsterisk aria-label='prefecture' styles={{ input: { maxWidth: 100 } }} {...form.getInputProps("prefecture")} onKeyDown={handleFormInputKeyDown} />
                                <TextInput label="市町村区" placeholder='市町村区' withAsterisk aria-label='city' {...form.getInputProps("city")} onKeyDown={handleFormInputKeyDown} />
                            </Group>
                            <div>
                                <TextInput label="番地" placeholder='番地' withAsterisk aria-label='streetAddress' {...form.getInputProps("streetAddress")} onKeyDown={handleFormInputKeyDown} />
                            </div>
                            <div>
                                <TextInput label="建物名、部屋番号" placeholder='建物名、部屋番号' aria-label='building' {...form.getInputProps("building")} onKeyDown={handleFormInputKeyDown}/>
                            </div>
                            {/* <div>
                                <DatesProvider  settings={{ consistentWeeks: true }}>
                                    <DateInput
                                        label="入園日"
                                        placeholder="入園日"
                                        clearable
                                        description="Input description"
                                        valueFormat="YYYY-MM-DD"
                                        monthLabelFormat="YYYY/MM"
                                        monthsListFormat="MM月"
                                        locale='ja'
                                        firstDayOfWeek={0}
                                        defaultDate={new Date()}
                                        value={value}
                                        // onChange={setValue}
                                        onKeyDown={handleFormInputKeyDown}
                                        {...form.getInputProps("hireDate")}
                                        />
                                </DatesProvider>
                            </div> */}
                            <div>
                                <Text size="sm">入園日<span className='text-red-500'> *</span></Text>
                                <input type='date' className='bg-white text-gray-600 dark:text-gray-600 dark:bg-white border rounded-custom border-gray-300' min="1900-01-01" max="2500-12-31" required {...form.getInputProps("hireDate")}/>
                                {form.errors.hireDate && (
                                    <Text size="xs" c="red">{form.errors.hireDate}</Text>
                                )}
                            </div>
                            <div>
                                <Space h="md" />
                                <Group justify="flex-end">
                                    <Button variant="outline" color="lime" radius="md" onClick={handlers.close} >キャンセル</Button> 
                                    <Button variant="filled" color="violet" radius="md" type='submit'>登録</Button> 
                                </Group>
                            </div>
                        </form>
                    </Box>
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
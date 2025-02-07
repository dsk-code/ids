import { useAuth0 } from '@auth0/auth0-react'
import { Button, Drawer, Stack } from '@mantine/core';
import { Link } from '@remix-run/react';
import React from 'react'
import Auth0NavButtons from '../auth0/Auth0NavButtons';
import { useDisclosure } from '@mantine/hooks';

export const DrawerMenuButton: React.FC = () => {
    const { isAuthenticated } = useAuth0();
    const [opened, { open, close }] = useDisclosure(false);

    return (
        <>
            <Drawer position="right" size="xs" opened={opened} onClose={close} title="Menu">
              <Stack align="flex-start" justify="flex-start">
                <Link to="/dashboard" className="font-medium text-black select-none" onClick={close}>dashboard</Link>
                <Link to="/dashboard/classes" className="font-medium text-black select-none" onClick={close}>クラス一覧</Link>
                <Auth0NavButtons />
              </Stack>
            </Drawer>
            <Button variant="default" onClick={open}>
                Open Menu
            </Button>
        </>
    )
    
}

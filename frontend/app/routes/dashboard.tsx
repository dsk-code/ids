import { useAuth0 } from "@auth0/auth0-react";
import { Link, Outlet } from '@remix-run/react';
import { AuthenticationGuard } from '../components/auth0/AuthenticationGuard';
import { AppShell, Burger, Button, Drawer, Group, Notification, Stack, Text } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import { DrawerMenuButton } from '~/components/common/DrawerMenuButton';
import { PageLoader } from '~/components/common/PageLoader';

export default function DashboardLayout() {
    const { isLoading } = useAuth0();
    const [opened, { toggle, open, close }] = useDisclosure();

    if (isLoading) {
      return (
        <div className="page-layout">
          <PageLoader />
        </div>
      );
    }

    return(
        <>
          <AuthenticationGuard>
            <AppShell
              header={{ height: 60 }}
              navbar={{
                width: 250,
                breakpoint: 'sm',
                collapsed: { mobile: !opened },               
              }}
              padding="md"
              transitionDuration={500}
            >
              <AppShell.Header>
                <Group h="100%" px="md" justify="space-between">
                  <Group h="100%" px="md">
                    <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />
                    <Text fw={700}>IDS</Text>
                  </Group>
                  <Group h="100%" px="md" justify="flex-end">
                    <DrawerMenuButton />
                  </Group>
                      {/* </div> */}
                  {/* </nav> */}
                </Group>
              </AppShell.Header>

              <AppShell.Navbar p="md">
              <Stack
                h={300}
                bg="var(--mantine-color-body)"
                align="center"
                justify="flex-start"
                gap="md"
              >
                <Link to="/dashboard" className="font-medium text-black select-none" onClick={close}>dashboard</Link>
                <Link to="/dashboard/classList" className="font-medium text-black select-none" onClick={close}>クラス一覧</Link>
              </Stack>
              </AppShell.Navbar>

              <AppShell.Main>
                <Outlet />
              </AppShell.Main>
            </AppShell>
          </AuthenticationGuard>
        </>
    )
};

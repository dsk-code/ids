import Header from '../components/common/Header';
import SideBar from '../components/common/SideBar';
import { useAuth0 } from "@auth0/auth0-react";
import PageLoader from "../components/common/PageLoader";
import { Link, Outlet } from '@remix-run/react';
import { AuthenticationGuard } from '../components/auth0/AuthenticationGuard';
import { AppShell, Burger, Group, Stack, Text } from '@mantine/core';
import { useDisclosure } from '@mantine/hooks';
import Auth0NavButtons from '~/components/auth0/Auth0NavButtons';

export default function DashboardLayout() {
    const { isLoading } = useAuth0();
    const [opened, { toggle }] = useDisclosure();

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
            {/* <div className='flex overscroll-y-contain'>
                <SideBar />
                <div className='flex-1 h-screen flex flex-col'>
                <Header />
                <main className='flex-1 overflow-y-auto overscroll-none bg-white'>
                    <Outlet />
                </main>
                </div>
            </div> */}
            <AppShell
              header={{ height: 60 }}
              navbar={{
                width: 250,
                breakpoint: 'sm',
                collapsed: { mobile: !opened },
              }}
              padding="md"
            >
              <AppShell.Header>
                <Group h="100%" px="md" justify="space-between">
                  <Group h="100%" px="md">
                    <Burger opened={opened} onClick={toggle} hiddenFrom="sm" size="sm" />
                    <Text fw={700}>IDS</Text>
                  </Group>
                  {/* <nav className="max-w-[85rem] w-full mx-auto px-4 sm:flex sm:items-center sm:justify-between" aria-label="Global"> */}
                      {/* <div className="hidden sm:block"> */}
                  <Group h="100%" px="md" justify="flex-end">
                        <Link to="/dashboard" className="hidden sm:block font-medium text-blue-500 select-none">dashboard</Link>
                        <Link to="/dashboard/classList" className="hidden sm:block font-medium text-blue-500 select-none">クラス一覧</Link>
                        <Link to="/dashboard/classCreation" className="hidden sm:block font-medium text-blue-500 select-none">クラス作成</Link>
                        <Auth0NavButtons />
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
                <Link to="/dashboard" className="hidden sm:block font-medium text-blue-500 select-none">dashboard</Link>
                <Link to="/dashboard/classList" className="hidden sm:block font-medium text-blue-500 select-none">クラス一覧</Link>
                <Link to="/dashboard/classCreation" className="hidden sm:block font-medium text-blue-500 select-none">クラス作成</Link>
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

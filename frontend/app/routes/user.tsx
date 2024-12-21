import Header from '../components/common/Header';
import SideBar from '../components/common/SideBar';
import { useAuth0 } from "@auth0/auth0-react";
import PageLoader from "../components/common/PageLoader";
import { Outlet } from '@remix-run/react';
import { AuthenticationGuard } from '../components/auth0/AuthenticationGuard';

export default function UserLayout() {
    const { isLoading } = useAuth0();

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
            <div className='flex overscroll-y-contain'>
                <SideBar />
                <div className='flex-1 h-screen flex flex-col'>
                <Header />
                <main className='flex-1 overflow-y-auto overscroll-none bg-white'>
                    <Outlet />
                </main>
                </div>
            </div>
          </AuthenticationGuard>
        </>
    )
};

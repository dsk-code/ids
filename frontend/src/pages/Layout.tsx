import React from 'react'
import Header from '../components/common/Header';
import { Outlet } from 'react-router-dom';
import SideBar from '../components/common/SideBar';
import { useAuth0 } from "@auth0/auth0-react";
import PageLoader from "../components/common/PageLoader";

const Layout: React.FC = () => {
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
            <div className='flex overscroll-y-contain'>
                <SideBar />
                <div className='flex-1 h-screen flex flex-col'>
                <Header />
                <main className='flex-1 overflow-y-auto overscroll-none'>
                    <Outlet />
                </main>
                </div>
            </div>
        </>
    )
};
export default Layout;
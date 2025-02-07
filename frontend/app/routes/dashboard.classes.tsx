import { useAuth0 } from "@auth0/auth0-react";
import { Outlet } from '@remix-run/react';
import { PageLoader } from '~/components/common/PageLoader';

export default function ClassesLayout() {
    const { isLoading, isAuthenticated } = useAuth0();

    if (isLoading) {
      return (
        <div className="page-layout">
          <PageLoader />
        </div>
      );
    }

    return(
        isAuthenticated && (
            <div>
                <Outlet />
            </div>
        )
    )
};

import { withAuthenticationRequired } from "@auth0/auth0-react";
import React, { ReactNode } from "react";
import PageLoader from "../common/PageLoader";

interface AuthenticationGuardProps {
  children: ReactNode;
}

export const AuthenticationGuard: React.FC<AuthenticationGuardProps> = ({ children }) => {
  const WrappedComponent = withAuthenticationRequired(() => <>{children}</>, {
    onRedirecting: () => (
      <div className="page-layout">
        <PageLoader />
      </div>
    ),
    returnTo: '/dashboard',
  });

  return <WrappedComponent />;
};

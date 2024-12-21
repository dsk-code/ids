// import { withAuthenticationRequired } from '@auth0/auth0-react';
// import React, { ComponentType } from 'react'
// import PageLoader from '../common/PageLoader';

// interface AuthenticationGuardProps {
//     component: ComponentType,
// }

// export const AuthenticationGuard: React.FC<AuthenticationGuardProps> = ({ component }) => {
//     const Component = withAuthenticationRequired(component, {
//         onRedirecting: () => (
//           <div className="page-layout">
//             <PageLoader />
//           </div>
//         ),
//     });
    
//     return <Component />;
// }

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
    returnTo: '/user',
  });

  return <>{children}</>;
};

import { AppState, Auth0Provider } from "@auth0/auth0-react";
import React, { ReactNode } from "react";
import { useNavigate } from "react-router-dom";
import useEnv from "../hooks/useEnv";

interface Auth0ProviderWithNavigateProps {
    children: ReactNode;
}

export const Auth0ProviderWithNavigate: React.FC<Auth0ProviderWithNavigateProps> = ({ children }) => {
  const navigate = useNavigate();
  const { domain, clientId, callbackUrl, audience, scope } = useEnv();

  // const domain = import.meta.env.VITE_AUTH0_DOMAIN as string;
  // const clientId = import.meta.env.VITE_AUTH0_CLIENT_ID as string;
  // const redirectUri = import.meta.env.VITE_AUTH0_CALLBACK_URL as string;
  // const audience = import.meta.env.VITE_AUTH0_AUDIENCE as string;
  // const scope = import.meta.env.VITE_AUTH0_SCOPE as string;

  const onRedirectCallback = (appState?: AppState) => {
    navigate(appState?.returnTo || window.location.pathname);
  };

  if (!(domain && clientId && callbackUrl)) {
    return null;
  }

  // https://auth0.com/docs/quickstart/spa/react/02-calling-an-api#get-an-access-token
  return (
    <Auth0Provider
      domain={domain}
      clientId={clientId}
      authorizationParams={{
        redirect_uri: callbackUrl,
        audience: audience,
        scope: scope
      }}
      onRedirectCallback={onRedirectCallback}
      cacheLocation="localstorage" // Cookieの代わりにLocal Storageを使用
    >
      {children}
    </Auth0Provider>
  );
};



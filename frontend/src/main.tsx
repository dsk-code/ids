import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "preline/preline";
import { RecoilRoot } from "recoil";
import { Auth0ProviderWithNavigate } from "./auth/auth0-provider-with-navigate";
import { BrowserRouter } from "react-router-dom";
// import { Auth0Provider } from "@auth0/auth0-react";


ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <BrowserRouter>
      <Auth0ProviderWithNavigate>
        <RecoilRoot>
          <App />
        </RecoilRoot>
      </Auth0ProviderWithNavigate>
    </BrowserRouter>
  </React.StrictMode>,
);


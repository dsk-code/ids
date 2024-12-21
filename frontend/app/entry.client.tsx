/**
 * By default, Remix will handle hydrating your app on the client for you.
 * You are free to delete this file if you'd like to, but if you ever want it revealed again, you can run `npx remix reveal` ✨
 * For more information, see https://remix.run/file-conventions/entry.client
 */

import { RemixBrowser } from "@remix-run/react";
import { startTransition, StrictMode } from "react";
import { hydrateRoot } from "react-dom/client";
import { Auth0ProviderWithNavigate } from "./components/auth0/auth0-provider-with-navigate";
import { RecoilRoot } from 'recoil';

startTransition(() => {
  hydrateRoot(
    document,
      <StrictMode>
        <RecoilRoot>
          <Auth0ProviderWithNavigate>
            <RemixBrowser />
          </Auth0ProviderWithNavigate>
        </RecoilRoot>
      </StrictMode>
  );
});

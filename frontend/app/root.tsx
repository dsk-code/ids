import {
  Form,
  Links,
  Meta,
  Outlet,
  Scripts,
  ScrollRestoration,
} from "@remix-run/react";
import "./styles/tailwind.css";
import '@mantine/core/styles.css';
import { ColorSchemeScript, MantineProvider, mantineHtmlProps } from '@mantine/core';

export default function App() {
  return (
      <html lang="en" {...mantineHtmlProps}>
        <head>
          <meta charSet="utf-8" />
          <meta
            name="viewport"
            content="width=device-width, initial-scale=1"
          />
          <link href="https://fonts.googleapis.com/css2?family=Poppins:wght@300;400;500;600;700&display=swap" rel="stylesheet"></link>
          <ColorSchemeScript />
          <Meta />
          <Links />
        </head>
        <body>
        <MantineProvider>
          <Outlet />
        </MantineProvider>
        </body>
      </html>
  );
}



import { auth } from "@/auth";
import { } from "@/components";
import { ReactQueryClientProvider } from "@/components/QueryClientProvider";
import { Case, Default, Switch } from "@/components/Switch";
import { Toaster } from "@/components/ui/toaster";
import { AuthenticatedLayout } from "@/layouts/Authenticated/AuthenticatedLayout";
import { AuthenticationLayout } from "@/layouts/Authentication/Authentication";
import { LiveblocksProvider } from "@/liveblocks-configs/client";
import type { Metadata } from "next";
import { SessionProvider } from "next-auth/react";
import { Inter } from "next/font/google";
import "./styles/globals.css";

const inter = Inter({ subsets: ["latin"] });

export const metadata: Metadata = {
  title: "Liveblocks",
};

export default async function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  const session = await auth();

  return (
    <html lang="en">
      <head>
        <link
          href="https://liveblocks.io/favicon-32x32.png"
          rel="icon"
          sizes="32x32"
          type="image/png"
        />
        <link
          href="https://liveblocks.io/favicon-16x16.png"
          rel="icon"
          sizes="16x16"
          type="image/png"
        />
      </head>
      <body className="bg-neutral-50 text-neutral-900 antialiased overflow-y-scroll">
        <SessionProvider session={session}>
          <LiveblocksProvider>
            <ReactQueryClientProvider>
              <Switch>
                <Case condition={Boolean(session)}>
                  <AuthenticatedLayout session={session}>
                    {children}
                  </AuthenticatedLayout>
                </Case>
                <Default>
                  <AuthenticationLayout />
                </Default>
              </Switch>
              {/* <ReactQueryDevtools initialIsOpen /> */}
              <Toaster />

            </ReactQueryClientProvider>
          </LiveblocksProvider>
        </SessionProvider>
      </body>
    </html>
  );
}

/**
 * Checking that your Liveblocks API key has been added
 * https://liveblocks.io/dashboard/apikeys
 */
const API_KEY = process.env.LIVEBLOCKS_SECRET_KEY;
const API_KEY_WARNING = process.env.CODESANDBOX_SSE
  ? `Add your secret key from https://liveblocks.io/dashboard/apikeys as the \`LIVEBLOCKS_SECRET_KEY\` secret in CodeSandbox.\n` +
  `Learn more: https://github.com/liveblocks/liveblocks/tree/main/examples/nextjs-lost-connection-toasts#codesandbox.`
  : `Create an \`.env.local\` file and add your secret key from https://liveblocks.io/dashboard/apikeys as the \`LIVEBLOCKS_SECRET_KEY\` environment variable.\n` +
  `Learn more: https://github.com/liveblocks/liveblocks/tree/main/examples/nextjs-lost-connection-toasts#getting-started.`;

if (!API_KEY) {
  console.warn(API_KEY_WARNING);
}

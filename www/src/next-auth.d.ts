import "next-auth";
import { DefaultSession } from "next-auth";


interface User {
  id?: string
  name?: string | null
  email?: string | null
  image?: string | null
}

declare module "next-auth" {
  /**
   * Returned by `useSession`, `getSession` and received as a prop on the `SessionProvider` React Context
   */
  interface Session extends DefaultSession {
    user: User;
  }
}
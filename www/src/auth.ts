
import { prisma } from '@/prisma-client';
import { PrismaAdapter } from "@auth/prisma-adapter";
import NextAuth, { NextAuthConfig, } from "next-auth";
import GoogleProvider, { GoogleProfile } from "next-auth/providers/google";
import { AUTH_SECRET, GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET } from './env';







const providers = [
    GoogleProvider<GoogleProfile>({
        clientId: GOOGLE_CLIENT_ID,
        clientSecret: GOOGLE_CLIENT_SECRET,
    })
];



export const options: NextAuthConfig = {
    providers,
    adapter: PrismaAdapter(prisma),
    secret: AUTH_SECRET

}




export const {
    handlers: { GET, POST },
    auth,
    signIn, signOut

} = NextAuth(options)

export { providers };




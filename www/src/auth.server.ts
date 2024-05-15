
import './env-config';

import { prisma } from '@/prisma-client';
import { PrismaAdapter } from "@auth/prisma-adapter";
import NextAuth, { NextAuthConfig, } from "next-auth";
import GoogleProvider, { GoogleProfile } from "next-auth/providers/google";





const providers = [
    GoogleProvider<GoogleProfile>({
        clientId: process.env.GOOGLE_CLIENT_ID,
        clientSecret: process.env.GOOGLE_CLIENT_SECRET,
    })
];






export const { handlers: { GET, POST}, auth, signIn, signOut } = NextAuth((req) => {

    const providers = [
        GoogleProvider<GoogleProfile>({
            clientId: process.env.GOOGLE_CLIENT_ID,
            clientSecret: process.env.GOOGLE_CLIENT_SECRET,
        })
    ];
    
    const options: NextAuthConfig = {
        providers,
        adapter: PrismaAdapter(prisma),
        secret: process.env.AUTH_SECRET
    
    }

    return options
})

export { providers };








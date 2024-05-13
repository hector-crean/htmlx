import { prisma } from '@/prisma-client';
import { PrismaAdapter } from "@auth/prisma-adapter";
import NextAuth, { NextAuthConfig, } from "next-auth";
import GoogleProvider, { GoogleProfile } from "next-auth/providers/google";


const providers = [
    GoogleProvider<GoogleProfile>({
        clientId: process.env.GOOGLE_CLIENT_ID!,
        clientSecret: process.env.GOOGLE_CLIENT_SECRET!
    })
];



export const options: NextAuthConfig = {
    providers,
    adapter: PrismaAdapter(prisma),
    secret: "J2X2syAzRngwiG1opUIkkSaqT+Xpu30tIWpW9Lb/br4="

}




export const {
    handlers: { GET, POST },
    auth,
    signIn, signOut

} = NextAuth(options)

export { providers };




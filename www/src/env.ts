"use server";
import './env-config';

const GOOGLE_CLIENT_ID = process.env.NEXT_GOOGLE_CLIENT_ID!;
const GOOGLE_CLIENT_SECRET = process.env.NEXT_GOOGLE_CLIENT_SECRET!
const AUTH_SECRET = process.env.NEXT_AUTH_SECRET!

const env = async () => {

    const GOOGLE_CLIENT_ID = process.env.NEXT_GOOGLE_CLIENT_ID!;
const GOOGLE_CLIENT_SECRET = process.env.NEXT_GOOGLE_CLIENT_SECRET!
const AUTH_SECRET = process.env.NEXT_AUTH_SECRET!

    return {
        GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET, AUTH_SECRET
    }
}


export { env };


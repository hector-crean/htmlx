import './env-config';

const GOOGLE_CLIENT_ID = process.env.NEXT_GOOGLE_CLIENT_ID!;
const GOOGLE_CLIENT_SECRET = process.env.NEXT_GOOGLE_CLIENT_SECRET!
const AUTH_SECRET = process.env.NEXT_AUTH_SECRET!



export { AUTH_SECRET, GOOGLE_CLIENT_ID, GOOGLE_CLIENT_SECRET };


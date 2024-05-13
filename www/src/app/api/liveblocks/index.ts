import { Liveblocks } from "@liveblocks/node";

const LIVEBLOCKS_SECRET_KEY = process.env.LIVEBLOCKS_SECRET_KEY ?? 'sk_dev_DeSgaxOtliASkZ1DiFF_sM3iUhKYVBSDA53LxPZsdC6NiG6zMDF6jrlW4W-8zbkm' as string

const liveblocks = new Liveblocks({
    secret: LIVEBLOCKS_SECRET_KEY
});

export { LIVEBLOCKS_SECRET_KEY, liveblocks };




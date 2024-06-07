import { Liveblocks } from "@liveblocks/node";

const LIVEBLOCKS_SECRET_KEY = "sk_prod_CaklGs0VI-T7UxGn8LX-RZaJIEcg9YeNHcB416LvHt4wY4z0R6Joo2sAAePxJQpD"

const liveblocks = new Liveblocks({
    secret: LIVEBLOCKS_SECRET_KEY
});

export { LIVEBLOCKS_SECRET_KEY, liveblocks };




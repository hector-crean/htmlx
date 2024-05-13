import { LIVEBLOCKS_SECRET_KEY } from "@/app/api/liveblocks";
import { RoomInfoSchema } from "@/lib/types";
import axios from 'axios';
import { z } from 'zod';


const RoomsQueryResultSchema = z.object({
    nextPage: z.string().nullable(),
    nextCursor: z.string().nullable(),
    data: z.array(RoomInfoSchema)
})

type RoomsQueryResultType = z.infer<typeof RoomsQueryResultSchema>;


async function getRooms(): Promise<RoomsQueryResultType> {

    return axios
        .get(`https://api.liveblocks.io/v2/rooms`, {
            headers: {
                Authorization: `Bearer ${LIVEBLOCKS_SECRET_KEY}`,
            },
        })
        .then((res) => res.data)
}

export { getRooms };


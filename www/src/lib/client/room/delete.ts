import { LIVEBLOCKS_SECRET_KEY } from "@/app/api/liveblocks";
import { RoomInfoSchema, RoomInfoType } from "@/lib/types";
import axios, { AxiosResponse } from 'axios';
import { z } from 'zod';



/**
 * How do we keep the liveblocks api types well syncronised with our zod types we define here?
 * - zod types make it more convenient to validate form inputs etc.
 */


const DeleteRoomParamsSchema = RoomInfoSchema.pick({ id: true, })
type DeleteRoomParamsType = z.infer<typeof DeleteRoomParamsSchema>;


async function deleteRoom(request: DeleteRoomParamsType): Promise<RoomInfoType> {

  const req =  axios
    .delete<DeleteRoomParamsType, AxiosResponse<RoomInfoType>>(`https://api.liveblocks.io/v2/rooms/${request.id}`, {
      headers: {
        Authorization: `Bearer ${LIVEBLOCKS_SECRET_KEY}`,
      },
    })
    .then((res) => res.data)

    return req;

}


export { DeleteRoomParamsSchema, deleteRoom };
export type { DeleteRoomParamsType };


import { LIVEBLOCKS_SECRET_KEY } from "@/app/api/liveblocks";
import { RoomInfoSchema, RoomInfoType } from "@/lib/types";
import axios, { AxiosResponse } from 'axios';
import { z } from 'zod';



/**
 * How do we keep the liveblocks api types well syncronised with our zod types we define here?
 * - zod types make it more convenient to validate form inputs etc.
 */


const CreateRoomParamsSchema = RoomInfoSchema.pick({ id: true, defaultAccesses: true, metadata: true, usersAccesses: true, groupsAccesses: true })
type CreateRoomParamsType = z.infer<typeof CreateRoomParamsSchema>;


async function createRoom(request: CreateRoomParamsType): Promise<RoomInfoType> {

  return axios
    .post<CreateRoomParamsType, AxiosResponse<RoomInfoType>>(`https://api.liveblocks.io/v2/rooms`, request, {
      headers: {
        Authorization: `Bearer ${LIVEBLOCKS_SECRET_KEY}`,
      },
    })
    .then((res) => res.data)

}


export { CreateRoomParamsSchema, createRoom };
export type { CreateRoomParamsType };


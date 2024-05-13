"use client";

import { Loading } from "@/components/loading/Loading";
import { RoomProvider } from "@/liveblocks-configs/block-room.config";
import { ClientSideSuspense } from "@liveblocks/react";
import { useSession } from "next-auth/react";
import { useParams } from "next/navigation";

import * as z from "zod";
import { TextRoom } from "./TextRoom";

const NextAuthSchema = z.object({
  image: z.string().nullable(),
  name: z.string().nullable(),
  email: z.string(),
});


const RoomQuerySchema = z.object({
  id: z.string(),
});

type RoomQuery = z.infer<typeof RoomQuerySchema>;

const Page = () => {
  const { id } = useParams<RoomQuery>();

  const { data } = useSession();

  const result = NextAuthSchema.safeParse(data?.user);

  if (result.success) {
    return (
      <RoomProvider
        id={id}
        initialPresence={{
          cursor: null,

        }}
      >
        <ClientSideSuspense fallback={<Loading />}>
          {() => <TextRoom id={id} />}
        </ClientSideSuspense>
      </RoomProvider>
    );
  } else {
    return null;
  }
};

export default Page;

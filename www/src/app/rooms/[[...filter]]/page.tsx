"use client";
import Dashboard from "@/components/dashboard/Dashboard";
import { createRoom } from "@/lib/client/room/create";
import { getRooms } from "@/lib/client/room/get";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";

type RoomFilter = "active" | "draft" | "archived";

type QueryParams = {
  params: {
    filter: Array<RoomFilter>;
  };
};

const Page = ({ params }: QueryParams) => {
  // const rooms = await liveblocks.getRooms();

  const queryClient = useQueryClient();

  // Queries
  const roomInfoQuery = useQuery({
    queryKey: ["rooms"],
    queryFn: getRooms,
  });

  const createRoomMutation = useMutation({
    mutationFn: createRoom,
    onError: console.log,
    onSuccess: () => {
      queryClient.invalidateQueries({
        type: "all",
        queryKey: ["rooms"],
      });
    },
  });

  if (roomInfoQuery.isPending) return "Loading...";

  if (roomInfoQuery.error)
    return "An error has occurred: " + roomInfoQuery.error.message;

  if (roomInfoQuery.data) {
    return (
      <Dashboard
        rooms={roomInfoQuery.data}
        filters={params.filter}
        createRoom={createRoomMutation.mutateAsync}
      />
    );
  } else {
    return null;
  }
};

export default Page;

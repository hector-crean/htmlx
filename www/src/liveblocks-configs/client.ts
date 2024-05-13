"use client";
import { createClient, } from "@liveblocks/client";
import { createLiveblocksContext } from "@liveblocks/react";

export const ENDPOINT_BASE_URL = "/api/liveblocks" as const;





const client = createClient({
    authEndpoint: "/api/liveblocks/auth",
    async resolveUsers({ userIds }) {


        const searchParams = new URLSearchParams(
            userIds.map((userId) => ["userIds", userId])
        );

        const response = await fetch(`/api/users?${searchParams}`);

        if (!response.ok) {
            throw new Error("Problem resolving user");
        }

        const users = await response.json();

        return users;
    },
  // Get rooms' info from their ID
  resolveRoomsInfo: async ({ roomIds }) => {
    const searchParams = new URLSearchParams(
      roomIds.map((roomId) => ["roomIds", roomId])
    );
    const response = await fetch(`/api/rooms?${searchParams}`);

    if (!response.ok) {
      throw new Error("Problem resolving rooms info");
    }

    const roomsInfo = await response.json();
    return roomsInfo;
  },

  // Find a list of users that match the current search term
  resolveMentionSuggestions: async ({ text }) => {
    const response = await fetch(
      `/api/users/search?text=${encodeURIComponent(text)}`
    );

    if (!response.ok) {
      throw new Error("Problem resolving user");
    }

    const userIds = await response.json();
    return userIds;
  },

});


const { LiveblocksProvider, useInboxNotifications, useMarkAllInboxNotificationsAsRead, useMarkInboxNotificationAsRead, useUnreadInboxNotificationsCount,  } = createLiveblocksContext(client)



export { LiveblocksProvider, client, useInboxNotifications, useMarkAllInboxNotificationsAsRead, useMarkInboxNotificationAsRead, useUnreadInboxNotificationsCount };



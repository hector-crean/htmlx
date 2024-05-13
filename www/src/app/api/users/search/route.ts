import { getUsersFuzzy } from "@/lib/server/users/get";
import { NextRequest, NextResponse } from "next/server";

/**
 * Returns a list of user IDs from a partial search input
 * For `resolveMentionSuggestions` in liveblocks.config.ts
 */

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const text = searchParams.get("text") as string;

  const users = await getUsersFuzzy({text});

  return NextResponse.json(users.map((user) => user.id));
}
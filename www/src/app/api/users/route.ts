import { getUsers } from "@/lib/server/users/get";
import { NextRequest, NextResponse } from "next/server";

export async function GET(request: NextRequest) {
  const { searchParams } = new URL(request.url);
  const userIds = searchParams.getAll("userIds");

  if (!userIds || !Array.isArray(userIds)) {
    return new NextResponse("Missing or invalid userIds", { status: 400 });
  }

  const users = await getUsers({ userIds: userIds })

  return NextResponse.json(
    users,
    { status: 200 }
  );
}
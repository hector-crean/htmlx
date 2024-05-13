

import { liveblocks } from "@/app/api/liveblocks";
import { auth } from "@/auth";
import { prisma } from "@/prisma-client";
import { IUserInfo } from "@liveblocks/node";
import { Group, Organisation } from "@prisma/client";
import type { NextApiRequest, NextApiResponse } from "next";
import { User as NextAuthUser } from 'next-auth';


export type UserInfo = IUserInfo & {
  email: string;
  groups: Array<Group>,
  organisations: Array<Organisation>
}


export async function POST(req: NextApiRequest, res: NextApiResponse) {


  const DEFAULT_NEXT_AUTH_USER: NextAuthUser = {}

  const nextSession = await auth()
  const nextAuthUser: NextAuthUser = nextSession?.user ?? DEFAULT_NEXT_AUTH_USER


  const user = await prisma.user.findUnique({
    where: {
        email: nextAuthUser.email ?? 'anonymous@gmail.com'
    },
    include: {
      groups: true,
      organisations: true
    }
  });

  if(user){
      // Start an auth session inside your endpoint

     const userInfo: UserInfo = { 
      name: user.name ?? '',
      avatar: user.image ?? '',
      email: user.email,
      groups: user.groups,
      organisations: user.organisations
     }

  const session = liveblocks.prepareSession(
    user.email,
    {
      userInfo
    }
  );



  // Use a naming pattern to allow access to rooms with wildcards
  // Giving the user read access on their org, and write access on their group

  //<organization_id>:<group_id>:<document_id>
  // session.allow(`liveblocks:examples:*`, session.FULL_ACCESS);
  session.allow('*', session.FULL_ACCESS)
  // session.allow(`synapse:${user.group}:*`, session.FULL_ACCESS)
  // session.allow(`${user.organization}:*`, session.READ_ACCESS);
  // session.allow(`${user.organization}:${user.group}:*`, session.FULL_ACCESS);



  const { body, status } = await session.authorize();
  return new Response(body, { status });

  } else {


    return new Response(body, { status });
  }
 

  

  

}

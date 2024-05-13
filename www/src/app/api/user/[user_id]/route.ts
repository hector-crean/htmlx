import { prisma } from "@/prisma-client";
import { NextRequest } from "next/server";
import { z } from 'zod';

const AccessSchema = z.object({
    organisation_id: z.string(),
    group_id: z.string(),
    document_id: z.string(),
})

type Access = z.infer<typeof AccessSchema>;



type QueryUserParams = {
    params: {
        user_id: string
    }
}

export async function GET(request: NextRequest, { params }: QueryUserParams) {

    const result = await AccessSchema.safeParseAsync(request.body)

    if (result.success) {


        const user = await prisma.user.findUnique({
            where: {
                email: params.user_id
            }
        });

        return Response.json({ user })


    } else {
        return Response.json({ error: 'error' })
    }



}

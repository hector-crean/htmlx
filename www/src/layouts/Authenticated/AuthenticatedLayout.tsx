"use client";
import { Session } from "next-auth";
import { PropsWithChildren } from "react";

type Props = PropsWithChildren<{ session: Session | null }>;

export function AuthenticatedLayout({ children, session }: Props) {


  return <>{children}</>;
}

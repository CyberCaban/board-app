"use client";
import { UserBoardsProvider } from "@/providers/userBoardsProvider";

export default function BoardLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return <UserBoardsProvider>{children}</UserBoardsProvider>;
}

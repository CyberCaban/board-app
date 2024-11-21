"use client";

import { useUserBoardsStore } from "@/providers/userBoardsProvider";
import Link from "next/link";
import { useEffect, } from "react";

export default function AvailableBoards() {
  const ubstore = useUserBoardsStore((state) => state);

  useEffect(() => {
    ubstore.requestUserBoards().catch((e) => console.log(e.message));
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);
  return (
    <>
      <h1>Available Boards</h1>
      {ubstore.userBoards.map((board) => {
        return (
          <Link key={board.id} href={`/dashboard/${board.id}`}>
            <h3>{board.name}</h3>
          </Link>
        );
      })}
    </>
  );
}

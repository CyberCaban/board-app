"use client";
import { useUnauthorized } from "@/utils/hooks";
import AvailableBoards from "../_components/AvailableBoards";
import CreateBoardForm from "../_components/CreateBoardForm";

// TODO: redirect to sign in if not signed
export default function Board() {
  useUnauthorized();
  return (
    <>
      <main className="flex min-h-screen flex-col items-center p-24">
        <h1>Board</h1>
        <CreateBoardForm />
        <AvailableBoards />
      </main>
    </>
  );
}

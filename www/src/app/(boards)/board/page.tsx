"use client";
import AvailableBoards from "../_components/AvailableBoards";
import CreateBoardForm from "../_components/CreateBoardForm";

export default function Board() {
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
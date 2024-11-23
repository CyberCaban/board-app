"use client";
import { KanbanStoreProvider } from "@/providers/kanbanProvider";

export default function BoardsLayout({
  children,
  cardModal,
}: {
  children: React.ReactNode;
  cardModal: React.ReactNode;
}) {
  return (
    <main className="flex flex-col px-6">
      <KanbanStoreProvider>
        {cardModal}
        {children}
      </KanbanStoreProvider>
    </main>
  );
}

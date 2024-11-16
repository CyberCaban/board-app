import { KanbanStoreProvider } from "@/providers/kanbanProvider";
import { Suspense } from "react";

export default function BoardsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <main className="flex flex-col px-6">
      <KanbanStoreProvider>
        <Suspense fallback={<div>Loading...</div>}>{children}</Suspense>
      </KanbanStoreProvider>
    </main>
  );
}

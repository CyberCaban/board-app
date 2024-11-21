import { KanbanStoreProvider } from "@/providers/kanbanProvider";

export default function BoardsLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <main className="flex flex-col px-6">
      <KanbanStoreProvider>{children}</KanbanStoreProvider>
    </main>
  );
}

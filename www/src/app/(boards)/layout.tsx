import AuthGuardRedirect from "@/components/authGuards/AuthGuardRedirect";
import { KanbanStoreProvider } from "@/providers/kanbanProvider";

export default function BoardsLayout({
  children,
  cardModal,
}: {
  children: React.ReactNode;
  cardModal: React.ReactNode;
}) {
  return (
    <AuthGuardRedirect>
      <main className="flex flex-col px-6">
        <KanbanStoreProvider>
          {cardModal}
          {children}
        </KanbanStoreProvider>
      </main>
    </AuthGuardRedirect>
  );
}

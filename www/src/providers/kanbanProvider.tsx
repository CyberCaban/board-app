"use client";

import { createKanbanStore, initKanbanStore, KanbanStore } from "@/stores/kanbanStore";
import { createContext, useContext, useRef } from "react";
import { useStore } from "zustand";

export type KanbanStoreApi = ReturnType<typeof createKanbanStore>;

export const KanbanStoreContext = createContext<KanbanStoreApi | undefined>(
  undefined
);

export function KanbanStoreProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const storeRef = useRef<KanbanStoreApi>();

  if (!storeRef.current) {
    storeRef.current = createKanbanStore(initKanbanStore());
  }

  return (
    <KanbanStoreContext.Provider value={storeRef.current}>
      {children}
    </KanbanStoreContext.Provider>
  );
}

export const useKanbanStore = <T,>(
  selector: (state: KanbanStore) => T
): [T, KanbanStoreApi] => {
  const store = useContext(KanbanStoreContext);

  if (!store) {
    throw new Error("useKanbanStore must be used within a KanbanStoreProvider");
  }

  return [useStore(store, selector), store];
};

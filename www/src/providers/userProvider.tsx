"use client";

import { createUserStore, UserStore } from "@/stores/userStore";
import { createContext, useContext, useRef } from "react";
import { useStore } from "zustand";

export type UserStoreApi = ReturnType<typeof createUserStore>;

export const UserStoreContext = createContext<UserStoreApi | null>(null);

export function UserStoreProvider({ children }: { children: React.ReactNode }) {
  const storeRef = useRef<UserStoreApi | null>(null);

  if (!storeRef.current) {
    storeRef.current = createUserStore();
  }

  return (
    <UserStoreContext.Provider value={storeRef.current}>
      {children}
    </UserStoreContext.Provider>
  );
}

export const useUserStore = <T,>(selector: (store: UserStore) => T): T => {
  const store = useContext(UserStoreContext);

  if (!store) {
    throw new Error("useUserStore must be used within a UserStoreProvider");
  }

  return useStore(store, selector);
};

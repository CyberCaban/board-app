"use client";

import { createUserStore, initUserStore, UserStore } from "@/stores/userStore";
import { getData } from "@/utils/utils";
import { createContext, useContext, useEffect, useRef } from "react";
import { useStore } from "zustand";

export type UserStoreApi = ReturnType<typeof createUserStore>;

export const UserStoreContext = createContext<UserStoreApi | undefined>(
  undefined,
);

export function UserStoreProvider({ children }: { children: React.ReactNode }) {
  const storeRef = useRef<UserStoreApi>();

  if (!storeRef.current) {
    storeRef.current = createUserStore(initUserStore());
  }
  // reset user if not authorized
  getData("/api/user").catch(() => storeRef.current?.getState().resetUser());

  return (
    <UserStoreContext.Provider value={storeRef.current}>
      {children}
    </UserStoreContext.Provider>
  );
}

export const useUserStore = <T,>(
  selector: (store: UserStore) => T,
): [T, UserStoreApi] => {
  const store = useContext(UserStoreContext);

  if (!store) {
    throw new Error("useUserStore must be used within a UserStoreProvider");
  }
  // reset user if not authorized
  useEffect(() => {
    getData("/api/user").catch(() => store?.getState().resetUser());
  }, []);

  return [useStore(store, selector), store];
};

import {
  createUserBoardsStore,
  initUserBoardsStore,
  UserBoardsStore,
} from "@/stores/userBoardsStore";
import { createContext, useContext, useRef } from "react";
import { useStore } from "zustand";

export type UserBoardsStoreApi = ReturnType<typeof createUserBoardsStore>;

export const UserBoardsStoreContext = createContext<
  UserBoardsStoreApi | undefined
>(undefined);

export function UserBoardsProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const storeRef = useRef<UserBoardsStoreApi>();

  if (!storeRef.current) {
    storeRef.current = createUserBoardsStore(initUserBoardsStore());
  }

  return (
    <UserBoardsStoreContext.Provider value={storeRef.current}>
      {children}
    </UserBoardsStoreContext.Provider>
  );
}

export const useUserBoardsStore = <T,>(
  selector: (state: UserBoardsStore) => T,
) => {
  const store = useContext(UserBoardsStoreContext);
  if (!store) {
    throw new Error(
      "useUserBoardsStore must be used within a UserBoardsProvider",
    );
  }
  return useStore(store, selector);
};

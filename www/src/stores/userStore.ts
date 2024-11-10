import { createStore } from "zustand";
import { persist } from "zustand/middleware";

export type User = {
  token: string;
  username: string;
  profile_url: string;
};

export type UserActions = {
  setUser: (user: User) => void;
  resetUser: () => void;
};

export type UserStore = User & UserActions;

export const defaultUser: User = {
  token: "",
  username: "",
  profile_url: "",
};

export const createUserStore = (user: User = defaultUser) => {
  return createStore<UserStore>()(
    persist(
      (set) => ({
        ...user,
        setUser: (user: User) => set(user),
        resetUser: () => set(defaultUser),
      }),
      {
        name: "user",
      }
    )
  );
};

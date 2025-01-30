import { postData } from "@/utils/utils";
import { toast } from "sonner";
import { createStore } from "zustand";
import { devtools, persist, subscribeWithSelector } from "zustand/middleware";

export type TUser = {
  id: string;
  username: string;
  profile_url: string;
};

export type UserActions = {
  register: (username: string, email: string, password: string) => void;
  login: (username: string, password: string) => void;
  logout: () => void;
  setUser: (user: TUser) => void;
  resetUser: () => void;
};

export type UserStore = TUser & UserActions;

export const initUserStore = (): TUser => ({
  id: "",
  username: "",
  profile_url: "",
});

export const defaultUser: TUser = {
  id: "",
  username: "",
  profile_url: "",
};

export const createUserStore = (user: TUser = defaultUser) => {
  const store = createStore<UserStore>()(
    subscribeWithSelector(
      persist(
        devtools((set) => ({
          ...user,
          register: (username: string, email: string, password: string) => {
            postData("/api/register", { username, email, password })
              .then((res: TUser) => {
                set({
                  id: res.id,
                  username: res.username,
                  profile_url: res.profile_url,
                });
                toast.success("User registered successfully");
              })
              .catch((err) => toast.error(err.message));
          },
          login: (email: string, password: string) => {
            postData("/api/login", { email, password })
              .then((res: TUser) => {
                set({
                  id: res.id,
                  username: res.username,
                  profile_url: res.profile_url,
                });
                toast.success("User logged in successfully");
              })
              .catch((err) => toast.error(err.message));
          },
          logout: () => {
            postData("/api/logout", {}).then(() => {
              set(defaultUser);
              toast.success("User logged out");
            });
          },
          setUser: (user: TUser) => set(user),
          resetUser: () => set(defaultUser),
        })),
        {
          name: "user",
        },
      ),
    ),
  );

  store.subscribe(() => {});
  return store;
};

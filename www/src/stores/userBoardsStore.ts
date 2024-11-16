import { IBoard } from "@/types";
import { getData, postData } from "@/utils/utils";
import { createStore } from "zustand";
import { devtools, persist, subscribeWithSelector } from "zustand/middleware";

export type TUserBoards = { userBoards: Array<{ id: string; name: string }> };

export type UserBoardsActions = {
  requestUserBoards: () => Promise<void>;
  addUserBoard: (name: string) => Promise<void>;
  resetUserBoards: () => void;
};

export type UserBoardsStore = UserBoardsActions & TUserBoards;

export const defaultUserBoardsStore = {
  userBoards: [],
};

export const initUserBoardsStore = (): TUserBoards => ({
  userBoards: [],
});

export const createUserBoardsStore = (
  userBoards: TUserBoards = defaultUserBoardsStore,
) => {
  const store = createStore<UserBoardsStore>()(
    subscribeWithSelector(
      persist(
        devtools((set) => ({
          ...userBoards,
          requestUserBoards: () =>
            getData("/boards").then(
              (res: Array<{ id: string; name: string }>) => {
                set({
                  userBoards: res,
                });
              },
            ),
          addUserBoard: (name: string) =>
            postData("/boards", { name }).then((res) => {
              getData(`/boards/${res}`).then(({ id, name }: IBoard) => {
                set((prev) => ({
                  ...prev,
                  userBoards: [...prev.userBoards, { id, name }],
                }));
              });
            }),
          resetUserBoards: () => set({ ...defaultUserBoardsStore }),
        })),
        { name: "userBoards" },
      ),
    ),
  );

  return store;
};

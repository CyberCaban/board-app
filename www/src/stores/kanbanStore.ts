import { IBoard, IBoardCard, ICard } from "@/types";
import { deleteData, getData, postData, putData } from "@/utils/utils";
import { toast } from "sonner";
import { createStore } from "zustand";
import { devtools, persist, subscribeWithSelector } from "zustand/middleware";

export type TKanban = IBoard & { cardModal?: ICard };

export type KanbanActions = {
  requestBoard: (id: string) => Promise<void>;
  getBoard: () => void;
  update: (kanban: TKanban) => void;
  reset: () => void;
  addColumn: (name: string, position: number) => void;
  deleteColumn: (id: string) => void;
  updateColumn: (id: string, name: string, position: number) => void;
  requestCardModal: (board_id: string, id: string) => Promise<void>;
  resetCardModal: () => void;
  addCard: (name: string, column_id: string, position: number) => void;
  deleteCard: (id: string, column_id: string) => void;
  updateCard: (
    id: string,
    name: string,
    cover_attachment: string,
    description: string,
    column_id: string,
    attachments: string[],
  ) => void;
  swapCards: (id1: string, id2: string, column_id: string) => void;
  reorderList: (
    fromColumn: string,
    toColumn: string,
    toPos: number,
    cardId: string,
  ) => void;
};

export type KanbanStore = TKanban & KanbanActions;

export const initKanbanStore = (): TKanban => ({
  id: "",
  name: "",
  columns: [],
  cards: [],
  cardModal: undefined,
});

export const defaultKanbanStore = initKanbanStore();

export const createKanbanStore = (board: TKanban = defaultKanbanStore) => {
  const store = createStore<KanbanStore>()(
    subscribeWithSelector(
      persist(
        devtools((set, get) => ({
          ...board,
          getBoard: () => board,
          requestBoard: (id: string) => getData(`/boards/${id}`).then(set),
          update: (kanban: TKanban) => set({ ...kanban }),
          reset: () => set(defaultKanbanStore),
          addColumn: (name: string, position: number) => {
            postData(`/boards/${get().id}/columns`, { name, position })
              .then((res) => {
                getData(`/boards/${get().id}/columns/${res}`)
                  .then((res) => {
                    set((prev) => ({
                      ...prev,
                      columns: [...prev.columns, res],
                    }));
                  })
                  .catch((e) => toast.error(e.message));
              })
              .catch((e) => toast.error(e.message));
          },
          deleteColumn: (id: string) => {
            deleteData(`/boards/${get().id}/columns/${id}`)
              .then((res) => {
                console.log(res);
                set((prev) => ({
                  ...prev,
                  columns: prev.columns.filter((column) => column.id !== id),
                  cards: prev.cards.filter((card) => card.column_id !== id),
                }));
              })
              .catch((e) => toast.error(e.message));
          },
          updateColumn: (id: string, name: string, position: number) => {
            postData(`/boards/${get().id}/columns/${id}`, {
              name,
              position,
            })
              .then(() => {
                getData(`/boards/${get().id}/columns/${id}`)
                  .then((res) => {
                    set((prev) => ({
                      ...prev,
                      columns: prev.columns.map((column) => {
                        if (column.id === id) {
                          return res;
                        }
                        return column;
                      }),
                    }));
                  })
                  .catch((e) => toast.error(e.message));
              })
              .catch((e) => toast.error(e.message));
          },
          requestCardModal: (board_id: string, id: string) =>
            getData(`/boards/${board_id}/cards/${id}`).then((res) => {
              set((prev) => ({
                ...prev,
                cardModal: res,
              }));
            }),
          resetCardModal: () => set({ cardModal: undefined }),
          addCard: (name: string, column_id: string, position: number) => {
            postData(`/boards/${get().id}/columns/${column_id}/cards`, {
              name,
              description: "",
              column_id,
              position,
            })
              .then((res: IBoardCard) => {
                getData(
                  `/boards/${get().id}/columns/${column_id}/cards/${res.id}`,
                )
                  .then((res) => {
                    set((prev) => ({
                      ...prev,
                      cards: [...prev.cards, res],
                    }));
                  })
                  .catch((e) => toast.error(e.message));
              })
              .catch((e) => toast.error(e.message));
          },
          deleteCard: (id: string, column_id: string) => {
            deleteData(`/boards/${get().id}/columns/${column_id}/cards/${id}`)
              .then(() => {
                set((prev) => ({
                  ...prev,
                  cards: prev.cards.filter((card) => card.id !== id),
                }));
              })
              .catch((e) => toast.error(e.message));
          },
          updateCard: (
            id: string,
            name: string,
            cover_attachment: string,
            description: string,
            column_id: string,
            // attachments: string[],
          ) => {
            putData(`/boards/${get().id}/columns/${column_id}/cards/${id}`, {
              name,
              description,
              cover_attachment,
            })
              .then(() => {
                getData(`/boards/${get().id}/columns/${column_id}/cards/${id}`)
                  .then((res) => {
                    set((prev) => ({
                      ...prev,
                      cards: prev.cards.map((card) => {
                        if (card.id === id) {
                          return res;
                        }
                        return card;
                      }),
                      cardModal: res,
                    }));
                  })
                  .catch((e) => toast.error(e.message));
              })
              .catch((e) => toast.error(e.message));
          },
          swapCards: (id1, id2, column_id) => {
            putData(
              `/boards/${get().id}/columns/${column_id}/cards/${id1}/${id2}`,
              {},
            )
              .then(() => {
                getData(`/boards/${get().id}/columns/${column_id}/cards`)
                  .then((res) => {
                    set((prev) => ({
                      ...prev,
                      cards: prev.cards.map((card) => {
                        if (card.id === id1) {
                          return res.find(
                            (card: IBoardCard) => card.id === id2,
                          )!;
                        }
                        if (card.id === id2) {
                          return res.find(
                            (card: IBoardCard) => card.id === id1,
                          )!;
                        }
                        return card;
                      }),
                    }));
                  })
                  .catch((e) => toast.error(e.message));
              })
              .catch((e) => toast.error(e.message));
          },
          reorderList: (fromColumn, toColumn, toPos, cardId) => {
            putData(
              `/boards/${get().id}/columns/${fromColumn}/cards/${cardId}/reorder/${toColumn}/${toPos}`,
              {},
            )
              .then(() => get().requestBoard(get().id))
              .catch((e) => toast.error(e.message));
          },
        })),
        { name: "kanbanStore" },
      ),
    ),
  );
  return store;
};

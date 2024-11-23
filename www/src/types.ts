export interface IFile {
  id: string;
  name: string;
  private: boolean;
  user_id: string;
}

export interface IBoardColumn {
  id: string;
  name: string;
  position: number;
}

export interface IBoardCard {
  id: string;
  name: string;
  column_id: string;
  position: number;
}

export interface IBoard {
  id: string;
  name: string;
  columns: IBoardColumn[];
  cards: IBoardCard[];
}

export type ApiError = "Failed to parse UUID" | "Unauthorized";
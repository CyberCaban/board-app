// Type for describing file row from backend
export interface IFile {
  id: string;
  name: string;
  private: boolean;
  user_id: string;
}

// Interface for describing object that get passed to ImagesMasonry
export interface IFileView {
  url: string;
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
  cover_attachment: string;
}

export interface ICard {
  id: string;
  name: string;
  cover_attachment: string;
  description: string;
  column_id: string;
  position: number;
  attachments: ICardAttachment[];
}

export interface ICardAttachment {
  id: string;
  url: string;
}

export interface IBoard {
  id: string;
  name: string;
  columns: IBoardColumn[];
  cards: IBoardCard[];
}

export type ApiError = "Failed to parse UUID" | "Unauthorized";

export interface IPubUser {
  id: string;
  username: string;
  bio: string;
  profile_url: string;
}

export interface IMessage {
  id: string;
  content: string;
  sender_id: string;
  conversation_id: string;
  created_at: number;
}

export interface IConversation {
  id: string;
  member_one: string;
  member_two: string;
}

export interface IMember {
  id: string;
  bio: string;
  username: string;
  profile_url: string;
}

export interface SearchState {
  query: string;
  isOpen: boolean;
  results: number[];
  currentIndex: number;
}

import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";
import type { IMember, IMessage } from "@/types";

interface ChatMessageProps {
  message: IMessage;
  isSearchResult: boolean;
  isCurrentSearchResult: boolean;
  searchQuery: string;
  index: number;
  members: IMember[];
}

export function ChatMessage({
  message,
  isSearchResult,
  isCurrentSearchResult,
  searchQuery,
  index,
  members,
}: ChatMessageProps) {
  // Функция для выделения найденного текста
  const highlightText = (text: string) => {
    if (!searchQuery.trim() || !isSearchResult) return text;

    const parts = text.split(new RegExp(`(${searchQuery})`, "gi"));
    return parts.map((part, i) =>
      part.toLowerCase() === searchQuery.toLowerCase() ? (
        <span key={i} className="rounded bg-yellow-500 px-1 text-black">
          {part}
        </span>
      ) : (
        part
      ),
    );
  };

  const sender = members.find((m) => m.id === message.sender_id);

  return (
    <div
      id={`message-${index}`}
      className={`flex items-start space-x-3 ${
        isCurrentSearchResult
          ? "-mx-4 rounded bg-gray-100 px-4 py-2 dark:bg-gray-800"
          : ""
      }`}
    >
      <Avatar className="mt-1 h-10 w-10">
        <AvatarImage src={sender?.profile_url} alt={message.sender_id} />
        <AvatarFallback>
          {message.sender_id.charAt(0).toUpperCase()}{" "}
        </AvatarFallback>
      </Avatar>
      <div className="flex-1">
        <div className="flex items-baseline">
          <span className="text-sm text-gray-400">{sender?.username}</span>
          <span className="ml-auto text-xs text-gray-500">
            {new Date(message.created_at).toLocaleTimeString()}
          </span>
        </div>
        <p className="font-inter mt-1 text-sm leading-relaxed">
          {highlightText(message.content)}
        </p>
        {/* {message.image && (
          <div className="mt-2">
            <img
              src={message.image || "/placeholder.svg"}
              alt="Uploaded"
              className="max-w-xs rounded-md"
            />
          </div>
        )} */}
      </div>
    </div>
  );
}

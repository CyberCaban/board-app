"use client";

import { useRef, useEffect } from "react";
import { ChatMessage } from "./ChatMessage";
import type { IMember, IMessage, SearchState } from "@/types";

interface MessageListProps {
  messages: IMessage[];
  searchState: SearchState;
  members: IMember[]
}

export function MessageList({ messages, searchState, members }: MessageListProps) {
  const messagesEndRef = useRef<HTMLDivElement>(null);
  const { query, results, currentIndex } = searchState;

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({ behavior: "smooth" });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

  return (
    <div className="chat-scrollbar max-h-[calc(100vh-14rem)] flex-1 space-y-4 overflow-y-auto p-4">
      {messages.map((message, index) => {
        const isSearchResult = results.includes(index);
        const isCurrentSearchResult =
          isSearchResult && results[currentIndex] === index;

        return (
          <ChatMessage
            key={message.id}
            message={message}
            isSearchResult={isSearchResult}
            isCurrentSearchResult={isCurrentSearchResult}
            searchQuery={query}
            index={index}
            members={members}
          />
        );
      })}
      <div ref={messagesEndRef} />
    </div>
  );
}

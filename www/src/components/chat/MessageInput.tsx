"use client";

import { useState, type KeyboardEvent } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Send, Upload } from "lucide-react";

interface MessageInputProps {
  onSendMessage: (content: string) => void;
  onOpenUpload: () => void;
}

export function MessageInput({
  onSendMessage,
  onOpenUpload,
}: MessageInputProps) {
  const [newMessage, setNewMessage] = useState("");

  const handleSendMessage = () => {
    if (newMessage.trim() === "") return;
    onSendMessage(newMessage);
    setNewMessage("");
  };

  const handleKeyDown = (e: KeyboardEvent<HTMLInputElement>) => {
    if (e.key === "Enter" && !e.shiftKey) {
      e.preventDefault();
      handleSendMessage();
    }
  };

  return (
    <div className="border-t border-gray-200 p-4 dark:border-gray-800">
      <div className="flex items-center">
        <Button
          variant="ghost"
          className="mr-2 bg-foreground"
          title="Upload file"
          onClick={onOpenUpload}
        >
          <Upload className="h-4 w-4 text-background" />
        </Button>
        <Input
          value={newMessage}
          onChange={(e) => setNewMessage(e.target.value)}
          onKeyDown={handleKeyDown}
          placeholder="Введите сообщение..."
          className="flex-1 border-gray-300 bg-background text-foreground focus:border-gray-400 dark:border-gray-700 dark:focus:border-gray-600"
        />
        <Button
          onClick={handleSendMessage}
          className="ml-2 bg-foreground hover:bg-foreground/90"
          disabled={!newMessage.trim()}
        >
          <Send className="h-4 w-4" />
          <span className="ml-2">send</span>
        </Button>
      </div>
    </div>
  );
}

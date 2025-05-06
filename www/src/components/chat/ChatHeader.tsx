"use client";

import { Button } from "@/components/ui/button";

interface ChatHeaderProps {
  onToggleSearch: () => void;
  isSearchOpen: boolean;
}

export function ChatHeader({ onToggleSearch, isSearchOpen }: ChatHeaderProps) {
  return (
    <header className="flex items-center border-b border-gray-200 bg-background p-4 dark:border-gray-800">
      <div className="ml-auto flex items-center space-x-2">
        <Button
          variant="ghost"
          size="sm"
          onClick={onToggleSearch}
          className={isSearchOpen ? "bg-gray-100" : "bg-background"}
        >
          {/* <Search className="h-4 w-4" /> */}
        </Button>
      </div>
    </header>
  );
}

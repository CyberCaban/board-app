"use client";

import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Search, ChevronUp, ChevronDown, X } from "lucide-react";
import { SearchState } from "@/types";

interface SearchPanelProps {
  searchState: SearchState;
  setSearchQuery: (query: string) => void;
  handleSearch: () => void;
  navigateSearch: (direction: "next" | "prev") => void;
  clearSearch: () => void;
}

export function SearchPanel({
  searchState,
  setSearchQuery,
  handleSearch,
  navigateSearch,
  clearSearch,
}: SearchPanelProps) {
  const { query, results, currentIndex } = searchState;

  return (
    <div className="flex items-center border-b border-gray-200 bg-gray-100 p-2 dark:border-gray-700 dark:bg-gray-800">
      <div className="flex flex-1 items-center space-x-2">
        <Input
          value={query}
          onChange={(e) => setSearchQuery(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              e.preventDefault();
              handleSearch();
            } else if (e.key === "Escape") {
              clearSearch();
            }
          }}
          placeholder="Поиск в сообщениях..."
          className="h-8 border-gray-300 bg-white dark:border-gray-700 dark:bg-gray-900"
          autoFocus
        />
        <Button
          variant="ghost"
          size="sm"
          onClick={handleSearch}
          disabled={!query.trim()}
        >
          <Search className="h-4 w-4" />
        </Button>
        <span className="text-xs text-gray-400">
          {results.length > 0
            ? `${currentIndex + 1} из ${results.length}`
            : query.trim()
              ? "Ничего не найдено"
              : ""}
        </span>
      </div>
      <div className="flex items-center space-x-1">
        <Button
          variant="ghost"
          size="sm"
          onClick={() => navigateSearch("prev")}
          disabled={results.length <= 1}
        >
          <ChevronUp className="h-4 w-4" />
        </Button>
        <Button
          variant="ghost"
          size="sm"
          onClick={() => navigateSearch("next")}
          disabled={results.length <= 1}
        >
          <ChevronDown className="h-4 w-4" />
        </Button>
        <Button variant="ghost" size="sm" onClick={clearSearch}>
          <X className="h-4 w-4" />
        </Button>
      </div>
    </div>
  );
}

"use client";
import { Button } from "@/components/ui/button";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { useKanbanStore } from "@/providers/kanbanProvider";
import React, { useEffect, useRef, useState } from "react";

interface KanbanColumnProps extends React.HTMLAttributes<HTMLLIElement> {
  id: string;
  title: string;
  children: React.ReactNode;
}

export default function KanbanColumn({
  id,
  title,
  children,
  ...props
}: KanbanColumnProps) {
  const [isDanglingCard, setIsDanglingCard] = useState(false);
  const cardInputRef = useRef<HTMLInputElement>(null);

  const [kstore] = useKanbanStore((state) => state);

  useEffect(() => {
    if (cardInputRef.current) cardInputRef.current.focus();
  }, [isDanglingCard]);

  const handleAddCard = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const cardName = cardInputRef.current?.value;
    if (!cardName) return;
    const cardsCountInColumn = kstore.cards
      .filter((card) => card.column_id === id)
      .toSorted((a, b) => b.position - a.position)[0]?.position;

    kstore.addCard(
      cardName,
      id,
      cardsCountInColumn !== undefined ? cardsCountInColumn + 1 : 0,
    );
    cardInputRef.current.value = "";
    setIsDanglingCard(true);
  };

  return (
    <li
      className="flex max-h-full min-w-[300px] max-w-[300px] flex-col gap-2 overflow-x-hidden overflow-y-scroll rounded-xl p-4"
      {...props}
    >
      <div className="flex items-center justify-between">
        <h2 className="text-xl">{title}</h2>
        <DropdownMenu>
          <DropdownMenuTrigger
            key={id}
            asChild
            className="ml-2 h-auto w-auto self-start bg-transparent px-2 py-1 font-bold"
          >
            ...
          </DropdownMenuTrigger>
          <DropdownMenuContent
            className="cursor-pointer rounded-md border border-neutral-600 bg-black px-2 py-1 font-semibold text-white"
            onClick={() => kstore.deleteColumn(id)}
          >
            Delete
          </DropdownMenuContent>
        </DropdownMenu>
      </div>
      {children}
      {isDanglingCard ? (
        <form onSubmit={handleAddCard}>
          <Input
            type="text"
            className="mt-4"
            placeholder="Enter card name"
            onBlur={() => setIsDanglingCard(false)}
            ref={cardInputRef}
          />
        </form>
      ) : (
        <Button className="mt-4" onClick={() => setIsDanglingCard(true)}>
          + Add Card
        </Button>
      )}
    </li>
  );
}

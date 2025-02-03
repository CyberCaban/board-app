"use client";
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { cn } from "@/lib/utils";
import { useKanbanStore } from "@/providers/kanbanProvider";
import { IBoardCard } from "@/types";
import { isImage } from "@/utils/utils";
import Image from "next/image";
import Link from "next/link";
import { useState } from "react";

interface KanbanCardProps extends React.HTMLAttributes<HTMLAnchorElement> {
  card: IBoardCard;
  className?: string;
}

export default function KanbanCard({
  card,
  className,
  ...props
}: KanbanCardProps) {
  const [kstore] = useKanbanStore((state) => state);
  const [open, setOpen] = useState(false);
  const onDelete = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    // if (document.startViewTransition) document.startViewTransition();
    kstore.deleteCard(card.id, card.column_id);
    setOpen(false);
  };
  return (
    <Link
      href={`/dashboard/${kstore.id}/card/${card.id}`}
      className={cn(
        "kanban-card flex w-full cursor-grab touch-none flex-row justify-between gap-2 rounded-lg border border-neutral-300 bg-neutral-200 px-4 py-2 text-neutral-800 shadow-md transition-all duration-200 ease-in-out hover:bg-neutral-300 hover:shadow-lg active:cursor-grabbing",
        className,
      )}
      {...props}
    >
      <div className="mr-4 flex flex-col overflow-hidden">
        {card.cover_attachment && isImage(card.cover_attachment) && (
          <Image
            src={`/uploads/${card.cover_attachment}`}
            alt={card.name}
            width={400}
            height={200}
            className="h-32 w-full rounded-t-lg object-cover"
          />
        )}
        <h3 className="mt-2 w-full truncate text-center text-lg font-bold">
          {card.name}
        </h3>
      </div>
      <DropdownMenu open={open} onOpenChange={setOpen}>
        <DropdownMenuTrigger className="ml-2 h-auto w-auto self-start bg-transparent px-2 py-1 font-bold">
          ...
        </DropdownMenuTrigger>
        <DropdownMenuContent
          className="cursor-pointer rounded-md border border-neutral-300 bg-neutral-200 px-2 py-1 font-semibold text-neutral-800 transition-colors hover:bg-neutral-300"
          onClick={onDelete}
          onSelect={(e) => {
            e.preventDefault();
            setOpen(false);
          }}
        >
          Delete
        </DropdownMenuContent>
      </DropdownMenu>
    </Link>
  );
}

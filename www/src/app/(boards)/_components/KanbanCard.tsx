"use client";
import {
  DropdownMenu,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { cn } from "@/lib/utils";
import { useKanbanStore } from "@/providers/kanbanProvider";
import { IBoardCard } from "@/types";
import { DropdownMenuContent } from "@radix-ui/react-dropdown-menu";
import Link from "next/link";

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
  const onDelete = (e: React.MouseEvent) => {
    e.preventDefault();
    e.stopPropagation();
    document.startViewTransition();
    kstore.deleteCard(card.id, card.column_id);
  };
  return (
    <Link
      href={`/dashboard/${kstore.id}/card/${card.id}`}
      className={cn(
        "group flex w-full cursor-pointer flex-row items-center justify-between rounded-md bg-neutral-400 px-4 py-2 text-black hover:bg-neutral-300",
        className,
      )}
      {...props}
    >
      <p className="line-clamp-3 overflow-x-auto break-words">{card.name}</p>
      <DropdownMenu>
        <DropdownMenuTrigger className="ml-2 h-auto w-auto self-start bg-transparent px-2 py-1 font-bold">
          ...
        </DropdownMenuTrigger>
        <DropdownMenuContent
          className="rounded-md border border-neutral-600 bg-black px-2 py-1 font-semibold text-white"
          onClick={onDelete}
        >
          Delete
        </DropdownMenuContent>
      </DropdownMenu>
    </Link>
  );
}

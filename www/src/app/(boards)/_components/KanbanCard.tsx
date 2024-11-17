"use client";
import {
  DropdownMenu,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { useKanbanStore } from "@/providers/kanbanProvider";
import { IBoardCard } from "@/types";
import { DropdownMenuContent } from "@radix-ui/react-dropdown-menu";

interface KanbanCardProps extends React.HTMLAttributes<HTMLDivElement> {
  card: IBoardCard;
}
export default function KanbanCard({ card, ...props }: KanbanCardProps) {
  const [kstore] = useKanbanStore((state) => state);
  const onDelete = () => {
    document.startViewTransition();
    kstore.deleteCard(card.id, card.column_id);
  };
  return (
    <div
      className="flex w-full cursor-pointer flex-row items-center justify-between rounded-md bg-neutral-400 px-4 py-2 text-black hover:bg-neutral-300"
      {...props}
    >
      <p className="break-words overflow-x-auto line-clamp-3">{card.description}</p>
      <DropdownMenu>
        <DropdownMenuTrigger className="h-auto w-auto bg-transparent px-2 py-1 font-bold self-start ml-2">
          ...
        </DropdownMenuTrigger>
        <DropdownMenuContent
          className="rounded-md border border-neutral-600 bg-black px-2 py-1 font-semibold text-white"
          onClick={onDelete}
        >
          Delete
        </DropdownMenuContent>
      </DropdownMenu>
    </div>
  );
}

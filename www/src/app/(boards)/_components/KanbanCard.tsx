"use client";
import {
  DropdownMenu,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import { cn } from "@/lib/utils";
import { useKanbanStore } from "@/providers/kanbanProvider";
import { IBoardCard } from "@/types";
import { isImage } from "@/utils/utils";
import { DropdownMenuContent } from "@radix-ui/react-dropdown-menu";
import Image from "next/image";
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
    if (document.startViewTransition) document.startViewTransition();
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
      <div className="flex flex-col">
        {card.cover_attachment && isImage(card.cover_attachment) && (
          <div className="mb-2 self-center bg-cover">
            <Image
              src={`/uploads/${card.cover_attachment}`}
              alt=""
              width={100}
              height={50}
              className="rounded-md"
            />
          </div>
        )}
        <p className="line-clamp-3 overflow-x-auto break-words">{card.name}</p>
      </div>
      <div className="relative right-2 top-0">
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
      </div>
    </Link>
  );
}

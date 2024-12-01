import {
  DropdownMenu,
  DropdownMenuTrigger,
} from "@/components/ui/dropdown-menu";
import clsx from "clsx";
import KanbanCard from "./KanbanCard";
import { IBoardCard } from "@/types";

export default function DropZone({
  pos,
  dragged,
  dropZone,
  column_id,
  dropColumn,
  card,
}: {
  pos: number;
  dragged: string | null;
  dropZone: number | null;
  column_id: string;
  dropColumn: string | null;
  card?: IBoardCard;
}) {
  const isVisible =
    dragged !== null && dropZone === pos && column_id === dropColumn;
  const isHidden =
    (dragged === null && dropZone !== pos) || column_id !== dropColumn;

  return (
    <div
      className={clsx([
        "drop-zone flex w-full flex-row items-center justify-between px-4 duration-75",
        `column-${column_id}`,
        {
          hidden: isHidden,
          "mb-2 rounded-md bg-neutral-800 py-2": isVisible,
        },
      ])}
    >
      {isVisible && card && <KanbanCard card={card} className="w-full" />}
      {isVisible && (
        <DropdownMenu>
          <DropdownMenuTrigger className="ml-2 h-auto w-auto self-start bg-transparent px-2 py-1 font-bold">
            &nbsp;
          </DropdownMenuTrigger>
        </DropdownMenu>
      )}
    </div>
  );
}

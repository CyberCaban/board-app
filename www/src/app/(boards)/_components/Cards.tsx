import { IBoardCard } from "@/types";
import KanbanCard from "./KanbanCard";
import { useRef } from "react";
import { useKanbanStore } from "@/providers/kanbanProvider";

interface ICard {
  cards: IBoardCard[];
  column_id: string;
}
export default function Cards({ cards, column_id }: ICard) {
  const [kstore] = useKanbanStore((state) => state);
  const dragItem = useRef<IBoardCard | null>(null);
  const dragOverItem = useRef<IBoardCard | null>(null);
  const onDragStart = (
    event: React.DragEvent<HTMLDivElement>,
    card: IBoardCard,
  ) => {
    console.log("start", card);
    dragItem.current = card;
  };

  const onDragEnter = (
    event: React.DragEvent<HTMLDivElement>,
    card: IBoardCard,
  ) => {
    console.log("enter", card);
    dragOverItem.current = card;
  };
  const onDragEnd = (e: React.DragEvent<HTMLDivElement>, card: IBoardCard) => {
    e.preventDefault();
    console.log("end", card, dragOverItem.current);
    if (
      !dragOverItem.current ||
      !dragItem.current ||
      dragOverItem.current.column_id !== dragItem.current.column_id ||
      dragItem.current.id === dragOverItem.current.id
    )
      return;
    kstore.swapCards(dragItem.current.id, dragOverItem.current.id, column_id);
    document.startViewTransition();
    dragItem.current = null;
    dragOverItem.current = null;
  };
  // const onDragLeave = (
  //   event: React.DragEvent<HTMLDivElement>,
  //   card: IBoardCard,
  // ) => {
  //   event.preventDefault();
  //   console.log("leave", card.id);
  //   if (dragOverItem.current) dragOverItem.current = null;
  // };
  return (
    <div className="flex flex-col gap-2">
      {cards
        .filter((card) => card.column_id === column_id)
        .toSorted((a, b) => a.position - b.position)
        .map((card) => (
          <KanbanCard
            key={card.id}
            card={card}
            draggable
            onDragStart={(e) => onDragStart(e, card)}
            onDragOver={(e) => onDragEnter(e, card)}
            // onDragLeave={(e) => onDragLeave(e, card)}
            onDragEnd={(e) => onDragEnd(e, card)}
          />
        ))}
    </div>
  );
}

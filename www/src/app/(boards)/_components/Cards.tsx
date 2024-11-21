import { IBoardCard } from "@/types";
import KanbanCard from "./KanbanCard";
import { MutableRefObject } from "react";
import DropZone from "./DropZone";

interface ICard {
  cards: IBoardCard[];
  column_id: string;
  dragItem: MutableRefObject<HTMLDivElement | null>;
  dragOverItem: MutableRefObject<HTMLDivElement | null>;
  dragged: string | null;
  setDragged: React.Dispatch<React.SetStateAction<string | null>>;
  dropZone: number | null;
  dropColumn: string | null;
}
export default function Cards({
  cards,
  column_id,
  dragged,
  setDragged,
  dropZone,
  dropColumn,
}: ICard) {
  const columnCards = cards
    .filter((card) => card.column_id === column_id)
    .toSorted((a, b) => a.position - b.position);
  return (
    <div className="flex flex-col gap-2">
      {cards
        .filter((card) => card.column_id === column_id)
        .toSorted((a, b) => a.position - b.position)
        .map((card) => (
          <div className="flex flex-col" key={card.id}>
            <DropZone
              pos={card.position}
              dragged={dragged}
              dropZone={dropZone}
              column_id={column_id}
              dropColumn={dropColumn}
            />
            <>
              {dragged !== card.id && (
                <KanbanCard
                  card={card}
                  data-id={card.id}
                  data-position={card.position}
                  data-column-id={card.column_id}
                  onDragStart={(e) => {
                    e.preventDefault();
                    setDragged(card.id);
                  }}
                  draggable
                />
              )}
            </>
          </div>
        ))}
      <DropZone
        pos={columnCards.length}
        dragged={dragged}
        dropZone={dropZone}
        column_id={column_id}
        dropColumn={dropColumn}
      />
    </div>
  );
}

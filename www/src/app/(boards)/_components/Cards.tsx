import { IBoardCard } from "@/types";
import KanbanCard from "./KanbanCard";
import DropZone from "./DropZone";

interface ICard {
  cards: IBoardCard[];
  column_id: string;
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

  const draggedCard = dragged
    ? cards.find((card) => card.id === dragged)
    : undefined;

  return (
    <div className="flex max-w-full flex-col gap-2 overflow-y-scroll">
      {columnCards.map((card) => (
        <div className="flex flex-col" key={card.position}>
          <DropZone
            pos={card.position}
            dragged={dragged}
            dropZone={dropZone}
            column_id={column_id}
            dropColumn={dropColumn}
            card={dragged ? draggedCard : undefined}
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
        card={dragged ? draggedCard : undefined}
      />
    </div>
  );
}

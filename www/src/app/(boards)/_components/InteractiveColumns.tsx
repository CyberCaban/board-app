"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import KanbanColumn from "./KanbanColumn";
import Cards from "./Cards";
import KanbanCard from "./KanbanCard";
import { useKanbanStore } from "@/providers/kanbanProvider";

// interface InteractiveColumnsProps {}
export default function InteractiveColumns() {
  const dragItem = useRef<HTMLDivElement | null>(null);
  const dragOverItem = useRef<HTMLDivElement | null>(null);
  const [kstore] = useKanbanStore((state) => state);

  const [dragged, setDragged] = useState<string | null>(null);
  const [mousePos, setMousePos] = useState({ x: 0, y: 0 });
  const [dropZone, setDropZone] = useState<number | null>(null);
  const [dropColumn, setDropColumn] = useState<string | null>(null);
  const dCard = useMemo(
    () => kstore.cards.find((c) => c.id === dragged),
    [kstore.cards, dragged],
  );

  useEffect(() => {
    const mouseHandler = (e: MouseEvent) => {
      setMousePos({ x: e.x - 20, y: e.y - 20 });
    };
    document.addEventListener("mousemove", mouseHandler);
    return () => document.removeEventListener("mousemove", mouseHandler);
  }, []);
  useEffect(() => {
    if (dragged === null) return;
    const elems = document.querySelectorAll(
      `.drop-zone` + `.column-${dropColumn}`,
    );

    const positions = Array.from(elems).map(
      (e) => e.getBoundingClientRect().top,
    );
    const diffs = positions.map((p) => Math.abs(p - mousePos.y));
    const minDiff = Math.min(...diffs);
    let index = diffs.indexOf(minDiff);

    if (dCard!.column_id === dropColumn && index - dCard!.position === 1) {
      index -= 1;
    }
    // const newPosition =
    //   dCard!.column_id === dropColumn
    //     ? index <= dCard!.position
    //       ? index
    //       : index - 1
    //     : index;
    // console.log(newPosition);

    setDropZone(index);
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [dragged, mousePos]);
 
  return (
    <>
      {dragged !== null && (
        <div
          className="absolute w-[300px]"
          style={{ left: mousePos.x, top: mousePos.y }}
        >
          <KanbanCard
            card={dCard!}
            onMouseUp={(e) => {
              e.preventDefault();
              if (dropZone === null || dropColumn === null) return;
              const newPosition =
                dCard!.column_id === dropColumn
                  ? dropZone <= dCard!.position
                    ? dropZone
                    : dropZone - 1
                  : dropZone;
              kstore.reorderList(
                dCard!.column_id,
                dropColumn!,
                newPosition,
                dCard!.id,
              );
              // console.log(
              //   "reorder",
              //   dCard!.position,
              //   newPosition,
              //   "diff columns:",
              //   dCard!.column_id !== dropColumn,
              // );
              setDropZone(null);
              setDropColumn(null);
              setDragged(null);
            }}
          />
        </div>
      )}
      {kstore.columns.map((col) => (
        <KanbanColumn
          key={col.id}
          id={col.id}
          title={col.name}
          onMouseEnter={() => setDropColumn(col.id)}
        >
          <Cards
            cards={kstore.cards}
            column_id={col.id}
            dragItem={dragItem}
            dragOverItem={dragOverItem}
            dragged={dragged}
            setDragged={setDragged}
            dropZone={dropZone}
            dropColumn={dropColumn}
          />
        </KanbanColumn>
      ))}
    </>
  );
}

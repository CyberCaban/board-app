"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import KanbanColumn from "./KanbanColumn";
import Cards from "./Cards";
import KanbanCard from "./KanbanCard";
import { useKanbanStore } from "@/providers/kanbanProvider";
import clsx from "clsx";

// interface InteractiveColumnsProps {}
export default function InteractiveColumns() {
  const dragItem = useRef<HTMLDivElement | null>(null);
  const dragOverItem = useRef<HTMLDivElement | null>(null);
  const [kstore] = useKanbanStore((state) => state);

  const [dragged, setDragged] = useState<string | null>(null);
  const phantomCard = useRef<HTMLDivElement | null>(null);
  const [dropZone, setDropZone] = useState<number | null>(null);
  const [dropColumn, setDropColumn] = useState<string | null>(null);
  const dCard = useMemo(
    () => kstore.cards.find((c) => c.id === dragged),
    [kstore.cards, dragged],
  );

  useEffect(() => {
    const mouseHandler = (e: MouseEvent) => {
      if (phantomCard.current === null) return;
      phantomCard.current!.style.left = `${e.x - 20}px`;
      phantomCard.current!.style.top = `${e.y - 20}px`;
      if (dragged === null || phantomCard.current === null) return;
      const elems = document.querySelectorAll(
        `.drop-zone` + `.column-${dropColumn}`,
      );

      const positions = Array.from(elems).map(
        (e) => e.getBoundingClientRect().top,
      );

      const diffs = positions.map((p) => Math.abs(p - e.y));
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
    };
    document.addEventListener("mousemove", mouseHandler);
    return () => document.removeEventListener("mousemove", mouseHandler);
  }, [dCard, dragged, dropColumn]);

  return (
    <>
      {dragged !== null && (
        <div
          className={clsx(
            "phantom-card pointer-events-none absolute w-[300px]",
            { hidden: !phantomCard.current },
          )}
          ref={phantomCard}
        >
          <KanbanCard card={dCard!} />
        </div>
      )}
      {kstore.columns.map((col) => (
        <KanbanColumn
          key={col.id}
          id={col.id}
          title={col.name}
          onMouseOver={() => setDropColumn(col.id)}
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

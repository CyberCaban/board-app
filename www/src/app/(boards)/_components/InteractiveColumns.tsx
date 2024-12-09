"use client";

import { useEffect, useMemo, useRef, useState } from "react";
import KanbanColumn from "./KanbanColumn";
import Cards from "./Cards";
import KanbanCard from "./KanbanCard";
import { useKanbanStore } from "@/providers/kanbanProvider";

// interface InteractiveColumnsProps {}
export default function InteractiveColumns() {
  const [kstore] = useKanbanStore((state) => state);

  const [dragged, setDragged] = useState<string | null>(null);
  const phantomCard = useRef<HTMLDivElement | null>(null);
  const [dropZone, setDropZone] = useState<number | null>(null);
  const [dropColumn, setDropColumn] = useState<string | null>(null);
  const [animating, setAnimating] = useState(false);
  const dCard = useMemo(
    () => kstore.cards.find((c) => c.id === dragged),
    [kstore.cards, dragged],
  );

  useEffect(() => {
    const mouseHandler = (e: MouseEvent) => {
      if (phantomCard.current === null) return;

      requestAnimationFrame(() => {
        if (!phantomCard.current) return;
        phantomCard.current.style.transform = `translate(${e.clientX - 20}px, ${e.clientY - 20}px)`;
      });

      if (dragged === null) return;

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

      setDropZone(index);
    };

    if (dragged !== null) {
      document.addEventListener("mousemove", mouseHandler, { passive: true });
    }
    return () => document.removeEventListener("mousemove", mouseHandler);
  }, [dCard, dragged, dropColumn]);

  function onReorderEnd(e: React.MouseEvent) {
    e.preventDefault();
    if (dropZone === null || dropColumn === null || animating) return;

    const newPosition =
      dCard!.column_id === dropColumn
        ? dropZone <= dCard!.position
          ? dropZone
          : dropZone - 1
        : dropZone;

    setAnimating(true);

    const cards = document.querySelectorAll(".kanban-card");
    const positions = Array.from(cards).map((card) => {
      const rect = card.getBoundingClientRect();
      return { top: rect.top, left: rect.left };
    });

    kstore.reorderList(dCard!.column_id, dropColumn!, newPosition, dCard!.id);

    requestAnimationFrame(() => {
      cards.forEach((card, i) => {
        const rect = card.getBoundingClientRect();
        const dx = positions[i].left - rect.left;
        const dy = positions[i].top - rect.top;

        if (dx !== 0 || dy !== 0) {
          card.animate(
            [
              { transform: `translate(${dx}px, ${dy}px)` },
              { transform: "translate(0, 0)" },
            ],
            {
              duration: 200,
              easing: "ease-out",
            },
          );
        }
      });

      setTimeout(() => {
        setAnimating(false);
        setDropZone(null);
        setDropColumn(null);
        setDragged(null);
      }, 100);
    });
  }

  return (
    <>
      {dragged !== null && (
        <div
          className="phantom-card pointer-events-none fixed z-50 w-full max-w-[250px] will-change-transform"
          style={{
            position: "fixed",
            left: 0,
            top: 0,
            transform: "translate(0, 0)",
            opacity: phantomCard.current ? 1 : 0,
            transition: "opacity 0.2s ease-out",
          }}
          ref={phantomCard}
        >
          <KanbanCard card={dCard!} className="kanban-card cursor-grabbing" />
        </div>
      )}
      {kstore.columns.map((col) => (
        <KanbanColumn
          key={col.id}
          id={col.id}
          title={col.name}
          onMouseOver={() => setDropColumn(col.id)}
          onMouseUp={onReorderEnd}
        >
          <Cards
            cards={kstore.cards}
            column_id={col.id}
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

"use client";

import { use, useEffect, useRef, useState } from "react";
import KanbanColumn from "../../_components/KanbanColumn";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useKanbanStore } from "@/providers/kanbanProvider";
import KanbanCard from "../../_components/KanbanCard";
import { toast } from "sonner";

type Params = Promise<{ id: string }>;

export default function Dashboard(props: { params: Params }) {
  const { id } = use(props.params);
  const [kstore] = useKanbanStore((state) => state);

  const [isDanglingColumn, setIsDanglingColumn] = useState(false);
  const columnInputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    kstore.requestBoard(id).catch((e) => {
      toast.error(e.message);
      kstore.reset();
    });
    return () => kstore.reset();
  }, [id]);

  useEffect(() => {
    if (columnInputRef.current) {
      columnInputRef.current.focus();
    }
  }, [isDanglingColumn]);

  const handleAddColumn = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    const columnName = columnInputRef.current?.value;
    if (!columnName) return;
    kstore.addColumn(columnName, kstore.columns.length);
    setIsDanglingColumn(false);
  };

  return (
    <>
      {kstore.id && (
        <>
          <div>
            <h1>{kstore.name}</h1>
            <p>{kstore.id}</p>
          </div>
          <ol className="flex w-11/12 h-[calc(100vh-200px)] flex-row items-start gap-4 overflow-x-scroll p-2">
            {kstore.columns.map((column) => (
              <KanbanColumn key={column.id} id={column.id} title={column.name}>
                {kstore.cards
                  .filter((card) => card.column_id === column.id)
                  .toSorted((a, b) => a.position - b.position)
                  .map((card) => (
                    <KanbanCard
                      key={card.id}
                      id={card.id}
                      description={card.description}
                      position={card.position}
                      column_id={card.column_id}
                    />
                  ))}
              </KanbanColumn>
            ))}
            {isDanglingColumn ? (
              <>
                <form onSubmit={handleAddColumn}>
                  <Input
                    type="text"
                    className="mt-4"
                    placeholder="Enter column name"
                    onBlur={() => setIsDanglingColumn(false)}
                    ref={columnInputRef}
                  />
                  <Button type="submit" className="mt-4">
                    Add Column
                  </Button>
                </form>
              </>
            ) : (
              <Button
                className="mt-4"
                onClick={() => setIsDanglingColumn(true)}
              >
                Add Column
              </Button>
            )}
          </ol>
        </>
      )}
    </>
  );
}

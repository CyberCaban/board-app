"use client";

import { use, useEffect, useRef, useState } from "react";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useKanbanStore } from "@/providers/kanbanProvider";
import { toast } from "sonner";
import Link from "next/link";
import InteractiveColumns from "../../_components/InteractiveColumns";
import { DropdownMenu, DropdownMenuContent, DropdownMenuTrigger } from "@/components/ui/dropdown-menu";
import { useRouter } from "next/navigation";

function Error({ message }: { message: string }) {
  return (
    <>
      {message && (
        <div className="flex flex-col items-center gap-4">
          <h2>Something went wrong!</h2>
          <p className="break-words text-xl font-bold">{message}</p>
          <Link className="font-bold underline" href={"/board"}>
            Back to boards
          </Link>
        </div>
      )}
    </>
  );
}

type Params = Promise<{ id: string }>;

function Board(props: { params: Params }) {
  const router = useRouter();
  const { id } = use(props.params);
  const [kstore] = useKanbanStore((state) => state);

  const [isDanglingColumn, setIsDanglingColumn] = useState(false);
  const columnInputRef = useRef<HTMLInputElement>(null);

  const [error, setError] = useState<string | null>();

  useEffect(() => {
    kstore.requestBoard(id).catch((e: Error) => {
      toast.error(e.message);
      kstore.reset();
      setError(e.message);
    });
    return () => kstore.reset();
    // eslint-disable-next-line react-hooks/exhaustive-deps
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
      {error && <Error message={error} />}
      {kstore.id && (
        <div>
          <div>
            <div className="flex flex-row items-center gap-4">
              <h1>{kstore.name}</h1>
              <DropdownMenu>
                <DropdownMenuTrigger asChild>
                  <Button variant="outline" size="sm" className="bg-black">...</Button>
                </DropdownMenuTrigger>
                <DropdownMenuContent>
                  <Button
                    onClick={() => {
                      kstore
                        .deleteBoard()
                        .then(() => router.replace("/board"))
                        .catch((e) => toast.error(e.message));
                    }}
                  >
                    Delete board
                  </Button>
                </DropdownMenuContent>
              </DropdownMenu>
            </div>
            <p>{kstore.id}</p>
          </div>
          <ol className="flex h-[calc(100vh-200px)] w-11/12 flex-row items-start gap-4 overflow-x-auto p-2">
            <InteractiveColumns />
            {isDanglingColumn ? (
              <div className="min-w-60">
                <form onSubmit={handleAddColumn}>
                  <Input
                    type="text"
                    className="mt-4"
                    placeholder="Enter column name"
                    onBlur={() => setIsDanglingColumn(false)}
                    ref={columnInputRef}
                  />
                  {/* <Button type="submit" className="mt-4">
                    Add Column
                  </Button> */}
                </form>
              </div>
            ) : (
              <Button
                className="mt-4 min-w-60"
                onClick={() => setIsDanglingColumn(true)}
              >
                Add Column
              </Button>
            )}
          </ol>
        </div>
      )}
    </>
  );
}

export default function Dashboard({ params }: { params: Params }) {
  return <Board params={params} />;
}

import clsx from "clsx";

export default function DropZone({
  pos,
  dragged,
  dropZone,
  column_id,
  dropColumn,
}: {
  pos: number;
  dragged: string | null;
  dropZone: number | null;
  column_id: string;
  dropColumn: string | null;
}) {
  return (
    <div
      className={clsx([
        "drop-zone w-full px-4",
        `column-${column_id}`,
        {
          hidden:
            dragged === null && dropZone !== pos,
          "mb-2 rounded-md bg-neutral-800 py-2":
            dragged !== null && dropZone === pos && column_id === dropColumn,
        },
      ])}
    ></div>
  );
}

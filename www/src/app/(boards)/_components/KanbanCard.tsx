interface KanbanCardProps {
  id: string;
  description: string;
  column_id: string;
  position: number;
}
export default function KanbanCard({ description }: KanbanCardProps) {
  return (
    <div className="w-full mx-1 my-2 cursor-pointer rounded-md bg-neutral-400 px-4 py-2 text-black hover:bg-neutral-300">
      <p className="break-words">{description}</p>
    </div>
  );
}

"use client";
import Chat from "@/components/chat/Chat";
import { use } from "react";

type Params = Promise<{ id: string }>;
export default function NewPage({ params }: { params: Params }) {
  const { id: receiver_id } = use(params);

  return (
    <div className="h-full">
      <Chat receiver_id={receiver_id} />
    </div>
  );
}

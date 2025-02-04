"use client";
import { useUserStore } from "@/providers/userProvider";
import { getCookie, postData, postFormData } from "@/utils/utils";
import { FormEvent, use, useEffect, useState } from "react";

interface Message {
  content: string;
  sender_id: string;
  receiver_id: string;
  created_at: number;
}

type Params = Promise<{ id: string }>;

export default function Chat({ params }: { params: Params }) {
  const { id } = use(params);
  const [store] = useUserStore((s) => s);
  const [ev, setEv] = useState<WebSocket>();
  const [msg, setMsg] = useState<Message[]>([]);
  const [t, setT] = useState("");

  useEffect(() => {
    const es = new WebSocket(`http://localhost:3000/chat_source/events`);
    const onMessage = (e: MessageEvent) => {
      console.log(e);
      const m = JSON.parse(e.data);
      setMsg((prev) => [...prev, m]);
    };
    const onOpen = (e: Event) => {
      console.log("Open: ", e);
      es.send(getCookie("token") || "");
    };

    const onError = (e: Event) => {
      console.log("cloes: ", e);

      ev?.close();
    };
    es.onmessage = onMessage;
    es.onerror = onError;
    es.onopen = onOpen;

    setEv(es);
    return () => {};
  }, []);
  const handleSend = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    ev?.send(
      JSON.stringify({
        content: t,
        sender_id: store.id,
        receiver_id: id,
        created_at: Date.now(),
      }),
    );
    setT("");
  };
  return (
    <div>
      {msg.map((m) => (
        <div key={m.created_at}>{m.content}</div>
      ))}
      <form onSubmit={handleSend}>
        <input type="text" onChange={(e) => setT(e.target.value)} value={t} />
        <button type="submit">send</button>
      </form>
    </div>
  );
}

"use client";
import ChatMsgs from "@/components/ChatMsgs";
import { useUserStore } from "@/providers/userProvider";
import { IMessage, IPubUser } from "@/types";
import { getData } from "@/utils/utils";
import WS from "@/utils/ws";
import { FormEvent, use, useEffect, useRef, useState } from "react";

type Params = Promise<{ id: string }>;

export default function Chat({ params }: { params: Params }) {
  const { id: receiver_id } = use(params);
  const [store] = useUserStore((s) => s);
  const [ws, setWs] = useState<WS>();
  const [msg, setMsg] = useState<IMessage[]>([]);
  const [messageInput, setMessageInput] = useState("");
  const [user, setUser] = useState<IPubUser | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    console.log(msg);
  }, [msg]);

  useEffect(() => {
    const loadMessages = () => {
      getData("/chat_source/last_messages").then((res) => {
        setMsg(res.toReversed());
      });
    };
    loadMessages();
    getData(`/api/user/${receiver_id}`).then((res) => {
      setUser(res);
    });

    const ws = new WS("/chat_source/events", (msg: IMessage) => {
      setMsg((prev) => [...prev, msg]);
      inputRef.current?.focus();
      inputRef.current?.scrollIntoView({ behavior: "smooth" });
    });
    setWs(ws);


    return () => {
      ws?.close();
    };
  }, []);

  const handleSend = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    ws?.send({
      content: messageInput,
      sender_id: store.id,
      receiver_id,
      created_at: Date.now(),
    });
    setMessageInput("");
  };

  return (
    <div>
      <ChatMsgs msg={msg} user_id={store.id} user={user} />
      <form onSubmit={handleSend}>
        <input
          ref={inputRef}
          type="text"
          onChange={(e) => setMessageInput(e.target.value)}
          value={messageInput}
        />
        <button type="submit">send</button>
      </form>
    </div>
  );
}

"use client";
import { useUserStore } from "@/providers/userProvider";
import { IConversation, IMember, IMessage } from "@/types";
import { FormEvent, use, useEffect, useRef, useState } from "react";
import { findConversation, getLastMessages } from "./conversation";
import { getCookie } from "@/utils/utils";
import ChatMsgs from "@/components/ChatMsgs";

type Params = Promise<{ id: string }>;
export default function NewPage({ params }: { params: Params }) {
  const { id: receiver_id } = use(params);
  const [store] = useUserStore((s) => s);
  const [conversation, setConversation] = useState<IConversation | null>(null);
  const [messages, setMessages] = useState<IMessage[]>([]);
  const [members, setMembers] = useState<IMember[]>([]);
  const [ws, setWs] = useState<WebSocket | null>(null);
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (store.id) {
      findConversation(store.id, receiver_id).then((res) => {
        const [conversation, ...members] = res;
        setConversation(conversation);
        setMembers(members);
      });
    }
  }, [store.id, receiver_id]);

  useEffect(() => {
    if (!conversation) return;
    getLastMessages(conversation.id).then((res: IMessage[]) => {
      setMessages(res.toReversed());
    });
    const ws: WebSocket = new WebSocket(`/chat_source/events`);

    const handshake = {
      token: getCookie("token") || "",
      conversation_id: conversation.id,
    };
    ws.onopen = () => {
      ws.send(JSON.stringify(handshake));
    };
    ws.onmessage = (e) => {
      const data = JSON.parse(e.data);
      if (data.message) {
        console.log(data.message);
      } else setMessages((prev) => [...prev, data]);
    };
    ws.onclose = (e) => {
      console.error("ws closed", e);
    };

    setWs(ws);
  }, [conversation]);

  const handleSend = (e: FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!ws || !inputRef.current) return;
    ws.send(
      JSON.stringify({
        content: inputRef.current.value,
        sender_id: store.id,
        conversation_id: conversation?.id,
        created_at: Date.now(),
      }),
    );
    inputRef.current.value = "";
  };

  return (
    <div>
      <ChatMsgs
        msg={messages}
        user_id={store.id}
        other_users={members.filter((m) => m.id !== store.id)}
      />
      <form onSubmit={handleSend}>
        <input ref={inputRef} type="text" />
        <button type="submit">send</button>
      </form>
    </div>
  );
}

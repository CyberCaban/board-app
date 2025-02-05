"use client";
import { useUserStore } from "@/providers/userProvider";
import { IMessage, IPubUser } from "@/types";
import clsx from "clsx";
import Image from "next/image";
import { useEffect, useRef } from "react";

function Message({
  msg,
  user_id,
  user,
}: {
  msg: IMessage;
  user_id: string;
  user: IPubUser | null;
}) {
  const [store] = useUserStore((s) => s);
  return (
    <div
      className={clsx(
        "flex flex-col gap-2 rounded-md bg-primary px-4 py-2 text-left",
        {
          "bg-primary/50": msg.sender_id === store.id,
        },
      )}
    >
      {user && (
        <section className="flex flex-row justify-between gap-2">
          <section className="flex flex-row gap-2">
            <Image
              src={
                msg.sender_id === store.id
                  ? store.profile_url
                  : user.profile_url
              }
              alt={msg.sender_id === store.id ? store.username : user.username}
              width={48}
              height={48}
              className="aspect-square h-12 w-12 rounded-full"
            />
            <section className="flex flex-col justify-between gap-2">
              <span className="text-sm text-muted-foreground">
                {msg.sender_id === user_id ? "You" : user.username}
              </span>
              {msg.content}
            </section>
          </section>
          <span className="text-sm text-muted-foreground">
            {new Date(msg.created_at).toLocaleTimeString()}
          </span>
        </section>
      )}
    </div>
  );
}

export default function ChatMsgs({
  msg,
  user_id,
  user,
}: {
  msg: IMessage[];
  user_id: string;
  user: IPubUser | null;
}) {
  const messagesEndRef = useRef<HTMLDivElement>(null);
  useEffect(() => {
    if (messagesEndRef.current) {
      messagesEndRef.current.scrollIntoView({
        behavior: "smooth",
        block: "end",
      });
    }
  }, [msg]);
  return (
    <section
      // ref={messagesEndRef}
      className="m-2 flex max-h-[calc(100vh-12rem)] flex-1 flex-col gap-2 overflow-y-auto rounded-md bg-primary/20"
    >
      {msg.map((m) => (
        <Message key={m.created_at} msg={m} user_id={user_id} user={user} />
      ))}
      <div ref={messagesEndRef}></div>
    </section>
  );
}


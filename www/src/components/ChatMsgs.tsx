"use client";
import { useUserStore } from "@/providers/userProvider";
import { IMessage, IPubUser } from "@/types";
import clsx from "clsx";
import Image from "next/image";
import { useEffect, useRef } from "react";

function Message({
  msg,
  other_users,
}: {
  msg: IMessage;
  user_id: string;
  other_users: IPubUser[];
}) {
  const [store] = useUserStore((s) => s);
  const other_user = other_users.find((u) => u.id === msg.sender_id);
  const name =
    msg.sender_id === store.id ? store.username : other_user?.username || "";
  function UserProfile() {
    const imgSrc =
      msg.sender_id === store.id
        ? store.profile_url
        : other_user?.profile_url || "";
    const alt =
      msg.sender_id === store.id ? store.username : other_user?.username || "";
    return (
      <Image
        src={imgSrc}
        alt={alt}
        width={48}
        height={48}
        className="aspect-square h-12 w-12 rounded-full"
      />
    );
  }
  return (
    <div
      className={clsx(
        "flex w-full flex-col gap-2 rounded-md bg-primary px-4 py-2 text-left",
        {
          "bg-primary/50": msg.sender_id === store.id,
        },
      )}
    >
      {other_users && (
        <section className="flex flex-row justify-between gap-2">
          <section className="flex flex-row gap-2">
            <UserProfile />
            <section className="flex flex-col justify-between gap-2">
              <span className="text-sm text-muted-foreground">{name}</span>
              <span className="break-all text-sm">{msg.content}</span>
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
  other_users,
}: {
  msg: IMessage[];
  user_id: string;
  other_users: IPubUser[];
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
    <section className="m-2 flex max-h-[calc(100vh-12rem)] flex-1 flex-col gap-2 overflow-y-auto rounded-md bg-primary/20">
      {msg.map((m) => (
        <Message
          key={m.created_at}
          msg={m}
          user_id={user_id}
          other_users={other_users}
        />
      ))}
      <div ref={messagesEndRef}></div>
    </section>
  );
}

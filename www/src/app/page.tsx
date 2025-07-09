"use client";
import ImagesAndUpload from "@/components/ImagesPage";
import { useEventStreamContext } from "@/providers/EventStreamContext";
import { useEffect } from "react";

export default function Home() {
  const { messages } = useEventStreamContext()

  useEffect(() => {
    console.log(messages);

  }, [messages])
  return (
    <>
      <main className="flex min-h-screen flex-col items-center p-24">
        <ImagesAndUpload />
      </main>
    </>
  );
}

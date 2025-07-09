"use client"
import { useEffect, useRef, useState } from "react";
import { useFixedSizeArray } from "./useFixedSizeArray";

const MAX_LEN = 10

export function useEventStream(url: string) {
  const { array: messages, addItem } = useFixedSizeArray<string>(10);
  const [error, setError] = useState<Error | null>(null);
  const eventSourceRef = useRef<EventSource | null>(null)

  useEffect(() => {
    eventSourceRef.current = new EventSource(url)
    eventSourceRef.current.onmessage = (e) => {
      console.log(e);

      addItem(e.data)
    }
    eventSourceRef.current.onerror = (err) => {
      setError(new Error("EventSource failed: " + err))
      // eventSourceRef.current?.close()
    }

    return () => {
      eventSourceRef.current?.close()
    }
  }, [url])

  return { messages, error }
}

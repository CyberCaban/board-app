"use client";

import { useEventStream } from "@/hooks/useEventStream";
import { createContext, useContext } from "react";

interface EventStreamContextType {
  messages: string[]
  error: Error | null
}
const EventStreamContext = createContext<EventStreamContextType | undefined>(undefined)

export const EventStreamProvider = ({ children }: { children: React.ReactNode }) => {
  const { messages, error } = useEventStream("/api/update_stream")

  return (
    <EventStreamContext.Provider value={{ messages, error }}>
      {children}
    </EventStreamContext.Provider>
  )
}

export const useEventStreamContext = () => {
  const ctx = useContext(EventStreamContext)
  if (!ctx) {
    throw new Error("useEventStreamContext must be used within a EventStreamProvider");
  }
  return ctx
}

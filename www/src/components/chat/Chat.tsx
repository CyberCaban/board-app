import { IConversation, IMember, IMessage, SearchState } from "@/types";
import { useCallback, useEffect, useRef, useState } from "react";
import { ChatHeader } from "./ChatHeader";
import { SearchPanel } from "./SearchPanel";
import { MessageList } from "./MessagesList";
import { MessageInput } from "./MessageInput";
import { FileUploadDialog } from "./FileUploadDialog";
import { useUserStore } from "@/providers/userProvider";
import {
  findConversation,
  getLastMessages,
} from "@/app/(chat)/chat/[id]/conversation";
import { getCookie } from "@/utils/utils";

export default function Chat({ receiver_id }: { receiver_id: string }) {
  const [store] = useUserStore((s) => s);
  const [messages, setMessages] = useState<IMessage[]>([]);

  const [isUploadOpen, setIsUploadOpen] = useState(false);

  const [searchState, setSearchState] = useState<SearchState>({
    query: "",
    isOpen: false,
    results: [],
    currentIndex: 0,
  });

  const [conversation, setConversation] = useState<IConversation | null>(null);
  const [members, setMembers] = useState<IMember[]>([]);
  const ws = useRef<WebSocket | null>(null);

  const messagesEndRef = useRef<HTMLDivElement>(null);

  const scrollToBottom = () => {
    messagesEndRef.current?.scrollIntoView({
      behavior: "smooth",
      block: "end",
    });
  };

  useEffect(() => {
    scrollToBottom();
  }, [messages]);

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
    ws.current = new WebSocket(`/chat_source/events`);

    const handshake = {
      token: getCookie("token") || "",
      conversation_id: conversation.id,
    };
    ws.current.addEventListener("open", () => {
      ws.current?.send(JSON.stringify(handshake));
    });
    ws.current.addEventListener("message", (e) => {
      const data = JSON.parse(e.data);
      if (data.message) {
        console.log(data.message);
      } else setMessages((prev) => [...prev, data]);
    });
    ws.current.addEventListener("close", (e) => {
      console.error("ws closed", e);
      ws.current?.close()
    });

    return () => {
      if (ws.current && ws.current.readyState === WebSocket.OPEN) {
        ws.current.close();
      }
    };
  }, [conversation]);

  const handleSendMessage = (content: string) => {
    const msg = {
      content,
      sender_id: store.id,
      conversation_id: conversation?.id,
      created_at: Date.now(),
    };
    ws.current?.send(JSON.stringify(msg));
  };

  const handleUpload = () => {};
  const handleSearch = useCallback(() => {
    const { query } = searchState;
    if (!query.trim()) {
      setSearchState((prev) => ({ ...prev, results: [] }));
      return;
    }

    const searchQuery = query.toLowerCase();
    const results = messages
      .map((message, index) => ({ index, message }))
      .filter(({ message }) =>
        message.content.toLowerCase().includes(searchQuery),
      )
      .map(({ index }) => index);

    setSearchState((prev) => ({
      ...prev,
      results,
      currentIndex: results.length > 0 ? 0 : -1,
    }));

    if (results.length > 0) {
      const messageElement = document.getElementById(`message-${results[0]}`);
      messageElement?.scrollIntoView({ behavior: "smooth", block: "center" });
    }
  }, [messages, searchState.query]);

  const toggleSearch = () => {
    setSearchState((prev) => ({ ...prev, isOpen: !prev.isOpen }));
  };

  const setSearchQuery = (query: string) => {
    setSearchState((prev) => ({ ...prev, query }));
  };

  const navigateSearch = (direction: "next" | "prev") => {
    const { results, currentIndex } = searchState;
    if (results.length === 0) return;

    let newIndex;
    if (direction === "next") {
      newIndex = (currentIndex + 1) % results.length;
    } else {
      newIndex = (currentIndex - 1 + results.length) % results.length;
    }

    setSearchState((prev) => ({ ...prev, currentIndex: newIndex }));

    const messageElement = document.getElementById(
      `message-${results[newIndex]}`,
    );
    messageElement?.scrollIntoView({ behavior: "smooth", block: "center" });
  };

  const clearSearch = () => {
    setSearchState({
      query: "",
      isOpen: false,
      results: [],
      currentIndex: 0,
    });
  };

  return (
    <section className="h-full">
      <ChatHeader
        onToggleSearch={toggleSearch}
        isSearchOpen={searchState.isOpen}
      />

      {searchState.isOpen ? (
        <SearchPanel
          searchState={searchState}
          setSearchQuery={setSearchQuery}
          handleSearch={handleSearch}
          navigateSearch={navigateSearch}
          clearSearch={clearSearch}
        />
      ) : null}

      <MessageList
        messages={messages}
        searchState={searchState}
        members={members}
      />

      <MessageInput
        onOpenUpload={() => setIsUploadOpen(true)}
        onSendMessage={handleSendMessage}
      />
      <FileUploadDialog
        isOpen={isUploadOpen}
        onOpenChange={setIsUploadOpen}
        onUpload={handleUpload}
      />
    </section>
  );
}

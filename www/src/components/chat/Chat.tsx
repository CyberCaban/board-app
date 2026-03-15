import {
  IConversation,
  IMember,
  IMessage,
  intoIMessage,
  SearchState,
} from "@/types";
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
import { postData } from "@/utils/utils";
import { useCentrifugo } from "@/providers/centrifugoProvider";

export default function Chat({ receiver_id }: { receiver_id: string }) {
  const [store] = useUserStore((s) => s);
  const { addMessageListener } = useCentrifugo();
  const [messages, setMessages] = useState<IMessage[]>([]);

  const [, setFileId] = useState("");

  const [isUploadOpen, setIsUploadOpen] = useState(false);

  const [searchState, setSearchState] = useState<SearchState>({
    query: "",
    isOpen: false,
    results: [],
    currentIndex: 0,
  });

  const [conversation, setConversation] = useState<IConversation | null>(null);
  const [members, setMembers] = useState<IMember[]>([]);

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
    const removeListener = addMessageListener((message) => {
      setMessages((prev) => [...prev, intoIMessage(message)]);
    });

    return () => {
      removeListener();
    };
  }, [conversation]);

  const handleSendMessage = (content: string) => {
    const msg = {
      content,
      sender_id: store.id,
      conversation_id: conversation?.id,
      created_at: Date.now(),
      file_id: null,
    };
    postData("/chat_source/message", msg).catch((err) => {
      console.error("Failed to send message:", err);
    });
  };

  const handleUpload = (fileName: string) => {
    setFileId(fileName);
  };
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

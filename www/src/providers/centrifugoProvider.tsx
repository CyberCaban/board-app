"use client";

import { useUserStore } from "@/providers/userProvider";
import { getData, postData } from "@/utils/utils";
import { Centrifuge, type Subscription } from "centrifuge";
import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useRef,
  useState,
} from "react";

type ConnectionState = "disconnected" | "connecting" | "connected";

type ConnectResponse = {
  token?: string;
  centrifugo_token?: string;
};

type SubscribeHandlers<T = unknown> = {
  onPublication?: (data: T) => void;
  onSubscribed?: () => void;
  onError?: (error: unknown) => void;
};

type CentrifugoContextValue = {
  connectionState: ConnectionState;
  connect: () => Promise<Centrifuge>;
  disconnect: () => void;
  getClient: () => Centrifuge | null;
  subscribe: <T = unknown>(
    channel: string,
    handlers?: SubscribeHandlers<T>,
  ) => (() => void) | null;
};

const CentrifugoContext = createContext<CentrifugoContextValue | undefined>(
  undefined,
);

function resolveCentrifugoWsUrl() {
  return (
    process.env.NEXT_PUBLIC_CENTRIFUGO_WS_URL?.trim() ||
    "ws://localhost:8000/connection/websocket"
  );
}

export function CentrifugoProvider({
  children,
}: {
  children: React.ReactNode;
}) {
  const [userId] = useUserStore((state) => state.id);
  const [connectionState, setConnectionState] =
    useState<ConnectionState>("disconnected");
  const clientRef = useRef<Centrifuge | null>(null);
  const tokenRef = useRef("");
  const connectionStateRef = useRef<ConnectionState>("disconnected");
  const connectPromiseRef = useRef<Promise<Centrifuge> | null>(null);

  const updateConnectionState = useCallback((state: ConnectionState) => {
    connectionStateRef.current = state;
    setConnectionState(state);
  }, []);

  const connect = useCallback(async () => {
    if (!userId) {
      throw new Error("Cannot connect to Centrifugo without authorized user");
    }

    if (
      clientRef.current &&
      (connectionStateRef.current === "connected" ||
        connectionStateRef.current === "connecting")
    ) {
      return clientRef.current;
    }

    if (connectPromiseRef.current) {
      return connectPromiseRef.current;
    }

    updateConnectionState("connecting");

    const connectPromise = (async () => {
      const response = (await getData(
        "/api/centrifugo/connect",
      )) as ConnectResponse;

      const token = response.token ?? response.centrifugo_token ?? "";

      if (!token) {
        updateConnectionState("disconnected");
        throw new Error("Centrifugo token is missing in connect response");
      }

      if (clientRef.current && tokenRef.current !== token) {
        clientRef.current.disconnect();
        clientRef.current = null;
      }

      if (!clientRef.current) {
        const url = resolveCentrifugoWsUrl();
        if (!url) {
          updateConnectionState("disconnected");
          throw new Error("Centrifugo URL is not configured");
        }

        const client = new Centrifuge(url, { token });
        client.on("connected", () => {
          updateConnectionState("connected");
        });
        client.on("disconnected", () => updateConnectionState("disconnected"));

        clientRef.current = client;
        tokenRef.current = token;
        client.connect();
        return client;
      }

      tokenRef.current = token;
      clientRef.current.connect();
      return clientRef.current;
    })().finally(() => {
      connectPromiseRef.current = null;
    });

    connectPromiseRef.current = connectPromise;
    return connectPromise;
  }, [updateConnectionState, userId]);

  const disconnect = useCallback(() => {
    clientRef.current?.disconnect();
    clientRef.current = null;
    tokenRef.current = "";
    connectPromiseRef.current = null;
    updateConnectionState("disconnected");
  }, [updateConnectionState]);

  const getClient = useCallback(() => clientRef.current, []);

  const subscribe = useCallback(
    <T,>(
      channel: string,
      handlers?: SubscribeHandlers<T>,
    ): (() => void) | null => {
      const client = clientRef.current;
      if (!client) {
        return null;
      }

      const sub: Subscription = client.newSubscription(channel);

      sub.on("publication", (ctx) => {
        handlers?.onPublication?.(ctx.data as T);
      });

      sub.on("subscribed", () => {
        handlers?.onSubscribed?.();
      });

      sub.on("error", (ctx) => {
        handlers?.onError?.(ctx);
      });

      sub.subscribe();

      return () => {
        sub.unsubscribe();
      };
    },
    [],
  );

  useEffect(() => {
    if (!userId) {
      disconnect();
      return;
    }

    connect().catch(() => {
      updateConnectionState("disconnected");
    });
  }, [connect, disconnect, updateConnectionState, userId]);

  // Subscribe to user-specific channel after successful connection
  useEffect(() => {
    if (connectionState === "connected") {
      subscribe("chat#" + userId, {
        onSubscribed: () => {
          console.log("Received publication on user channel: 'chat#" + userId + "'");
        },
        onError: () => {
          disconnect();
        },
      });
    }
  }, [connectionState, subscribe, disconnect, userId]);

  useEffect(() => {
    return () => {
      disconnect();
    };
  }, [disconnect]);

  const value = useMemo<CentrifugoContextValue>(
    () => ({
      connectionState,
      connect,
      disconnect,
      getClient,
      subscribe,
    }),
    [connect, connectionState, disconnect, getClient, subscribe],
  );

  return (
    <CentrifugoContext.Provider value={value}>
      {children}
    </CentrifugoContext.Provider>
  );
}

export function useCentrifugo() {
  const context = useContext(CentrifugoContext);

  if (!context) {
    throw new Error("useCentrifugo must be used within a CentrifugoProvider");
  }

  return context;
}

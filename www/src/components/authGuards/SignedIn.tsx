"use client";
import { useUserStore } from "@/providers/userProvider";

export default function SignedOut({ children }: { children: React.ReactNode }) {
  const [store] = useUserStore((s) => s);

  return <>{store.id ? children : null}</>;
}

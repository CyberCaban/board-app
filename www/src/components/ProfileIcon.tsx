"use client";
import { useUserStore } from "@/providers/userProvider";
import { Button } from "./ui/button";
import Link from "next/link";

export default function Profile() {
  const [state] = useUserStore((state) => state);

  return (
    <Link href="/profile">
      <Button onClick={state.logout}>Logout</Button>
      {state.username}
    </Link>
  );
}

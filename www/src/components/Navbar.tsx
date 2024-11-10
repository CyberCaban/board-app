"use client";
import Link from "next/link";
import Profile from "./ProfileIcon";
import { useUserStore } from "@/providers/userProvider";

export default function Navbar() {
  const [store] = useUserStore((state) => state);
  return (
    <nav className="flex flex-row bg-foreground p-4 gap-4 justify-between">
      <div className="left_pad flex flex-row gap-4">
        <Link href="/">Home</Link>
        <Link href="/register">Register</Link>
        <Link href="/login">Login</Link>
      </div>
      <div className="right_pad">{store.id ? <Profile /> : null}</div>
    </nav>
  );
}

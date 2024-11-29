"use client";
import Link from "next/link";
import Profile from "./ProfileIcon";
import { useUserStore } from "@/providers/userProvider";
import { useEffect } from "react";
import { getData } from "@/utils/utils";

export default function Navbar() {
  const [store] = useUserStore((state) => state);

  useEffect(() => {
    getData("/api/user")
      .then((res) => {
        store.setUser({
          id: res.id,
          username: res.username,
          profile_url: res.profile_url,
        });
        console.log(res);
      })
      .catch((err) => {
        console.error(err);
        store.resetUser();
      });
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <nav className="flex flex-row justify-between gap-4 bg-foreground p-4">
      <div className="left_pad flex flex-row gap-4">
        <Link href="/">Home</Link>
        <Link href="/register">Register</Link>
        <Link href="/login">Login</Link>
        <Link href="/board">Boards</Link>
      </div>
      <div className="right_pad">{store.id ? <Profile /> : null}</div>
    </nav>
  );
}

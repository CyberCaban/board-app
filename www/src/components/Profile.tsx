"use client";

import { useUserStore } from "@/providers/userProvider";
import Button from "./Button";

export default function Profile() {
  const user = useUserStore((state) => state);

  const logout = () => {
    fetch("/api/logout", { method: "POST" });
    user.resetUser();
  };

  return (
    <>
      <Button onClick={logout}>Logout</Button>
      {user.username}
    </>
  );
}

"use client";
import { useState } from "react";
import { useUserStore } from "@/providers/userProvider";

export default function LoginForm() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [state] = useUserStore((state) => state);

  const login = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    state.login(username, password);
  };

  return (
    <form onSubmit={login}>
      <h1>Login</h1>
      <label htmlFor="loginUsername">Username</label>
      <input
        type="text"
        name="loginUsername"
        id="loginUsername"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
      />
      <label htmlFor="loginPassword">Password</label>
      <input
        type="password"
        name="loginPassword"
        id="loginPassword"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      <button type="submit">Login</button>
    </form>
  );
}

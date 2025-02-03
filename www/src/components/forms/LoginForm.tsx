"use client";
import { useState } from "react";
import { useUserStore } from "@/providers/userProvider";

export default function LoginForm() {
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [state] = useUserStore((state) => state);

  const login = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    state.login(email, password);
  };

  return (
    <form onSubmit={login}>
      <h1>Login</h1>
      <label htmlFor="loginEmail">Email</label>
      <input
        type="text"
        name="loginEmail"
        id="loginEmail"
        required
        value={email}
        onChange={(e) => setEmail(e.target.value)}
      />
      <label htmlFor="loginPassword">Password</label>
      <input
        type="password"
        name="loginPassword"
        id="loginPassword"
        required
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      <button type="submit">Login</button>
    </form>
  );
}

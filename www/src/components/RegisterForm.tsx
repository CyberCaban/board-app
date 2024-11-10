"use client";
import { useState } from "react";
import { useUserStore } from "@/providers/userProvider";

export default function RegisterForm() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const [state] = useUserStore((state) => state);

  const register = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    state.register(username, password);
  };

  return (
    <form onSubmit={register}>
      <h1>Register</h1>
      <label htmlFor="registerUsername">Username</label>
      <input
        type="text"
        name="registerUsername"
        id="registerUsername"
        value={username}
        onChange={(e) => setUsername(e.target.value)}
      />
      <label htmlFor="registerPassword">Password</label>
      <input
        type="password"
        name="registerPassword"
        id="registerPassword"
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      <button type="submit">Register</button>
    </form>
  );
}

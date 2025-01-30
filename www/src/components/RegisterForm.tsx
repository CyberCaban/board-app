"use client";
import { useState } from "react";
import { useUserStore } from "@/providers/userProvider";

export default function RegisterForm() {
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [state] = useUserStore((state) => state);

  const register = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    state.register(username, email, password);
  };

  return (
    <form onSubmit={register}>
      <h1>Register</h1>
      <label htmlFor="registerUsername">Username</label>
      <input
        type="text"
        name="registerUsername"
        id="registerUsername"
        required
        value={username}
        onChange={(e) => setUsername(e.target.value)}
      />
      <label htmlFor="loginEmail">Email</label>
      <input
        type="email"
        name="loginEmail"
        id="loginEmail"
        required
        value={email}
        onChange={(e) => setEmail(e.target.value)}
      />
      <label htmlFor="registerPassword">Password</label>
      <input
        type="password"
        name="registerPassword"
        id="registerPassword"
        required
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      <button type="submit">Register</button>
    </form>
  );
}

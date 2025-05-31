"use client";
import { useState } from "react";
import { useUserStore } from "@/providers/userProvider";
import { Input } from "../ui/input";
import { Button } from "../ui/button";
import { Label } from "../ui/label";

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
      <Label htmlFor="loginEmail">Email</Label>
      <Input
        type="text"
        name="loginEmail"
        id="loginEmail"
        required
        value={email}
        onChange={(e) => setEmail(e.target.value)}
      />
      <Label htmlFor="loginPassword">Password</Label>
      <Input
        type="password"
        name="loginPassword"
        id="loginPassword"
        required
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      <Button type="submit">Login</Button>
    </form>
  );
}

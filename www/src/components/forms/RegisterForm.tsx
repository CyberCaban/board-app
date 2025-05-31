"use client";
import { useState } from "react";
import { useUserStore } from "@/providers/userProvider";
import { Label } from "../ui/label";
import { Input } from "../ui/input";
import { Button } from "../ui/button";

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
      <Label htmlFor="registerUsername">Username</Label>
      <Input
        type="text"
        name="registerUsername"
        id="registerUsername"
        required
        value={username}
        onChange={(e) => setUsername(e.target.value)}
      />
      <Label htmlFor="loginEmail">Email</Label>
      <Input
        type="email"
        name="loginEmail"
        id="loginEmail"
        required
        value={email}
        onChange={(e) => setEmail(e.target.value)}
      />
      <Label htmlFor="registerPassword">Password</Label>
      <Input
        type="password"
        name="registerPassword"
        id="registerPassword"
        required
        value={password}
        onChange={(e) => setPassword(e.target.value)}
      />
      <Button type="submit">Register</Button>
    </form>
  );
}

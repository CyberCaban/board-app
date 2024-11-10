"use client";
import { useState } from "react";
import { postData } from "../utils/utils";
import { useUserStore } from "@/providers/userProvider";

export default function LoginForm() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const { setUser } = useUserStore((state) => state);
  const Form = (
    <form
      onSubmit={(e) => {
        e.preventDefault();
        postData("/api/login", {
          username: username,
          password: password,
        }).then((res) => {
          console.log(res);
          setUser(res);
        });
      }}
    >
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
  return <div>{Form}</div>;
}

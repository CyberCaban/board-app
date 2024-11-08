"use client";
import { useState } from "react";
import { postData } from "../utils/utils";

export default function RegisterForm() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  return (
    <>
      <form
        onSubmit={(e) => {
          e.preventDefault();
          postData("/api/register", {
            username: username,
            password: password,
          }).then((res) => {
            console.log(res);
          });
        }}
      >
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
    </>
  );
}

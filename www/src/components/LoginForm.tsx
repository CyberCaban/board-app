import {  useState } from "react";
import { postData } from "../utils/utils";

export default function LoginForm() {
  const [username, setUsername] = useState("");
  const [password, setPassword] = useState("");
  const Form = (
    <form
      className="create-user-form"
      onSubmit={(e) => {
        e.preventDefault();
        postData("/api/login", {
          username: username,
          password: password,
        }).then((data) => {
          console.log(data);
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
  return (
    <div>
      {Form}
    </div>
  );
}

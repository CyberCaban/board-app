import { useState } from "react";
import "./App.css";
import { getData, postData } from "./utils/utils";

function App() {
  const [msg, setMsg] = useState("");
  const [name, setName] = useState("");

  return (
    <>
      <div className="card">
        <pre style={{ textAlign: "left" }}>{msg}</pre>
        <button
          onClick={() =>
            getData("/api/hello").then((data) => {
              console.log(data);
              setMsg(JSON.stringify(data, null, 2));
            })
          }
        >
          Click to test GET api!
        </button>
        <input
          type="text"
          name="name"
          id="name"
          onChange={(e) => setName(e.target.value)}
        />
        <button
          onClick={() =>
            postData("/api/hello", { msg: name }).then((data) => {
              console.log(data);
              setMsg(JSON.stringify(data, null, 2));
            })
          }
        >
          Click to test POST api!{" "}
        </button>
        <button
          onClick={() =>
            fetch("/api/get_users")
              .then((res) => res.json())
              .then((data) => {
                console.log(JSON.stringify(data, null, 2));
                setMsg(JSON.stringify(data, null, 2));
              })
          }
        >
          Click to test db!
        </button>
        <form
          className="create-user-form"
          onSubmit={(e) => {
            e.preventDefault();
            postData("/api/register", {
              username: e.target.username.value,
              password: e.target.password.value,
            }).then((data) => {
              console.log(data);
            });
          }}
        >
          <label htmlFor="username">Username</label>
          <input type="text" name="username" id="username" />
          <label htmlFor="password">Password</label>
          <input type="password" name="password" id="password" />
          <input type="submit" value="Submit" />
        </form>
      </div>
    </>
  );
}

export default App;

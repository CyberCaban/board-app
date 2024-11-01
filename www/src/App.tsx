import { useEffect, useState } from "react";
import "./App.css";
import { getData, postData } from "./utils/utils";
import LoginForm from "./components/LoginForm";
import RegisterForm from "./components/RegisterForm";
import FileUploadForm from "./components/FileUploadForm";
import ImagesMasonry from "./components/ImagesMasonry";
import { useAtom } from "jotai";
import { masonryNeedsUpdate } from "./store";

window.delbnt = false;

export interface IFile {
  id: string;
  name: string;
  private: boolean;
  user_id: string;
}

function App() {
  const [msg, setMsg] = useState("");
  const [files, setFiles] = useState<IFile[]>([]);

  useEffect(() => {}, [window.delbnt]);

  return (
    <>
      <div className="card">
        {/* <pre style={{ textAlign: "left" }}>{msg}</pre> */}
        <div className="flex flex-row gap-2">
          <RegisterForm />
          <LoginForm />
        </div>
        <button onClick={() => postData("/api/logout", {})}>Logout</button>

        {files &&
          files.map((file) => (
            <div key={file.name}>
              <a href={`/uploads/${file.name}`}>{file.name}</a>
              {window.delbnt && (
                <button
                  className="ml-2 px-2 py-1 bg-red-500 text-white rounded-md"
                  onClick={() => {
                    fetch(`/api/file/${file.name}`, { method: "DELETE" });
                    getData("/api/files")
                      .then((res: IFile[]) => {
                        console.log(res);
                        setMsg(JSON.stringify(res, null, 2));
                        setFiles(res);
                      })
                      .catch((err) => console.error(err));
                  }}
                >
                  delete
                </button>
              )}
            </div>
          ))}
        <button
          onClick={() =>
            getData("/api/files")
              .then((res) => {
                if (res.error_msg) {
                  throw new Error(res.error_msg);
                }
                console.log(res);
                setMsg(JSON.stringify(res, null, 2));
                setFiles(res);
              })
              .catch((err) => console.error(err))
          }
        >
          Get all files
        </button>
        <ImagesMasonry />

        <FileUploadForm />
      </div>
    </>
  );
}

export default App;

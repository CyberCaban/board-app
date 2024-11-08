import "./App.css";
import { getData, postData } from "./utils/utils";
import LoginForm from "./components/LoginForm";
import RegisterForm from "./components/RegisterForm";
import FileUploadForm from "./components/FileUploadForm";
import ImagesMasonry from "./components/ImagesMasonry";
import { useAtom } from "jotai";
import { masonryNeedsUpdate } from "./store";

export interface IFile {
  id: string;
  name: string;
  private: boolean;
  user_id: string;
}

function App() {
  const [, setNeedsUpdate] = useAtom(masonryNeedsUpdate);
  return (
    <>
      <main>
        <div className="flex flex-row gap-2">
          <RegisterForm />
          <LoginForm />
        </div>
        <button
          onClick={() => {
            postData("/api/logout", {}).then((res) => {
              console.log(res);
              setNeedsUpdate((prev) => prev + 1);
            });
          }}
        >
          Logout
        </button>
        <button
          onClick={() =>
            getData("/api/files").then((res) => {
              console.log(res);
            })
          }
        >
          Get all files
        </button>
        <ImagesMasonry />
        <FileUploadForm />
      </main>
    </>
  );
}

export default App;

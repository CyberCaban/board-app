import { useRef, useState } from "react";
import { masonryNeedsUpdate } from "../store";
import { useAtom } from "jotai";

export default function FileUploadForm() {
  const fileInput = useRef<HTMLInputElement>(null);
  const [filename, setFilename] = useState<string>("");
  const [isPrivate, setIsPrivate] = useState<boolean>(false);
  const [, setNeedsUpdate] = useAtom(masonryNeedsUpdate);

  const handleFileChange = () => {
    if (fileInput.current?.files?.length === 0) return;
    if (!fileInput.current || !fileInput.current.files) return;
    const file = fileInput.current.files?.[0];
    const nameWithoutExt = file.name.split(".")[0];
    setFilename(nameWithoutExt);
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!fileInput.current?.files?.length) return;
    const file = fileInput.current.files?.[0];
    const formData = new FormData();
    formData.append("file", file);
    formData.append("filename", filename);
    formData.append("is_private", isPrivate.toString());
    fetch("/api/file/create", {
      method: "POST",
      body: formData,
    })
      .then((response) => response.json())
      .then((data) => {
        setFilename("");
        console.log(data);
        setNeedsUpdate((prev) => {
          return prev + 1;
        });
        if (!fileInput.current) return;
        fileInput.current.value = "";
      })
      .catch((error) => {
        console.error(error);
      });
  };

  return (
    <form className="upload-form" onSubmit={handleSubmit}>
      <input type="file" ref={fileInput} onChange={handleFileChange} />
      <input
        type="text"
        value={filename}
        onChange={(e) => setFilename(e.target.value)}
        placeholder="Filename"
      />
      <label>
        Private
        <input
          type="checkbox"
          checked={isPrivate}
          onChange={(e) => setIsPrivate(e.target.checked)}
        />
      </label>
      <button type="submit">Upload File</button>
    </form>
  );
}

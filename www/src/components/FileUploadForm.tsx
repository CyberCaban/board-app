"use client";
import { useRef, useState } from "react";
import { Button } from "./ui/button";
import { Input } from "./ui/input";
import { Label } from "./ui/label";
import { toast } from "sonner";
import { postFormData } from "@/utils/utils";

interface IFileUploadFormProps {
  refetch: () => void;
}

export default function FileUploadForm({ refetch }: IFileUploadFormProps) {
  const fileInput = useRef<HTMLInputElement>(null);
  const [filename, setFilename] = useState<string>("");
  const [isPrivate, setIsPrivate] = useState<boolean>(false);

  const handleFileChange = () => {
    if (fileInput.current?.files?.length === 0) return;
    if (!fileInput.current || !fileInput.current.files) return;
    const file = fileInput.current.files?.[0];
    const nameWithoutExt = file.name.split(".")[0];
    setFilename(nameWithoutExt);
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!fileInput.current || !fileInput.current?.files?.length) return;
    const file = fileInput.current.files?.[0];
    const formData = new FormData();
    formData.append("file", file);
    formData.append("filename", filename);
    formData.append("is_private", isPrivate.toString());
    postFormData("/api/file/create", formData)
      .then(() => {
        setFilename("");
        if (fileInput.current) fileInput.current.value = "";
        refetch();
        toast.success("File uploaded successfully");
      })
      .catch((error) => {
        toast.error(`Error uploading file: ${error.message}`);
        console.error(error);
      });
  };

  return (
    <form className="upload-form" onSubmit={handleSubmit}>
      <h1>Upload File</h1>
      <Input type="file" ref={fileInput} onChange={handleFileChange} />
      <Input
        type="text"
        value={filename}
        onChange={(e) => setFilename(e.target.value)}
        placeholder="Filename"
      />
      <div className="flex flex-row align-start items-center h-6">
        <Label htmlFor="private" className="text-base">
          Private
        </Label>
        <Input
          id="private"
          className="ml-4 w-6"
          type="checkbox"
          checked={isPrivate}
          onChange={(checked) => setIsPrivate(checked.target.checked)}
        />
      </div>
      <Button type="submit">Upload File</Button>
    </form>
  );
}

"use client";
import { useRef, useState } from "react";
import { Label } from "../ui/label";
import { Button } from "../ui/button";
import { Input } from "../ui/input";
import { sentFileToBackend } from "@/lib/queries";
import { toast } from "sonner";
import { MAX_FILE_SIZE } from "@/lib/constants";

interface IFileUploadFormProps {
  onSuccess?: (refName: string) => void;
  onFailure?: (error: unknown) => void;
}

export default function FileUploadForm({
  onSuccess,
  onFailure,
}: IFileUploadFormProps) {
  const fileInput = useRef<HTMLInputElement>(null);
  const [filename, setFilename] = useState<string>("");
  const [isPrivate, setIsPrivate] = useState<boolean>(false);

  const handleFileChange = () => {
    if (fileInput.current?.files?.length === 0) return;
    if (!fileInput.current || !fileInput.current.files) return;
    const file = fileInput.current.files?.[0];
    setFilename(file.name);
  };

  const handleSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (!fileInput.current || !fileInput.current?.files?.length) return;
    const file = fileInput.current.files?.[0];
    if (file.size > MAX_FILE_SIZE) {
      toast.error("Maximum file size exceeded");
      return;
    }
    sentFileToBackend(
      file,
      filename,
      isPrivate,
      (refName) => {
        setFilename("");
        if (fileInput.current) fileInput.current.value = "";

        if (onSuccess) onSuccess(refName);
      },
      (error) => {
        if (onFailure) onFailure(error);
      },
    );
  };

  return (
    <form
      className="upload-form"
      onSubmit={handleSubmit}
    >
      <h1>Upload File</h1>
      <Input type="file" ref={fileInput} onChange={handleFileChange} />
      <Input
        type="text"
        value={filename}
        onChange={(e) => setFilename(e.target.value)}
        placeholder="Filename"
      />
      <div className="align-start flex h-6 flex-row items-center">
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

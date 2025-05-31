"use client";

import type React from "react";

import * as VisuallyHidden from "@radix-ui/react-visually-hidden";
import {
  Dialog,
  DialogContent,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { useRef, useState } from "react";
import { sentFileToBackend } from "@/lib/queries";
import { Input } from "../ui/input";
import { Button } from "../ui/button";
import { MAX_FILE_SIZE } from "@/lib/constants";
import { toast } from "sonner";

interface FileUploadDialogProps {
  isOpen: boolean;
  onOpenChange: (open: boolean) => void;
  onUpload: (fileName: string) => void;
}

export function FileUploadDialog({
  isOpen,
  onOpenChange,
  onUpload,
}: FileUploadDialogProps) {
  const fileInput = useRef<HTMLInputElement>(null);
  const [filename, setFilename] = useState<string>("");

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
    console.log(file);
    if (file.size > MAX_FILE_SIZE) {
      toast.error("Maximum file size exceeded");
      return;
    }

    sentFileToBackend(
      file,
      filename,
      true,
      (refName) => {
        setFilename("");
        if (fileInput.current) fileInput.current.value = "";

        onUpload(refName);
      },
      (error) => {
        console.error(error);
      },
    );
  };

  return (
    <Dialog open={isOpen} onOpenChange={onOpenChange}>
      <DialogContent className="border-border bg-background text-foreground">
        <DialogHeader>
          <VisuallyHidden.Root>
            <DialogTitle>Upload File</DialogTitle>
          </VisuallyHidden.Root>
        </DialogHeader>
        <form className="upload-form" onSubmit={handleSubmit}>
          <h1>Upload File</h1>
          <Input type="file" ref={fileInput} onChange={handleFileChange} />
          <Input
            type="text"
            value={filename}
            onChange={(e) => setFilename(e.target.value)}
            placeholder="Filename"
          />
          <Button type="submit">Upload File</Button>
        </form>
      </DialogContent>
    </Dialog>
  );
}

"use client";

import { Input } from "@/components/ui/input";
import { postFormData } from "@/utils/utils";
import { useRef } from "react";

interface IFileUploadFormProps {
  board_id: string;
  card_id: string;
  update: () => void;
}

export default function UploadAttachment(props: IFileUploadFormProps) {
  const fileInputRef = useRef<HTMLInputElement>(null);
  const handleFileChange = () => {
    if (!fileInputRef.current || !fileInputRef.current?.files?.length) return;
    const file = fileInputRef.current.files?.[0];
    const formData = new FormData();
    formData.append("file", file);
    formData.append("filename", file.name);
    postFormData(`/boards/${props.board_id}/cards/${props.card_id}/attachments`, formData)
      .then(() => {
        if (fileInputRef.current) fileInputRef.current.value = "";
        props.update();
      })
      .catch((error) => console.error(error));
  };
  return (
    <>
      <Input type="file" ref={fileInputRef} onChange={handleFileChange} />
    </>
  );
}

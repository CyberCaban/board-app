"use client";
import { Button } from "@/components/ui/button";
import { ICardAttachment } from "@/types";
import { isImage } from "@/utils/utils";
import { X } from "lucide-react";
import Image from "next/image";

interface DisplayAttachmentsProps {
  attachments: ICardAttachment[] | undefined;
  handleDelete: (attachment: string) => void;
}

export default function DisplayAttachments({
  attachments,
  handleDelete,
}: DisplayAttachmentsProps) {
  return (
    <>
      {attachments &&
        attachments.map((attachment) => {
          const url = `/uploads/${attachment.url}`;
          
          if (!isImage(url)) {
            return (
              <div
                className="group relative h-auto w-full rounded-md border-2 border-white p-4"
                key={attachment.id}
              >
                <a href={url} target="_blank" rel="noopener noreferrer">
                  <h3 className="truncate">{attachment.url}</h3>
                </a>
                <Button
                  className="absolute right-0 top-0 rounded-full px-2 py-0.5 text-red-500 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
                  onClick={() => handleDelete(attachment.id)}
                >
                  <X />
                </Button>
              </div>
            );
          }
          return (
            <div className="group relative h-auto w-full" key={attachment.id}>
              <a
                href={url}
                target="_blank"
                className=""
                rel="noopener noreferrer"
              >
                <Image
                  src={url}
                  alt="Uploaded Image"
                  loading="lazy"
                  width={200}
                  height={200}
                  className="rounded-xl shadow transition-all duration-200 hover:scale-105 group-hover:-translate-y-1"
                />
              </a>
              <Button
                className="absolute right-0 top-0 rounded-full px-2 py-0.5 text-red-500 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
                onClick={() => handleDelete(attachment.id)}
              >
                <X />
              </Button>
            </div>
          );
        })}
    </>
  );
}

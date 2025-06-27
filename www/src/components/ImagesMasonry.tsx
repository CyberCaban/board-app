"use client";
import { isImage } from "@/utils/utils";
import Image from "next/image";
import { X } from "lucide-react";
import Link from "next/link";
import { IFileView } from "@/types";

interface MasonryImageProps {
  imagesURL: IFileView[];
  handleDelete: (id: string) => void;
  signedIn: boolean;
}

export default function ImagesMasonry({
  imagesURL,
  handleDelete,
  signedIn,
}: MasonryImageProps) {
  function DeleteButton({ signedIn, url }: { signedIn: boolean; url: string }) {
    return (
      <>
        {signedIn ? (
          <button
            className="absolute right-0 top-0 rounded-full p-0.5 text-red-500 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
            onClick={() => handleDelete(url)}
          >
            <X className="h-4 w-4" />
          </button>
        ) : null}
      </>
    );
  }
  return (
    <div className="max-w-full">
      <h1>Images</h1>
      <div className="columns-2 gap-4 space-y-4 sm:columns-3 md:columns-3 lg:columns-5 xl:columns-6">
        {imagesURL &&
          imagesURL.map((fileView) => {
            const url = fileView.url;
            if (!isImage(url)) {
              return (
                <div
                  className="group relative h-auto w-full break-inside-avoid-column rounded-md border-2 border-foreground p-4"
                  key={url}
                >
                  <Link
                    href={`/img${url}`}
                  // target="_blank"
                  // rel="noopener noreferrer"
                  // download
                  >
                    <h3 className="line-clamp-5 text-ellipsis break-words">
                      {url.split("/").pop()}
                    </h3>
                  </Link>
                  <DeleteButton url={url} signedIn={signedIn} />
                </div>
              );
            }
            return (
              <div
                className="group relative h-auto w-full break-inside-avoid-column"
                key={url}
              >
                <Link
                  href={`/img${url}`}
                  // target="_blank"
                  className=""
                // rel="noopener noreferrer"
                >
                  <Image
                    src={`/u${url}`}
                    alt={url}
                    loading="lazy"
                    width={200}
                    height={200}
                    className="min-w-full rounded-xl shadow transition-all duration-200 hover:scale-105 group-hover:-translate-y-1"
                  />
                </Link>
                <DeleteButton url={url} signedIn={signedIn} />
              </div>
            );
          })}
      </div>
    </div>
  );
}

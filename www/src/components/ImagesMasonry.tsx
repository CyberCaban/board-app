"use client";
import { useEffect, useState } from "react";
import { deleteData, getData } from "../utils/utils";
import { IFile } from "@/types";
import Image from "next/image";

export default function ImagesMasonry() {
  const [imagesURL, setImagesURL] = useState<string[]>([]);

  useEffect(() => {
    getData("/api/files").then((res) => {
      console.log("getData", res);
      setImagesURL(
        res.map((file: IFile) => {
          if (file.private) return `/uploads/${file.user_id}/${file.name}`;
          return `/uploads/${file.name}`;
        })
      );
    });
  }, []);

  const handleDelete = (image: string) => {
    const filename = image.split("/").pop()!;
    deleteData(`/api/file/${filename}`).then((res) => {
      setImagesURL(imagesURL.filter((img) => img !== image));
      console.log(res);
    });
  };

  return (
    <div>
      <h1>Images</h1>
      <div className="columns-4 md:columns-6 gap-4 space-y-4">
        {imagesURL &&
          imagesURL.map((image) => (
            <div className="w-full h-auto relative group" key={image}>
              <a
                href={image}
                target="_blank"
                className=""
                rel="noopener noreferrer"
              >
                <Image
                  src={image}
                  alt="Uploaded Image"
                  loading="lazy"
                  width={500}
                  height={500}
                  className="rounded-xl shadow group-hover:-translate-y-1 hover:scale-105 transition-all duration-200"
                />
              </a>
              <button
                className="absolute top-0 right-0 px-2 py-0.5 text-red-500 rounded-full opacity-0 group-hover:opacity-100 transition-opacity duration-200"
                onClick={() => handleDelete(image)}
              >
                X
              </button>
            </div>
          ))}
      </div>
    </div>
  );
}

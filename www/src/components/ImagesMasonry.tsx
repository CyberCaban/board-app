"use client";
import Image from "next/image";
// import Link from "next/link";

interface MasonryImageProps {
  imagesURL: string[];
  handleDelete: (id: string) => void;
}

export default function ImagesMasonry({
  imagesURL,
  handleDelete,
}: MasonryImageProps) {
  return (
    <div>
      <h1>Images</h1>
      <div className="columns-4 gap-4 space-y-4 md:columns-6">
        {imagesURL &&
          imagesURL.map((image) => {
            if (
              !image.endsWith(".jpg") &&
              !image.endsWith(".png") &&
              !image.endsWith(".jpeg")
            ) {
              return (
                <div
                  className="group relative h-auto w-full rounded-md border-2 border-white p-4"
                  key={image}
                >
                  <a href={image} target="_blank" rel="noopener noreferrer">
                    <h3>{image.split("/").pop()}</h3>
                  </a>
                  <button
                    className="absolute right-0 top-0 rounded-full px-2 py-0.5 text-red-500 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
                    onClick={() => handleDelete(image)}
                  >
                    X
                  </button>
                </div>
              );
            }
            return (
              <div className="group relative h-auto w-full" key={image}>
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
                    width={200}
                    height={200}
                    className="rounded-xl shadow transition-all duration-200 hover:scale-105 group-hover:-translate-y-1"
                  />
                </a>
                <button
                  className="absolute right-0 top-0 rounded-full px-2 py-0.5 text-red-500 opacity-0 transition-opacity duration-200 group-hover:opacity-100"
                  onClick={() => handleDelete(image)}
                >
                  X
                </button>
              </div>
            );
          })}
      </div>
    </div>
  );
}

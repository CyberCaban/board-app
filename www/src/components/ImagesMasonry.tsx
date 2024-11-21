"use client";
import Image from "next/image";
import Link from "next/link";

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
      <div className="columns-4 md:columns-6 gap-4 space-y-4">
        {imagesURL &&
          imagesURL.map((image) => {
            if (
              !image.endsWith(".jpg") &&
              !image.endsWith(".png") &&
              !image.endsWith(".jpeg")
            ) {
              return (
                <div className="w-full h-auto relative group border-white border-2 rounded-md p-4" key={image}>
                  <Link href={image} target="_blank" rel="noopener noreferrer">
                    <h3>{image.split("/").pop()}</h3>
                  </Link>
                </div>
              );
            }
            return (
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
            );
          })}
      </div>
    </div>
  );
}

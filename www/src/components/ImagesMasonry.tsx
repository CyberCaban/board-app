import { useEffect, useState } from "react";
import { getData } from "../utils/utils";
import { IFile } from "../App";
import { masonryNeedsUpdate } from "../store";
import { useAtom } from "jotai";

export default function ImagesMasonry() {
  const [images, setImages] = useState<string[]>([]);
  const [needsUpdate] = useAtom(masonryNeedsUpdate);

  useEffect(() => {
    getData("/api/files")
      .then((res) => {
        if (res.error_msg) {
          throw new Error(res.error_msg);
        }
        setImages(res.map((file: IFile) => `/uploads/${file.name}`));
      })
      .catch((error) => {
        console.error(error);
      });
  }, [needsUpdate]);

  return (
    <div>
      <h1>Images</h1>
      <div className="columns-4 md:columns-6 gap-4 space-y-4">
        {images &&
          images.map((image) => (
            <img
              src={image}
              key={image}
              className="w-full h-auto rounded-xl shadow hover:-translate-y-1 hover:scale-105 transition-all duration-200"
            />
          ))}
      </div>
    </div>
  );
}

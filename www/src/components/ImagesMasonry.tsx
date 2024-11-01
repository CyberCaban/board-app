import { useEffect, useState } from "react";
import { getData } from "../utils/utils";
import { IFile } from "../App";

export default function ImagesMasonry() {
  const [images, setImages] = useState<string[]>([]);
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
  }, []);

  return (
    <div>
      <h1>Images</h1>
      <div className="columns-4 md:columns-6 gap-4 space-y-4">
        {images &&
          images.map((image) => (
            <img
              src={image}
              key={image}
              className="w-full h-auto rounded-xl shadow"
            />
          ))}
      </div>
    </div>
  );
}

"use client";
import { useEffect, useState } from "react";
import FileUploadForm from "./FileUploadForm";
import ImagesMasonry from "./ImagesMasonry";
import { deleteData, getData } from "@/utils/utils";
import { IFile } from "@/types";
import { useUserStore } from "@/providers/userProvider";
import { toast } from "sonner";

export default function ImagesAndUpload() {
  const [imagesURL, setImagesURL] = useState<string[]>([]);
  const [, store] = useUserStore((state) => state);

  const fetchImages = () => {
    getData("/api/files").then((res) => {
      setImagesURL(
        res.map((file: IFile) => {
          if (file.private) return `/uploads/${file.user_id}/${file.name}`;
          return `/uploads/${file.name}`;
        })
      );
    });
  };

  useEffect(() => {
    fetchImages();
    const unsubscribe = store.subscribe(() => {
      fetchImages();
    });

    return unsubscribe;
  }, [store]);

  const handleDelete = (image: string) => {
    const filename = image.split("/").pop()!;
    deleteData(`/api/file/${filename}`)
      .then(() => {
        setImagesURL(imagesURL.filter((img) => img !== image));
        toast.success("Image deleted successfully");
      })
      .catch((err) => toast.error(err.message));
  };

  return (
    <>
      <FileUploadForm refetch={fetchImages} />
      <ImagesMasonry imagesURL={imagesURL} handleDelete={handleDelete} />
    </>
  );
}

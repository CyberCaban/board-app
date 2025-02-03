"use client";
import { useEffect, useState } from "react";
import ImagesMasonry from "./ImagesMasonry";
import { deleteData, getData } from "@/utils/utils";
import { IFile } from "@/types";
import { useUserStore } from "@/providers/userProvider";
import { toast } from "sonner";
import Link from "next/link";
import FileUploadForm from "./forms/FileUploadForm";

export default function ImagesAndUpload() {
  const [imagesURL, setImagesURL] = useState<string[]>([]);
  const [store, internalStore] = useUserStore((state) => state);

  const getId = () => store.id;

  const fetchImages = () => {
    getData("/api/files").then((res) => {
      setImagesURL(
        res.map((file: IFile) => {
          if (file.private) return `/uploads/${file.user_id}/${file.name}`;
          return `/uploads/${file.name}`;
        }),
      );
    });
  };

  function UploadComponent() {
    return (
      <>
        {getId() ? (
          <FileUploadForm refetch={fetchImages} />
        ) : (
          <h3>
            <Link className="underline" href={"/login"}>
              Sign In
            </Link>{" "}
            to upload files
          </h3>
        )}
      </>
    );
  }

  useEffect(() => {
    fetchImages();
    const unsubscribe = internalStore.subscribe(() => {
      fetchImages();
    });

    return unsubscribe;
  }, [internalStore]);

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
      <UploadComponent />
      <ImagesMasonry
        imagesURL={imagesURL}
        handleDelete={handleDelete}
        signedIn={!!getId()}
      />
    </>
  );
}

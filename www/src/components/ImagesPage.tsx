"use client";
import { useEffect, useState } from "react";
import ImagesMasonry from "./ImagesMasonry";
import { deleteData, getData } from "@/utils/utils";
import { IFile, IFileView } from "@/types";
import { useUserStore } from "@/providers/userProvider";
import { toast } from "sonner";
import Link from "next/link";
import FileUploadForm from "./forms/FileUploadForm";

export default function ImagesAndUpload() {
  const [imagesURL, setImagesURL] = useState<IFileView[]>([]);
  const [store, internalStore] = useUserStore((state) => state);

  const getId = () => store.id;

  const fetchImages = () => {
    getData("/api/files").then((res) => {

      setImagesURL(
        res.map((file: IFile) => {

          const url = file.private
            ? `/${file.user_id}/${file.name}`
            : `/${file.name}`;
          console.log(url);
          return {
            url,
            user_id: file.user_id,
          } satisfies IFileView;
        }),
      );
    });
  };

  function UploadComponent() {
    return (
      <>
        {getId() ? (
          <FileUploadForm
            onSuccess={() => {
              fetchImages();
              toast.success("File uploaded successfully!");
            }}
            onFailure={(error) => {
              toast.error(`Error uploading file: ${error}`);
              console.error(error);
            }}
          />
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

  const handleDelete = (url: string) => {
    const filename = url.split("/").pop()!;
    deleteData(`/api/file/${filename}`)
      .then(() => {
        setImagesURL(imagesURL.filter((img) => img.url !== url));
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

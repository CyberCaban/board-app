import FileUploadForm from "@/components/FileUploadForm";
import ImagesMasonry from "@/components/ImagesMasonry";
import Navbar from "@/components/Navbar";

export default function Home() {
  return (
    <>
      <Navbar />
      <main className="flex min-h-screen flex-col items-center p-24">
        <FileUploadForm />
        <ImagesMasonry />
      </main>
    </>
  );
}

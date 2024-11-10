"use client";
import ImagesAndUpload from "@/components/ImagesPage";
import Navbar from "@/components/Navbar";

export default function Home() {
  return (
    <>
      <Navbar />
      <main className="flex min-h-screen flex-col items-center p-24">
        <ImagesAndUpload />
      </main>
    </>
  );
}

import FileUploadForm from "@/components/FileUploadForm";
import ImagesMasonry from "@/components/ImagesMasonry";
import LoginForm from "@/components/LoginForm";
import RegisterForm from "@/components/RegisterForm";

export default function Home() {
  return (
    <main className="flex min-h-screen flex-col items-center p-24">
      <RegisterForm />
      <LoginForm />
      <ImagesMasonry />
      <FileUploadForm />
    </main>
  );
}

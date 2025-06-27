import type { Metadata } from "next";
import localFont from "next/font/local";
import "./globals.css";
import { UserStoreProvider } from "@/providers/userProvider";
import { Toaster } from "sonner";
import Navbar from "@/components/Navbar";
import { ThemeProvider } from "next-themes";

const geistMono = localFont({
  src: "./fonts/GeistMonoVF.woff",
  variable: "--font-geist-mono",
  weight: "100 900",
});

export const metadata: Metadata = {
  title: "Board-app",
  description: "Beautiful App",
};

export default function RootLayout({
  children,
  modal
}: Readonly<{
  children: React.ReactNode;
  modal: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body className={`${geistMono.variable} ${geistMono.className} bg-background text-foreground`}>
        <ThemeProvider attribute="class" defaultTheme="dark" enableSystem>
          <UserStoreProvider>
            <Navbar />
            {children}
            {modal}
          </UserStoreProvider>
          <Toaster />
        </ThemeProvider>
      </body>
    </html>
  );
}

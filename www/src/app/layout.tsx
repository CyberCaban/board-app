import type { Metadata } from "next";
import localFont from "next/font/local";
import "./globals.css";
import { UserStoreProvider } from "@/providers/userProvider";
import { Toaster } from "sonner";
import Navbar from "@/components/Navbar";

const geistSans = localFont({
  src: "./fonts/GeistVF.woff",
  variable: "--font-geist-sans",
  weight: "100 900",
});
const geistMono = localFont({
  src: "./fonts/GeistMonoVF.woff",
  variable: "--font-geist-mono",
  weight: "100 900",
});

export const metadata: Metadata = {
  title: "Board",
  description: "",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en">
      <body
        className={`${geistSans.variable} ${geistMono.variable} antialiased`}
      >
        <UserStoreProvider>
          <Navbar />
          {children}
        </UserStoreProvider>
        <Toaster />
      </body>
    </html>
  );
}

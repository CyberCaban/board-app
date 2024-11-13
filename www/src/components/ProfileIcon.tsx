"use client";
import { useUserStore } from "@/providers/userProvider";
import Link from "next/link";
import Image from "next/image";

export default function Profile() {
  const [state] = useUserStore((state) => state);

  return (
    <Link className="flex flex-row items-center" href="/profile">
      {/* <Button className="mr-4" onClick={state.logout}>Logout</Button> */}
      <Image
        src={state.profile_url}
        alt="Profile"
        width={50}
        height={50}
        className="rounded-full"
      />
    </Link>
  );
}
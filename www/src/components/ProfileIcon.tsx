"use client";
import { useUserStore } from "@/providers/userProvider";
import Link from "next/link";
import Image from "next/image";
import userSVG from "@/../public/user.svg";

export default function Profile() {
  const [state] = useUserStore((state) => state);

  return (
    <>
      {state.id ? (
        <Link className="flex flex-row items-center" href="/profile">
          {/* <Button className="mr-4" onClick={state.logout}>Logout</Button> */}
          <Image
            src={state.profile_url || userSVG}
            alt="Profile"
            width={50}
            height={50}
            className="rounded-full"
          />
        </Link>
      ) : null}
    </>
  );
}

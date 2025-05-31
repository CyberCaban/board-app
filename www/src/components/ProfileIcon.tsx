"use client";
import { useUserStore } from "@/providers/userProvider";
import Link from "next/link";
import { Avatar, AvatarFallback, AvatarImage } from "./ui/avatar";

export default function Profile() {
  const [state] = useUserStore((state) => state);

  return (
    <>
      {state.id ? (
        <Link className="flex flex-row items-center" href="/profile">
          {/* <Button className="mr-4" onClick={state.logout}>Logout</Button> */}
          <Avatar>
            <AvatarImage width={50} height={50} src={state.profile_url} />
            <AvatarFallback>{state.username[0]}</AvatarFallback>
          </Avatar>
        </Link>
      ) : null}
    </>
  );
}

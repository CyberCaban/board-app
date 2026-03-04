"use client";
import { useUserStore } from "@/providers/userProvider";
import Link from "next/link";
import { Avatar, AvatarFallback, AvatarImage, AvatarBadge } from "./ui/avatar";
import { useCentrifugo } from "@/providers/centrifugoProvider";

export default function Profile() {
  const [state] = useUserStore((state) => state);
  const {connectionState} = useCentrifugo();

  return (
    <>
      {state.id ? (
        <Link className="flex flex-row items-center" href="/profile">
          {/* <Button className="mr-4" onClick={state.logout}>Logout</Button> */}
          <Avatar>
            <AvatarImage width={50} height={50} src={state.profile_url} />
            <AvatarFallback>{state.username[0]}</AvatarFallback>
            {connectionState === "connected" && <AvatarBadge className="bg-green-600 dark:bg-green-800" />}
            {connectionState !== "connected" && <AvatarBadge className="bg-red-600 dark:bg-red-800" />}
          </Avatar>
        </Link>
      ) : null}
    </>
  );
}

"use client";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { IPubUser } from "@/types";
import { getData, postData } from "@/utils/utils";
import { useEffect, useState } from "react";
import { toast } from "sonner";
import Image from "next/image";
import Link from "next/link";

export default function FriendCodeSection() {
  const [code, setCode] = useState("");
  const [myCode, setMyCode] = useState("");
  const [friends, setFriends] = useState<IPubUser[]>([]);

  const generateCode = async () => {
    postData("/friends/code", {}).then((res) => {
      setMyCode(res.code);
    });
  };

  const redeemCode = async () => {
    postData("/friends/redeem", code).then((res) => {
      if (res.ok) {
        toast.success("Friend added successfully!");
        setCode("");
      }
    });
  };

  useEffect(() => {
    getData("/friends/list").then((res) => {
      setFriends(res);
    });
  }, []);

  return (
    <div className="space-y-4">
      <div>
        <h3 className="text-lg font-semibold">Your Friend Code</h3>
        <div className="flex gap-2">
          <Input value={myCode} readOnly className="text-black" />
          <Button onClick={generateCode}>
            {myCode ? "Refresh Code" : "Generate Code"}
          </Button>
        </div>
      </div>

      <div>
        <h3 className="text-lg font-semibold">Add Friend</h3>
        <div className="flex gap-2">
          <Input
            placeholder="Enter friend's code"
            value={code}
            onChange={(e) => setCode(e.target.value.toUpperCase())}
          />
          <Button onClick={redeemCode}>Add Friend</Button>
        </div>
      </div>

      <div>
        <h3 className="text-lg font-semibold">Friends</h3>
        <div className="flex gap-2">
          {friends.map((friend) => (
            <Link key={friend.id} href={`/chat/${friend.id}`}>
              <div className="group flex flex-row items-center gap-2 rounded-md p-4 hover:bg-gray-900">
                <Image
                  src={friend.profile_url}
                  alt={friend.username}
                  width={100}
                  height={100}
                  className="rounded-full transition-all duration-300 group-hover:scale-110"
                />
                <div className="ml-4 flex flex-col gap-2 text-sm transition-all duration-300">
                  <div>{friend.username}</div>
                  <div>{friend.bio}</div>
                </div>
              </div>
            </Link>
          ))}
        </div>
      </div>
    </div>
  );
}

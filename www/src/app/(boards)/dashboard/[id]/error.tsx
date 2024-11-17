"use client";

import Link from "next/link";

export default function Error({ error }: { error: Error }) {
  return (
    <div>
      <h2>Something went wrong!</h2>
      <p>{error.message}</p>
      <Link href="/boards">Back to boards</Link>
    </div>
  );
}

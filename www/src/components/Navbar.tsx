import { cookies } from "next/headers";
import Link from "next/link";
import Profile from "./Profile";

export default async function Navbar() {
  const cookieStore = await cookies();
  const token = cookieStore.get("token");

  return (
    <nav className="flex flex-row bg-background p-4 gap-4 justify-between">
      <div className="left_pad flex flex-row gap-4">
        <Link href="/">Home</Link>
        <Link href="/register">Register</Link>
        <Link href="/login">Login</Link>
      </div>
      <div className="right_pad">{token && <Profile />}</div>
    </nav>
  );
}

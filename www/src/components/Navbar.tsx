import Link from "next/link";
import Profile from "./ProfileIcon";
import SignedOut from "./authGuards/SignedOut";
import SignedIn from "./authGuards/SignedIn";
import { ThemeToggle } from "./ThemeToggle";

export default function Navbar() {
  return (
    <nav className="flex flex-row justify-between gap-4 p-4">
      <div className="left_pad flex flex-row gap-4">
        <Link href="/">Home</Link>
        <SignedOut>
          <Link href="/register">Register</Link>
          <Link href="/login">Login</Link>
        </SignedOut>
        <SignedIn>
          <Link href="/board">Boards</Link>
          <Link href="/friends">Friends</Link>
        </SignedIn>
      </div>
      <div className="right_pad flex flex-row items-center gap-4">
        <ThemeToggle />
        <Profile />
      </div>
    </nav>
  );
}

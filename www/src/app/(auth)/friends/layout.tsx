import AuthGuardRedirect from "@/components/authGuards/AuthGuardRedirect";

export default function Friends({ children }: { children: React.ReactNode }) {
  return <AuthGuardRedirect>{children}</AuthGuardRedirect>;
}

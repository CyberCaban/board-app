import { getData } from "@/utils/utils";
import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import { api_url } from "../../../next.config";

export default async function AuthGuardRedirect({
  children,
}: {
  children: React.ReactNode;
}) {
  const cookie = await cookies();
  const token = cookie.get("token")?.value;
  if (!token) redirect("/");
  await getData(`${api_url}/api/user`, {
    headers: {
      "Content-Type": "application/json",
      Authorization: `Bearer ${token}`,
    },
  }).catch(() => redirect("/"));

  return <>{children}</>;
}

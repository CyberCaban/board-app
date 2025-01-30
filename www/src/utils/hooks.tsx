import { useUserStore } from "@/providers/userProvider";
import { redirect } from "next/navigation";
import { useEffect } from "react";
import { toast } from "sonner";

export function useUnauthorized() {
  const [store] = useUserStore((s) => s);
  useEffect(() => {
    if (!store.id) {
      toast.warning("Unauthorized");
      redirect("/");
    }
  }, [store.id]);
}

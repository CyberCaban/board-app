import { useUserStore } from "@/providers/userProvider";
import { redirect } from "next/navigation";
import { useEffect } from "react";
import { toast } from "sonner";

export function useUnauthorized() {
  const [store] = useUserStore((s) => s);
  useEffect(() => {
    const f = () => {
      if (!store.id) {
        toast.warning("Unauthorized");
        redirect("/");
      }
    };
    const s = setInterval(f, 1000);
    return () => clearInterval(s);
  }, [store.id]);
}

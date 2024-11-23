"use client";

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogOverlay,
  DialogTitle,
} from "@/components/ui/dialog";
import { useRouter } from "next/navigation";

export default function Modal({ children }: { children: React.ReactNode }) {
  const router = useRouter();
  const handleOpenChange = () => {
    router.back();
  };
  return (
    <Dialog defaultOpen open onOpenChange={handleOpenChange}>
      <DialogOverlay className="bg-black/20">
        <DialogTitle className="text-center" hidden></DialogTitle>
        <DialogContent className="overflow-y-hidden bg-black">
          {children}
          <DialogDescription className="text-center">
            Press escape key to close
          </DialogDescription>
        </DialogContent>
      </DialogOverlay>
    </Dialog>
  );
}

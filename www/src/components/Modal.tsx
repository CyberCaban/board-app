"use client";

import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
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
        <DialogContent className="overflow-y-hidden bg-black">
          <DialogHeader>
            <DialogTitle>Change your card</DialogTitle>
            <DialogDescription>
              Anyone who has this link will be able to view this.
            </DialogDescription>
          </DialogHeader>
          {children}
          <DialogDescription className="text-center">
            Press escape key to close
          </DialogDescription>
        </DialogContent>
      </DialogOverlay>
    </Dialog>
  );
}

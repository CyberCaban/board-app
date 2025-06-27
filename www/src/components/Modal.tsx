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
import React from "react";

interface ModalProps {
  children: React.ReactNode;
  title?: React.ReactNode;
  headerDesc?: React.ReactNode;
  footerDesc?: React.ReactNode;
}

export default function Modal({
  children,
  title,
  headerDesc,
  footerDesc,
}: ModalProps) {
  const router = useRouter();
  const handleOpenChange = () => {
    router.back();
  };
  return (
    <Dialog defaultOpen open onOpenChange={handleOpenChange}>
      <DialogOverlay className="bg-background/20 text-foreground">
        <DialogContent className="overflow-y-hidden bg-background">
          <DialogHeader>
            <DialogTitle>{title}</DialogTitle>
            <DialogDescription>{headerDesc}</DialogDescription>
          </DialogHeader>
          {children}
          {footerDesc ? (
            <DialogDescription className="text-center">
              {footerDesc}
            </DialogDescription>
          ) : null}
        </DialogContent>
      </DialogOverlay>
    </Dialog>
  );
}

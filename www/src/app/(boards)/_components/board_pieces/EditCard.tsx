"use client";
import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { useKanbanStore } from "@/providers/kanbanProvider";
import { zodResolver } from "@hookform/resolvers/zod";
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";
import { deleteData } from "@/utils/utils";
import { Label } from "@/components/ui/label";
import DisplayAttachments from "../DisplayAttachments";
import UploadAttachment from "../UploadAttachment";
const formSchema = z.object({
  cardName: z.string(),
  cardDescription: z.string(),
});

export default function EditCard({
  board_id,
  card_id,
}: {
  board_id: string;
  card_id: string;
}) {
  const [kstore] = useKanbanStore((state) => state);

  useEffect(() => {
    kstore.requestCardModal(board_id, card_id).catch((e) => {
      console.log(e);
      toast.error(e.message);
    });
    return () => kstore.resetCardModal();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    // defaultValues: {
    //   cardName: kstore.cardModal?.name,
    //   cardDescription: kstore.cardModal?.description,
    // },
    values: {
      cardName: kstore.cardModal?.name || "",
      cardDescription: kstore.cardModal?.description || "",
    },
  });

  function onSubmit(values: z.infer<typeof formSchema>) {
    try {
      if (!kstore.cardModal) return;
      if (document.startViewTransition) document.startViewTransition();
      kstore.updateCard(
        card_id,
        values.cardName,
        values.cardDescription,
        kstore.cardModal.column_id,
      );
    } catch (error) {
      console.error("Form submission error", error);
    }
  }

  const handleDelete = (attachment_id: string) => {
    deleteData(
      `/boards/${board_id}/cards/${card_id}/attachments/${attachment_id}`,
    )
      .then(() => kstore.requestCardModal(board_id, card_id))
      .catch((e) => toast.error(e.message));
  };

  const update = () => {
    kstore.requestCardModal(board_id, card_id).catch((e) => {
      console.log(e);
      toast.error(e.message);
    });
  };

  return (
    <div className="flex flex-col items-center">
      <div className="flex flex-col gap-4 max-w-md">
        <Label className="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
          Attachments
        </Label>
        <DisplayAttachments
          attachments={kstore.cardModal?.attachments}
          handleDelete={handleDelete}
        />
        <UploadAttachment
          board_id={board_id}
          card_id={card_id}
          update={update}
        />
      </div>
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          className="w-full max-w-2xl space-y-4"
        >
          <FormField
            control={form.control}
            name="cardName"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Card Name</FormLabel>
                <FormControl>
                  <Input
                    className="line-clamp-3"
                    placeholder="Enter card name"
                    type="text"
                    {...field}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="cardDescription"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Description</FormLabel>
                <FormControl>
                  <Textarea
                    className="h-40 resize-none px-4 py-2"
                    placeholder="Description"
                    {...field}
                  />
                </FormControl>
                <FormMessage />
              </FormItem>
            )}
          />
          <Button type="submit">Save</Button>
        </form>
      </Form>
    </div>
  );
}

"use client";

import { Button } from "@/components/ui/button";
import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";
import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { toast } from "sonner";
import { z } from "zod";
import { useUserBoardsStore } from "../../../providers/userBoardsProvider";

const formSchema = z.object({
  boardName: z.string().min(3).max(20),
});

export default function CreateBoardForm() {
  const ubstore = useUserBoardsStore((state) => state);
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      boardName: "",
    },
  });
  const onSubmit = (data: z.infer<typeof formSchema>) => {
    ubstore.addUserBoard(data.boardName).catch((e) => toast.error(e.message));
    form.reset();
  };

  return (
    <>
      <Form {...form}>
        <form onSubmit={form.handleSubmit(onSubmit)}>
          <FormField
            control={form.control}
            name="boardName"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Board Name</FormLabel>
                <FormControl>
                  <Input placeholder="Board Name" {...field} />
                </FormControl>
              </FormItem>
            )}
          />
          <Button type="submit">Submit</Button>
        </form>
      </Form>
    </>
  );
}

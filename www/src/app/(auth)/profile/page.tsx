"use client";
import { useUserStore } from "@/providers/userProvider";
import Image from "next/image";
import user from "../../../../public/user.svg";
import { Input } from "@/components/ui/input";
import { z } from "zod";
import { useForm } from "react-hook-form";
import { zodResolver } from "@hookform/resolvers/zod";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
} from "@/components/ui/form";
import { Button } from "@/components/ui/button";
import { toast } from "sonner";
import { putData } from "@/utils/utils";

const formSchema = z.object({
  username: z.string().min(3).max(20),
  oldPassword: z.string().min(8).max(20),
  newPassword: z.string().min(8).max(20),
  confirmPassword: z.string().min(8).max(20),
});

export default function Profile() {
  const [store] = useUserStore((state) => state);

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      username: store.username,
      oldPassword: "",
      newPassword: "",
      confirmPassword: "",
    },
  });
  const onSubmit = (data: z.infer<typeof formSchema>) => {
    const { username, oldPassword, newPassword, confirmPassword } = data;

    if (newPassword !== confirmPassword) {
      toast.error("Passwords do not match");
      return;
    }

    if (newPassword === oldPassword) {
      toast.error("New password cannot be the same as the old password");
      return;
    }

    putData("/api/user", {
      username,
      old_password: oldPassword,
      new_password: newPassword,
      profile_url: "",
      bio: "",
    })
      .then(() => {
        toast.success("Profile updated successfully");
        form.reset({
          username,
          oldPassword: "",
          newPassword: "",
          confirmPassword: "",
        });
      })
      .catch((err) => toast.error(err.message));
  };

  return (
    <main className="flex min-h-screen flex-col items-center p-24">
      <h1>Profile</h1>
      <Image
        src={store.profile_url || user}
        alt="Profile Image"
        width={100}
        height={100}
      />
      <Form {...form}>
        <form
          onSubmit={form.handleSubmit(onSubmit)}
          className="flex flex-col gap-2"
        >
          <FormField
            control={form.control}
            name="username"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Username</FormLabel>
                <FormControl>
                  <Input {...field} />
                </FormControl>
                <FormDescription>Change your username</FormDescription>
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="oldPassword"
            render={({ field }) => (
              <>
                <FormLabel>Old Password</FormLabel>
                <FormItem>
                  <FormControl>
                    <Input type="password" {...field} />
                  </FormControl>
                  <FormDescription>Enter your current password</FormDescription>
                </FormItem>
              </>
            )}
          />
          <FormField
            control={form.control}
            name="newPassword"
            render={({ field }) => (
              <FormItem>
                <FormLabel>New Password</FormLabel>
                <FormControl>
                  <Input type="password" {...field} />
                </FormControl>
                <FormDescription>Enter your new password</FormDescription>
              </FormItem>
            )}
          />
          <FormField
            control={form.control}
            name="confirmPassword"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Confirm Password</FormLabel>
                <FormControl>
                  <Input type="password" {...field} />
                </FormControl>
                <FormDescription>Confirm your new password</FormDescription>
              </FormItem>
            )}
          />
          <Button type="submit">Save</Button>
        </form>
      </Form>
    </main>
  );
}

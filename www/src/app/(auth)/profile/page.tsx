"use client";
import { useUserStore } from "@/providers/userProvider";
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
import { getData, putData, sanitizeProfileUrl } from "@/utils/utils";
import { useRouter } from "next/navigation";
import { Avatar, AvatarFallback, AvatarImage } from "@/components/ui/avatar";

// TODO: redirect on signin if not signed

const formSchema = z.object({
  username: z.string().min(3).max(20),
  profile_url: z.string(),
  // oldPassword: z.string().min(8).max(20),
  // newPassword: z.string().min(8).max(20),
  // confirmPassword: z.string().min(8).max(20),
});

export default function Profile() {
  const router = useRouter();
  const [store] = useUserStore((state) => {
    return state;
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      username: store.username,
      profile_url: store.profile_url,
      // oldPassword: "",
      // newPassword: "",
      // confirmPassword: "",
    },
    values: {
      username: store.username,
      profile_url: store.profile_url,
      // oldPassword: "",
      // newPassword: "",
      // confirmPassword: "",
    },
  });

  const onSubmit = (data: z.infer<typeof formSchema>) => {
    const {
      username,
      // oldPassword,
      // newPassword,
      // confirmPassword,
      profile_url,
    } = data;

    // if (newPassword !== confirmPassword) {
    //   toast.error("Passwords do not match");
    //   return;
    // }

    // if (newPassword === oldPassword) {
    //   toast.error("New password cannot be the same as the old password");
    //   return;
    // }

    putData("/api/user", {
      username,
      // old_password: oldPassword,
      // new_password: newPassword,
      old_password: "",
      new_password: "",
      profile_url: sanitizeProfileUrl(profile_url),
      bio: "",
    })
      .then(() => {
        getData("/api/user")
          .then((res) => {
            store.setUser({
              id: res.id,
              username: res.username,
              profile_url: sanitizeProfileUrl(res.profile_url),
            });
            console.log(res);
            form.reset({
              username: res.username,
              profile_url: sanitizeProfileUrl(res.profile_url),
            });
          })
          .catch((err) => {
            console.error(err);
            store.resetUser();
          });

        toast.success("Profile updated successfully");
      })
      .catch((err) => toast.error(err.message));
  };

  const onLogout = () => {
    store.logout();
    router.replace("/");
  };

  return (
    <>
      <h1>Profile</h1>
      <Avatar className="w-[100px] h-[100px]">
        <AvatarImage width={100} height={100} src={store.profile_url}/>
        <AvatarFallback>{store.username[0]}</AvatarFallback>
      </Avatar>
      <Button className="mt-4" onClick={onLogout}>
        Logout
      </Button>
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
            name="profile_url"
            render={({ field }) => (
              <FormItem>
                <FormLabel>Profile URL</FormLabel>
                <FormControl>
                  <Input {...field} />
                </FormControl>
                <FormDescription>Change your profile URL</FormDescription>
              </FormItem>
            )}
          />
          {/* <FormField
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
          /> */}
          <Button type="submit">Save</Button>
        </form>
      </Form>
    </>
  );
}

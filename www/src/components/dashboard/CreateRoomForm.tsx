"use client";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import {
  Form,
  FormControl,
  FormDescription,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from "@/components/ui/form";
import {
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { toast } from "@/components/ui/use-toast";
import {
  CreateRoomParamsSchema,
  CreateRoomParamsType,
} from "@/lib/client/room/create";
import { RoomTypeSchema } from "@/lib/types";
import { zodResolver } from "@hookform/resolvers/zod";
import { PlusCircle } from "lucide-react";
import { nanoid } from "nanoid";
import { useState } from "react";
import { SubmitHandler, useForm } from "react-hook-form";
import { Input } from "../ui/input";
const wait = () => new Promise((resolve) => setTimeout(resolve, 1000));

type CreateRoomFormProps = {
  createRoomQuery: (params: CreateRoomParamsType) => Promise<void>;
};

const CreateRoomForm = ({ createRoomQuery }: CreateRoomFormProps) => {
  const [open, setOpen] = useState(false);

  const defaultParams: CreateRoomParamsType = {
    id: `room-${nanoid()}`,
    defaultAccesses: ["room:write"],
    usersAccesses: {
      "hectorcrean@gmail.com": ["room:write"],
    },
    groupsAccesses: {},
    metadata: {
      type: "flow",
    },
  };

  const form = useForm<CreateRoomParamsType>({
    resolver: zodResolver(CreateRoomParamsSchema),
    defaultValues: defaultParams,
  });

  const onValidForm: SubmitHandler<CreateRoomParamsType> = async (data) => {
    await createRoomQuery(data);
    toast({
      title: "You submitted the following values:",
      description: (
        <pre className="mt-2 w-[340px] rounded-md bg-slate-950 p-4">
          <code className="text-white">{JSON.stringify(data, null, 2)}</code>
        </pre>
      ),
    });
    setOpen(false);
  };

  return (
    <Dialog open={open} onOpenChange={setOpen}>
      <DialogTrigger>
        <Button
          size="sm"
          className="h-8 gap-1"
          onPointerDown={() => {
            console.log("create room");
          }}
        >
          <PlusCircle className="h-3.5 w-3.5" />
          <span className="sr-only sm:not-sr-only sm:whitespace-nowrap">
            Add Room
          </span>
        </Button>
      </DialogTrigger>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Add room</DialogTitle>
          <DialogDescription>Add...</DialogDescription>
        </DialogHeader>

        <Form {...form}>
          <form
            onSubmit={form.handleSubmit(onValidForm)}
            className="w-2/3 space-y-6"
          >
            <FormField
              control={form.control}
              name="id"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Room Name</FormLabel>
                  <FormControl>
                    <Input placeholder={defaultParams["id"]} {...field} />
                  </FormControl>
                  <FormDescription>This is the Room id.</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            <FormField
              control={form.control}
              name="metadata.type"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Type</FormLabel>
                  <Select
                    onValueChange={field.onChange}
                    defaultValue={field.value}
                  >
                    <FormControl>
                      <SelectTrigger>
                        <SelectValue placeholder="Select a room type" />
                      </SelectTrigger>
                    </FormControl>
                    <SelectContent>
                      {Object.values(RoomTypeSchema.Values).map((roomType) => (
                        <SelectItem key={roomType} value={roomType}>
                          {roomType}
                        </SelectItem>
                      ))}
                    </SelectContent>
                  </Select>
                  <FormDescription></FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            />
            {/* <FormField
              control={form.control}
              name="defaultAccesses"
              render={({ field }) => (
                <FormItem>
                  <FormLabel>Default Accesses</FormLabel>
                  <FormControl>
                    <MultiSelect<PermissionEnumType>
                      selected={field.value}
                      onSelectedChange={(selected) =>
                        form.setValue("defaultAccesses", selected)
                      }
                      options={[
                        "room:write",
                        "room:read",
                        "room:presence:write",
                      ]}
                    />
                  </FormControl>
                  <FormDescription>Default Accesses</FormDescription>
                  <FormMessage />
                </FormItem>
              )}
            /> */}
            {/* <FormField
          control={form.control}
          name="groupsAccesses"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Group Accesses</FormLabel>
              <FormControl>
              </FormControl>
              <FormDescription>Group Accesses</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        /> */}
            {/* <FormField
          control={form.control}
          name="usersAccesses"
          render={({ field }) => (
            <FormItem>
              <FormLabel>Users Accesses</FormLabel>
              <FormControl>
              </FormControl>
              <FormDescription>Users Accesses</FormDescription>
              <FormMessage />
            </FormItem>
          )}
        /> */}

            <Button type="submit">Submit</Button>
          </form>
        </Form>
      </DialogContent>
    </Dialog>
  );
};

export { CreateRoomForm };

"use client";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import {
  DropdownMenu,
  DropdownMenuCheckboxItem,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger
} from "@/components/ui/dropdown-menu";
import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { TooltipProvider } from "@/components/ui/tooltip";
import { CreateRoomParamsType } from "@/lib/client/room/create";
import { DeleteRoomParamsType } from "@/lib/client/room/delete";
import { RoomsQueryResultType } from "@/lib/client/room/get";
import { RoomInfoType } from "@/lib/types";
import { File, ListFilter, MoreHorizontal } from "lucide-react";
import Image from "next/image";
import Link from "next/link";
import { useRouter } from "next/navigation";
import { useMemo } from "react";
import { CreateRoomForm } from "./CreateRoomForm";
import { Header } from "./Header";
import { DocumentFilter, docFilters } from "./types";

interface DashboardProps {
  rooms: RoomsQueryResultType;
  filters: Array<DocumentFilter>;
  createRoom: (params: CreateRoomParamsType) => Promise<RoomInfoType>;
  deleteRoom: (params: DeleteRoomParamsType) => Promise<RoomInfoType>;
}
const Dashboard = ({ rooms, filters, createRoom, deleteRoom }: DashboardProps) => {
  const filter = useMemo(() => filters?.[0] ?? "all", [filters]);


  const router = useRouter();

  return (
    <TooltipProvider>
      <div className="flex flex-col w-full min-h-screen bg-muted/40">
        {/* <Aside /> */}
        <div className="flex flex-col sm:gap-4 sm:py-4 sm:pl-14">
          <Header filter={filter} />
          <main className="grid items-start flex-1 gap-4 p-4 sm:px-6 sm:py-0 md:gap-8">
            <Tabs defaultValue={"all"} value={filter}>
              <div className="flex items-center">
                <TabsList>
                  {Object.keys(docFilters).map((key) => (
                    <TabsTrigger key={key} value={docFilters.all.id}>
                      <Link
                        href={`${key}`}
                        className="flex items-center justify-center w-12 transition-colors rounded-lg h-9 text-muted-foreground hover:text-foreground md:h-8 md:w-8"
                      >
                        <span>{key}</span>
                      </Link>
                    </TabsTrigger>
                  ))}
                </TabsList>
                <div className="flex items-center gap-2 ml-auto">
                  <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                      <Button variant="outline" size="sm" className="h-8 gap-1">
                        <ListFilter className="h-3.5 w-3.5" />
                        <span className="sr-only sm:not-sr-only sm:whitespace-nowrap">
                          Filter
                        </span>
                      </Button>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
                      <DropdownMenuLabel>Filter by</DropdownMenuLabel>
                      <DropdownMenuSeparator />
                      {Object.keys(docFilters).map((f) => (
                        <DropdownMenuCheckboxItem key={f} checked={f === filter}>
                          <Link
                            href={`/documents/${f}`}
                            className="flex items-center justify-center transition-colors rounded-lg h-9 w-9 text-muted-foreground hover:text-foreground md:h-8 md:w-8"
                          >
                            <span>{f}</span>
                          </Link>
                        </DropdownMenuCheckboxItem>
                      ))}
                    </DropdownMenuContent>
                  </DropdownMenu>
                  <Button size="sm" variant="outline" className="h-8 gap-1">
                    <File className="h-3.5 w-3.5" />
                    <span className="sr-only sm:not-sr-only sm:whitespace-nowrap">
                      Export
                    </span>
                  </Button>

                  <CreateRoomForm createRoomQuery={createRoom} />
                </div>
              </div>
              <TabsContent value={filter}>
                <Card>
                  <CardHeader>
                    <CardTitle>Rooms</CardTitle>
                    <CardDescription>Manage your Rooms</CardDescription>
                  </CardHeader>
                  <CardContent>
                    <Table>
                      <TableHeader>
                        <TableRow>
                          <TableHead className="hidden w-[100px] sm:table-cell">
                            <span className="sr-only">Image</span>
                          </TableHead>
                          <TableHead>Name</TableHead>
                          <TableHead>Type</TableHead>
                          <TableHead className="hidden md:table-cell">
                            Default Access
                          </TableHead>
                          <TableHead className="hidden md:table-cell">
                            Group Access
                          </TableHead>
                          <TableHead className="hidden md:table-cell">
                            User Access
                          </TableHead>
                          <TableHead className="hidden md:table-cell">
                            Created at
                          </TableHead>
                          <TableHead>
                            <span className="sr-only">Actions</span>
                          </TableHead>
                        </TableRow>
                      </TableHeader>
                      <TableBody>
                        {rooms.data.map((room, idx) => (
                          <TableRow
                            key={`${idx}`}

                            className="hover:bg-slate-400"
                          >
                            <TableCell className="hidden sm:table-cell">
                              <Image
                                alt={room.id}
                                className="object-cover rounded-md aspect-square"
                                height="64"
                                src="/placeholder.png"
                                width="64"
                              />
                            </TableCell>
                            <TableCell className="font-medium">
                              {room.id}
                            </TableCell>
                            <TableCell>
                              <Badge variant="outline">
                                {room.metadata.type}
                              </Badge>
                            </TableCell>
                            <TableCell className="hidden md:table-cell">
                              {room.defaultAccesses}
                            </TableCell>
                            <TableCell className="hidden md:table-cell">
                              {Object.entries(room.groupsAccesses).map(
                                ([group, access]) => (
                                  <div key={`${group}`}>
                                    <strong>{group}: </strong>
                                    <span>{access}</span>
                                  </div>
                                )
                              )}
                            </TableCell>
                            <TableCell className="hidden md:table-cell">
                              {Object.entries(room.usersAccesses).map(
                                ([user, access]) => (
                                  <div key={`${user}`}>
                                    <strong>{user}: </strong>
                                    <span>{access}</span>
                                  </div>
                                )
                              )}
                            </TableCell>
                            <TableCell className="hidden md:table-cell">
                              {room.createdAt.toString()}
                            </TableCell>
                            <TableCell>
                              <DropdownMenu>
                                <DropdownMenuTrigger asChild>
                                  <Button
                                    aria-haspopup="true"
                                    size="icon"
                                    variant="ghost"
                                  >
                                    <MoreHorizontal className="w-4 h-4" />
                                    <span className="sr-only">Toggle menu</span>
                                  </Button>
                                </DropdownMenuTrigger>


                                <DropdownMenuContent align="end">
                                  {/* <DropdownMenuItem onSelect={(e) => {
                                    e.preventDefault()
                                    e.stopPropagation()
                                    deleteRoom({ id: room.id })
                                  }}>DeleteRoom</DropdownMenuItem> */}
                                  <DropdownMenuItem onSelect={(e) => {
                                    router.push(`/${room.metadata.type}/${room.id}`)

                                  }}>Go to room</DropdownMenuItem>

                                </DropdownMenuContent>

                              </DropdownMenu>
                            </TableCell>
                          </TableRow>
                        ))}
                      </TableBody>
                    </Table>
                  </CardContent>
                  <CardFooter>
                    <div className="text-xs text-muted-foreground">
                      Showing{" "}
                      <strong>
                        {Math.min(rooms.data.length, 1)}-
                        {rooms.nextCursor ?? rooms.data.length}
                      </strong>{" "}
                      of <strong>{rooms.data.length}</strong> Documents
                    </div>
                  </CardFooter>
                </Card>
              </TabsContent>
            </Tabs>
          </main>
        </div>
      </div>
    </TooltipProvider >
  );
};

export { Dashboard };
export default Dashboard;

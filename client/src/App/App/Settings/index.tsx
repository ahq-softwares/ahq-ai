import { ServersState } from "@/App/store/db/servers";
import useStateData from "@/App/store/state";

import { Category } from "@/components/category";
import { Separator } from "@/components/ui/separator";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { ServerCog, PlusIcon, Trash2 } from "lucide-react";

import { useState } from "react";
import { ResponsiveDialog } from "@/components/responsive/dialog";

import AddServer from "./AddServer";

export default function Settings() {
  const servers = useStateData(ServersState);

  const [open, setOpen] = useState(false);

  return <>
    <ResponsiveDialog
      open={open}
      setOpen={setOpen}
      button={<></>}
      buttonVariant={"ghost"}
      title="Add Server"
      description="Enter the server information with required credentials"
      content={
        <AddServer setOpen={setOpen} />
      }
    />

    <h1 className="text-lg mb-1">General Settings</h1>
    <Category
      title="Servers"
      description="Configure servers"
      Icon={ServerCog}
    >
      <h1 className="mb-2">Server List</h1>

      <Separator />

      <Table>
        <TableHeader>
          <TableRow>
            <TableHead>SNo</TableHead>
            <TableHead className="md:w-[40%]">Name</TableHead>
            <TableHead className="md:w-[40%]">Address</TableHead>
            <TableHead className="text-right">Actions</TableHead>
          </TableRow>
        </TableHeader>

        <TableBody>
          {servers.map((value, i) =>
          (
            <TableRow key={value.name + value.url}>
              <TableCell className="font-medium text-muted-foreground">{i + 1}.</TableCell>
              <TableCell className="font-medium text-muted-foreground">{value.name}</TableCell>
              <TableCell className="text-muted-foreground">{value.url}</TableCell>
              <TableCell className="text-right">
                <button
                  className="p-1 sm:p-2 m-1 bg-error/80 text-error-content cursor-pointer rounded-md"
                  onClick={() => {
                    ServersState.updateValueViaCallback((v) =>
                      v.filter((_, index) => index != i)
                    )
                  }}
                >
                  <Trash2 size="1.2rem" />
                </button>
              </TableCell>
            </TableRow>)
          )
          }

          {servers.length == 0 &&
            <TableRow>
              <TableCell className="font-medium text-muted-foreground text-center select-none" colSpan={3}>No Servers Found</TableCell>
            </TableRow>
          }
        </TableBody>

      </Table>

      <Separator />

      <div className="w-full flex text-center mt-6">
        <span className="text-lg my-auto">Add a new server</span>

        <button
          className="ml-auto flex cursor-pointer items-center gap-1 px-2 py-1 bg-accent dark:bg-neutral-content/20 dark:hover:bg-neutral-content/30 rounded-md"
          onClick={() => setOpen(true)}
        >
          <PlusIcon size={"1.2rem"} />
          <span>Add</span>
        </button>
      </div>
    </Category>

    <h1 className="text-lg mt-10 mb-1">Advanced Settings</h1>
  </>;
}
import { AuthType, HTTPServer, StatusFlags } from "@/App/server";
import { ServersState } from "@/App/store/db/servers";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

import { XCircle, AlertCircle } from "lucide-react";
import { useRef, useState } from "react";

type ServerLoginState = "Initial" | "Connecting";

export default function AddServer({ setOpen }: { setOpen: (_: boolean) => void }) {
  const [state, setState] = useState<ServerLoginState>("Initial");
  const [err, setErr] = useState<string | undefined>();
  const [warn, setWarn] = useState<string | undefined>();

  const serverNameRef = useRef<HTMLInputElement>(null);
  const serverUrlRef = useRef<HTMLInputElement>(null);

  return <>
    <div className="grid gap-4">
      <form
        className="grid gap-4 w-full"
        onSubmit={(e) => {
          e.preventDefault();
          setState("Connecting");

          const serverFriendlyName = serverNameRef.current!!.value;
          const url = serverUrlRef.current!!.value;

          httpLoad(url, serverFriendlyName, setOpen, setErr, setWarn, setState);
        }}
      >
        <div className="w-full flex gap-5">
          <div className="w-full grid gap-3">
            <Label htmlFor="name-1">Server Name</Label>
            <Input id="name-1" ref={serverNameRef} name="name" required minLength={2} autoComplete="off" disabled={state != "Initial"} placeholder="eg. Jail's Hub" />
          </div>

          <div className="w-full grid gap-3">
            <Label htmlFor="username-1">Server Url</Label>
            <Input id="username-1" ref={serverUrlRef} name="username" required minLength={5} autoComplete="off" disabled={state != "Initial"} placeholder="http://localhost:3124" />
          </div>
        </div>

        {state == "Initial" &&
          <Button
            type="submit"
          >
            Connect
          </Button>
        }
      </form>

      {err &&
        <div role="alert" className="dui-alert dui-alert-soft dui-alert-error">
          <XCircle />
          <span>{err}</span>
        </div>
      }

      {warn &&
        <div role="alert" className="dui-alert dui-alert-soft dui-alert-warn">
          <AlertCircle />
          <span>{warn}</span>
        </div>
      }

      {state == "Connecting" &&
        <div className="w-full flex text-center justify-center items-center gap-3 text-muted-foreground">
          <span className="dui-loading-spinner dui-loading" />

          <span className="select-none">Connecting...</span>
        </div>
      }
    </div>
  </>;
}

async function httpLoad(url: string, serverFriendlyName: string, setOpen: (_: boolean) => void, setErr: (_?: string) => void, setWarn: (_?: string) => void, setState: (_: ServerLoginState) => void) {
  try {
    const server = new HTTPServer(url, "no-auth");

    const flags = await server.getFlags();

    if ((flags & StatusFlags.Unavailable) > 0) {
      setErr("Server is unavailable!");
      setState("Initial");
      return;
    }

    if ((flags & StatusFlags.Unauthorized) > 0) {
      setErr("Unauthorized to use the server!");
      setState("Initial");
      return;
    }

    if ((flags & StatusFlags.UnsupportedServerVersion) > 0) {
      setErr("Server is on an unsupported version!");
      setState("Initial");
      return;
    }

    if ((flags & StatusFlags.ChallengeFailed) > 0) {
      setErr();
      setWarn("Integrity Challenge Failed!");
    }

    // We need no type of authentication
    if (server.auth == AuthType.OpenToAll) {
      ServersState.updateValueViaCallback((val) => {
        val.push({
          instance: server,
          session: "no-auth",
          name: serverFriendlyName,
          url: url,
          status: flags
        });

        return val;
      });

      setOpen(false);
      return;
    }
  } catch (e) {
    console.warn(e);
    setErr("Something went wrong. Please check if everything is correct!");
    setState("Initial");
  }
}
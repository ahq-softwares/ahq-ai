import { AuthType, HTTPServer, StatusFlags } from "@/App/server";
import { ServersState } from "@/App/store/db/servers";
import { Button } from "@/components/ui/button";
import { Card } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

import { XCircle, AlertCircle, CheckCircle2 } from "lucide-react";
import { useEffect, useRef, useState } from "react";

type ServerLoginState = "Initial" | "NoAuthVerify" | "AuthAccount" | "CheckingAuth" | "Connecting";

export default function AddServer({ setOpen, setLarge }: { setOpen: (_: boolean) => void, setLarge: (_: boolean) => void }) {
  const [state, setState] = useState<ServerLoginState>("Initial");
  const [err, setErr] = useState<string | undefined>();
  const [warn, setWarn] = useState<boolean | undefined>();

  const [err2, setErr2] = useState<string | undefined>();

  const [server, setSrvr] = useState<HTTPServer | undefined>();

  useEffect(() => {
    setLarge(state == "AuthAccount" || state == "CheckingAuth" || state == "NoAuthVerify");
  }, [state]);

  const serverNameRef = useRef<HTMLInputElement>(null);
  const serverUrlRef = useRef<HTMLInputElement>(null);

  return <>
    <div className="grid gap-4">
      <form
        className="grid gap-4 w-full"
        onSubmit={(e) => {
          e.preventDefault();
          setState("Connecting");

          const url = serverUrlRef.current!!.value;

          httpLoad(url, setErr, setWarn, setState).then(setSrvr);
        }}
      >
        <div className="w-full flex gap-5">
          <div className="w-full grid gap-3">
            <Label htmlFor="name-1">Server Name</Label>
            <Input id="name-1" ref={serverNameRef} name="name" required minLength={2} autoComplete="off" disabled={state != "Initial"} defaultValue="Jail's Hub" />
          </div>

          <div className="w-full grid gap-3">
            <Label htmlFor="username-1">Server Url</Label>
            <Input id="username-1" ref={serverUrlRef} name="username" required minLength={5} autoComplete="off" disabled={state != "Initial"} defaultValue="http://localhost:3124" />
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
        <div role="alert" className="dui-alert dui-alert-soft dui-alert-error flex flex-col">
          <div className="flex gap-2 w-full">
            <AlertCircle />
            <span>Integrity Challenge Failed</span>
          </div>

          <div className="flex gap-2 w-full text-justify">
            <span>Failed to verify server integrity. The server was unable to respond with the correct signature for the designated server version or it may be that the server is running on an outdated version. We recommend not to connect to this server.</span>
          </div>
        </div>
      }

      {warn === false &&
        <div role="alert" className="dui-alert dui-alert-soft dui-alert-success flex flex-col gap-2">
          <div className="flex gap-2 w-full">
            <CheckCircle2 />
            <span>Integrity Challenge Passed</span>
          </div>

          <div className="flex gap-2 w-full text-justify">
            <span>The server has passed the integrity challenge. This indicates a higher chance that the server is running the <strong>OFFICIAL AHQ AI SERVER</strong> executable</span>
          </div>

          <div className="flex gap-2 w-full text-justify">
            <span>We would like to aware you that the server might also be spoofing the signature, so please make sure you trust the server.</span>
          </div>
        </div>
      }

      {state == "Connecting" &&
        <div className="w-full flex text-center justify-center items-center gap-3 text-muted-foreground">
          <span className="dui-loading-spinner dui-loading" />

          <span className="select-none">Connecting...</span>
        </div>
      }

      {state == "NoAuthVerify" &&
        <Button
          className="w-full"
          onClick={() => {
            const serverFriendlyName = serverNameRef.current!!.value;

            ServersState.updateValueViaCallback((val) => {
              val.push({
                instance: server!!,
                session: "no-auth",
                name: serverFriendlyName,
                url: server!!.url,
                status: server!!.flags
              });

              return val;
            });

            setOpen(false);
          }}
        >
          Add Server
        </Button>
      }

      {(state == "AuthAccount" || state == "CheckingAuth") &&
        <Card className="px-3 bg-muted/20">
          <h1 className="mx-auto text-base-content underline">Authentication Required</h1>
          <form
            className="w-full flex flex-col text-center justify-center items-center gap-3 text-muted-foreground"
            onSubmit={(e) => {
              e.preventDefault();

              const uuid = document.getElementById("uuid-1")!! as HTMLInputElement;
              const pwd = document.getElementById("pwd-1")!! as HTMLInputElement;

              setState("CheckingAuth");

              server!!.authenticate(uuid.value, pwd.value)
                .then(() => {
                  setErr2(undefined);
                  const serverFriendlyName = serverNameRef.current!!.value;

                  ServersState.updateValueViaCallback((val) => {
                    val.push({
                      instance: server!!,
                      session: server!!.session,
                      name: serverFriendlyName,
                      url: server!!.url,
                      status: server!!.flags
                    });

                    return val;
                  });

                  setOpen(false);
                })
                .catch(() => {
                  setState("AuthAccount");
                  setErr2("Invalid username or password");
                });
            }}
          >
            {err2 &&
              <div role="alert" className="w-full dui-alert dui-alert-soft dui-alert-error">
                <XCircle />
                <span>{err2}</span>
              </div>
            }

            <div className="w-full grid gap-3">
              <Label htmlFor="uuid-1 text-muted-content">Username</Label>
              <Input id="uuid-1" name="uuid" disabled={state == "CheckingAuth"} required minLength={2} autoComplete="off" placeholder="Unique ID" />
            </div>

            <div className="w-full grid gap-3">
              <Label htmlFor="pwd-1 text-muted-content">Password</Label>
              <Input id="pwd-1" name="pwd" disabled={state == "CheckingAuth"} required minLength={5} autoComplete="off" placeholder="Enter your password" type="password" />
            </div>

            <Button
              type="submit"
              className="w-full cursor-pointer"
              variant={"default"}
              disabled={state == "CheckingAuth"}
            >
              {state == "CheckingAuth" ?
                <div className="flex gap-2">
                  <span className="dui-loading dui-loading-xs dui-loading-spinner"></span>

                  <span>Please wait...</span>
                </div> :
                <>Login</>}
            </Button>
          </form>
        </Card>
      }
    </div>
  </>;
}

async function httpLoad(url: string, setErr: (_?: string) => void, setWarn: (_?: boolean) => void, setState: (_: ServerLoginState) => void): Promise<HTTPServer> {
  const server = new HTTPServer(url, "no-auth");

  try {
    const flags = await server.getFlags();

    if ((flags & StatusFlags.Unavailable) > 0) {
      setErr("Server is unavailable!");
      setState("Initial");
      return server;
    }

    if ((flags & StatusFlags.Unauthorized) > 0) {
      setErr("Unauthorized to use the server!");
      setState("Initial");
      return server;
    }

    if ((flags & StatusFlags.UnsupportedServerVersion) > 0) {
      setErr("Server is on an unsupported version!");
      setState("Initial");
      return server;
    }

    if ((flags & StatusFlags.ChallengeFailed) > 0) {
      setErr();
      setWarn(true);
    } else {
      setWarn(false);
    }

    // We need no type of authentication
    if (server.auth == AuthType.OpenToAll) {
      setState("NoAuthVerify");
      return server;
    }

    if (server.auth == AuthType.Account) {
      setState("AuthAccount");
      return server;
    }
  } catch (e) {
    console.warn(e);
    setErr("Something went wrong. Please check if everything is correct!");
    setState("Initial");
  }

  return server;
}
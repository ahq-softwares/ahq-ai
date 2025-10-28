import { fetch } from "@tauri-apps/plugin-http";
import { satisfies } from "semver"

export const supportedServerSemver = "0.x";

export const StatusFlags = {
  Ok: 1,
  Unavailable: 2,
  Unauthorized: 4,
  UnsupportedServerVersion: 8
}

export enum AuthType {
  Unknown,
  OpenToAll,
  TokenBased,
  Account
}

export class HTTPServer {
  private url: string;
  session: string;

  auth = AuthType.Unknown;

  constructor(url: string, session: string) {
    this.url = url, this.session = session;
  }

  async getFlags() {
    let out = 0;

    let output;
    try {
      output = await fetch(`${this.url}/`, {
        connectTimeout: 1000
      })
        .then((d) => d.json());
    } catch (e) {
      console.warn(e);
      out |= StatusFlags.Unavailable;

      return out;
    }

    if (!satisfies(output.version, supportedServerSemver)) out |= StatusFlags.UnsupportedServerVersion;

    this.auth = (() => {
      switch (output.auth as string) {
        case "OpenToAll":
          return AuthType.OpenToAll;
        case "TokenBased":
          return AuthType.TokenBased;
        case "Account":
          return AuthType.Account;
        default:
          return AuthType.Unknown;
      }
    })();

    if (this.auth == AuthType.Unknown) {
      out |= StatusFlags.Unauthorized;

      return out;
    }

    if (this.auth == AuthType.OpenToAll) {
      return out;
    }

    // Auth Check

    return out;
  }
}
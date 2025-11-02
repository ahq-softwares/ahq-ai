import { fetch } from "@tauri-apps/plugin-http";
import { satisfies } from "semver"
import { getKeys } from "./key";

import { checkServerIntegrity } from "tauri-plugin-ahqai-api"

export const supportedServerSemver = "0.x";

export const StatusFlags = {
  Unavailable: 2,
  Unauthorized: 4,
  UnsupportedServerVersion: 8,
  ChallengeFailed: 16
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
    const keys = (await getKeys(true)).keys;

    console.log(keys);

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

    const versionKey = `v${output.version}`;

    if (!keys[versionKey]) out |= (StatusFlags.ChallengeFailed || StatusFlags.UnsupportedServerVersion);

    if (keys[versionKey]) {
      const pubkey = keys[versionKey].pubkey;

      const data = new Uint8Array(256);

      data.fill(0);

      for (let i = 0; i < 256; i++) {
        data[i] = Math.floor(Math.random() * 127);
      }

      console.log(`Using pubkey ${pubkey}`);

      const binaryString = atob(pubkey);
      const pkey = Uint8Array.from(binaryString, (char) => char.charCodeAt(0));

      const signature = await fetch(`${this.url}/challenge`, {
        method: "POST",
        body: data.buffer
      })
        .then((d) => d.arrayBuffer())
        .catch(() => new ArrayBuffer());

      console.warn(data, signature, pkey);

      if (!(await checkServerIntegrity(data.buffer, signature, pkey))) out |= StatusFlags.ChallengeFailed;
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
    // TODO: Auth Ping

    return out;
  }
}
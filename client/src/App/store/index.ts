import { load, Store } from "@tauri-apps/plugin-store"
import { initServerState } from "./db/servers";

let store: Store;

export async function initStore() {
  await initServerState();

  store = await load("prefs.json", { autoSave: true, defaults: {} });
}

export const getStore = () => store;
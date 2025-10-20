import { load, Store } from "@tauri-apps/plugin-store"

let store: Store;

export async function initStore() {
  store = await load("prefs.json", { autoSave: true, defaults: {} });
}

export const getStore = () => store;
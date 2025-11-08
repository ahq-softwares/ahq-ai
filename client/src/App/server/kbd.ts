import { platform } from "@tauri-apps/plugin-os";

export default function ctrlOrCmd(key: string) {
  switch (platform()) {
    case "macos":
      return `⌘ + ${key}`
    case "android":
      return "";
    default:
      return `Ctrl + ${key}`
  }
}
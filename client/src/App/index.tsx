import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { useEffect, useMemo, useState } from "react";
import Splash from "./Splash";

import { ThemeContext } from "./theme";
import { initStore } from "./store";
import Application from "./App";
import { getKeys } from "./server/key";

export const PageId = {
  Splash: 0,
  Home: 1
};

export default function App() {
  const [page, setPage] = useState(PageId.Splash);

  const [, showDangerScreen] = useState(false);

  useEffect(() => {
    (async () => {
      // Show the window when its ready
      try {
        await initStore();
        await getCurrentWebviewWindow().show();
        try {
          await getKeys();
        } catch (e) {
          console.error(e);
          showDangerScreen(true);
        }

        setTimeout(() => {
          setPage(PageId.Home);
        }, 2000);
      } catch (e) {
        console.error(e);
      }
    })()
  }, []);

  const pageContent = useMemo(() => {
    switch (page) {
      case PageId.Splash:
        return <Splash />;
      default:
        return <Application />;
    }
  }, [page]);

  return <ThemeContext>
    {pageContent}
  </ThemeContext>
}

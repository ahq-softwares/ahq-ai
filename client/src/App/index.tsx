import { getCurrentWebviewWindow } from "@tauri-apps/api/webviewWindow";
import { useEffect, useMemo, useState } from "react";
import Splash from "./Splash";

import { ThemeContext } from "./theme";
import { initStore } from "./store";

export const PageId = {
  Splash: 0,
  Home: 1
};

export default function App() {
  const [page, setPage] = useState(PageId.Splash);

  useEffect(() => {
    (async () => {
      // Show the window when its ready
      try {
        await initStore();
        await getCurrentWebviewWindow().show();

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
        return <>Not Found</>;
    }
  }, [page]);

  return <ThemeContext>
    {pageContent}
  </ThemeContext>
}


// look at discord
// whats going on rn
// Navigation
// you are making me feel like an amature
// Nah
// Its how i develop
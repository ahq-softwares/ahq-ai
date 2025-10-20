import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App/index.tsx";
import { ContextMenu, ContextMenuTrigger } from "./components/ui/context-menu.tsx";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <ContextMenu>
      <ContextMenuTrigger>
        <App />
      </ContextMenuTrigger>
    </ContextMenu>
  </React.StrictMode>,
);

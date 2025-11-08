import ReactDOM from "react-dom/client";
import App from "./App/index.tsx";

import "./global.css"

import { ContextMenu, ContextMenuTrigger } from "./components/ui/context-menu.tsx";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <ContextMenu>
    <ContextMenuTrigger>
      <App />
    </ContextMenuTrigger>
  </ContextMenu>,
);

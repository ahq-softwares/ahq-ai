import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { useMediaQuery } from "@/hooks/use-media-query";
import { useMemo } from "react";

export default function Application() {
  const tab = useMediaQuery("(min-width: 768px)");

  if (tab) {
    return <ApplicationDesktop />;
  }

  // Use a bottom nav bar
  return <>Mobile</>;
}

export function ApplicationDesktop() {
  const lg = useMediaQuery("(min-width: 1024px)");

  const { min, max, def } = useMemo(() => {
    const size = parseInt(localStorage.getItem("sidebar-wid") || "20");

    if (lg) {
      return {
        min: 20,
        max: 30,
        def: size <= 20 ? 20 : size >= 30 ? 30 : size
      }
    }

    return {
      min: 30,
      max: 40,
      def: size <= 30 ? 30 : size >= 40 ? 40 : size
    }
  }, [lg]);

  return <ResizablePanelGroup className="!w-screen !h-screen" direction="horizontal">
    <ResizablePanel
      minSize={min}
      maxSize={max}
      defaultSize={def}
      onResize={(size) => {
        localStorage.setItem("sidebar-wid", String(size));
      }}
      className="h-full w-full"
    >
      Sidebar
    </ResizablePanel>

    <ResizableHandle className="!bg-none" withHandle />

    <ResizablePanel className="bg-neutral/40 rounded-tl-4xl p-3">
      Content
    </ResizablePanel>
  </ResizablePanelGroup>
}


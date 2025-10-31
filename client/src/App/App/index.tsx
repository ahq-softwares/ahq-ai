import { ResizableHandle, ResizablePanel, ResizablePanelGroup } from "@/components/ui/resizable";
import { useMediaQuery } from "@/hooks/use-media-query";
import { ReactNode, useMemo, useState } from "react";
import Sidebar from "./Sidebar";
import { Sheet, SheetContent, SheetDescription, SheetHeader, SheetTitle, SheetTrigger } from "@/components/ui/sheet";

import { Menu } from "lucide-react";
import Settings from "./Settings";

export enum AppPage {
  Diposable = 2,
  Chat = 3,
  Admin = 4,
  Settings = 5,
  ChatPage = 6,
}

const txtMap = {
  2: "Incognito Chat",
  3: "New Chat",
  4: "Administrator Portal",
  5: "Settings",
  6: ""
};

export default function Application() {
  const tab = useMediaQuery("(min-width: 768px)");

  const [dialogOpen, setOpeNav] = useState(false);
  const [page, setPage] = useState<AppPage>(AppPage.Chat);

  const content = useMemo(() => {
    switch (page) {
      case AppPage.Settings:
        return <Settings />
      default:
        return <>Hi</>;
    }
  }, [page]);

  if (tab) {
    return <ApplicationDesktop page={page} content={content} pageSet={(page) => setPage(page)} />;
  }

  // Use a hamburger sidebar
  return <div className="flex flex-col h-screen! w-screen!">
    <div className="h-12 px-2 flex text-center w-full">
      <Sheet open={dialogOpen} onOpenChange={(c) => setOpeNav(c)}>
        <SheetTrigger>
          <div className="cursor-pointer rounded-lg hover:bg-neutral/40 p-2 text-muted-foreground">
            <Menu size={"1rem"} />
          </div>
        </SheetTrigger>

        <SheetContent side="left">
          <SheetHeader>
            <SheetTitle aria-valuetext="Sidebar" aria-description="Sidebar"></SheetTitle>
            <SheetDescription aria-valuetext="Sidebar"></SheetDescription>
          </SheetHeader>

          <Sidebar
            chats={[]}
            page={page}
            pageSet={(page) => {
              setOpeNav(false);
              setPage(page);
            }}
          />
        </SheetContent>
      </Sheet>

      <div className="mx-auto select-none text-sm text-muted-foreground flex text-center justify-center items-center">
        <span>{txtMap[page]}</span>
      </div>

      <div className="rounded-lg p-2 text-transparent">
        <Menu size={"1rem"} />
      </div>
    </div>

    <div className="w-full h-full flex flex-col px-4">
      {content}
    </div>
  </div>;
}

interface Props {
  page: AppPage;
  pageSet: (page: AppPage) => void;
  content: ReactNode | ReactNode[]
}

export function ApplicationDesktop({ pageSet, page, content }: Props) {
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

  return <ResizablePanelGroup className="w-screen! h-screen!" direction="horizontal">
    <ResizablePanel
      minSize={min}
      maxSize={max}
      defaultSize={def}
      onResize={(size) => {
        localStorage.setItem("sidebar-wid", String(size));
      }}
      className="h-full w-full"
    >
      <Sidebar
        chats={[]}
        page={page}
        pageSet={pageSet}
      />
    </ResizablePanel>

    <ResizableHandle className="bg-none!" withHandle />

    <ResizablePanel className="bg-neutral/30 rounded-tl-2xl w-full h-full p-3 flex flex-col">
      {content}
    </ResizablePanel>
  </ResizablePanelGroup>
}


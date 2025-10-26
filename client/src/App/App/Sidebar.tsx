import { Separator } from "@/components/ui/separator";
import { MessageCircleDashed, MessageCircle, Settings, ShieldUser, LucideProps } from "lucide-react";
import { ForwardRefExoticComponent, RefAttributes } from "react";

interface SidebarProps {

}

export default function Sidebar(props: SidebarProps) {
  return <div className="w-full h-full px-3 py-2 gap-1 flex flex-col overflow-y-scroll overflow-x-clip">
    <SidebarItem
      text="New Chat"
      Icon={MessageCircle}
    />

    <SidebarItem
      text="New Chat"
      Icon={MessageCircleDashed}
    />

    <Separator />

    <div className="h-full flex flex-col w-full">
      <div className="text-muted-foreground mx-auto">
        Chats
      </div>
      <SidebarItem
        text="When operating systems don't work as expected"
        Icon={MessageCircle}
      />
    </div>

    <Separator />

    <SidebarItem
      text="Admin Portal"
      Icon={ShieldUser}
    />

    <SidebarItem
      text="Settings"
      Icon={Settings}
    />
  </div>
}

function SidebarItem({ text, Icon }: { text: string, Icon: ForwardRefExoticComponent<Omit<LucideProps, "ref"> & RefAttributes<SVGSVGElement>> }) {
  return <div className="w-full h-10 flex overflow-x-hidden rounded-lg overflow-y-hidden px-3 gap-2 py-2 select-none cursor-pointer transition-all border border-transparent hover:shadow-lg hover:border-border hover:bg-neutral/30 items-center group">
    <Icon className="text-muted-foreground group-hover:text-base-content min-h-5 max-h-5 min-w-5 max-w-5" />
    <span className="text-sm line-clamp-1 text-muted-foreground group-hover:text-base-content">{text}</span>
  </div>
}
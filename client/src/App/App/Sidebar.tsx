import { BotMessageSquare, MessageCircleDashed, MessageCircle, Settings, ShieldUser } from "lucide-react";

interface SidebarProps {

}

export default function Sidebar(props: SidebarProps) {
  return <div className="w-full h-full px-3 py-2 flex flex-col overflow-y-scroll overflow-x-clip">
    <SidebarItem />
  </div>
}

function SidebarItem() {
  return <div className="w-full h-10 flex overflow-x-hidden rounded-lg overflow-y-hidden px-3 gap-2 py-2 select-none cursor-pointer transition-all border border-transparent hover:shadow-lg hover:border-border hover:bg-neutral/30 items-center group">
    <MessageCircle className="text-muted-foreground group-hover:text-base-content min-h-5 max-h-5 min-w-5 max-w-5" />
    <span className="block text-sm line-clamp-1 text-muted-foreground group-hover:text-base-content text-clip">New Chat</span>
  </div>
}
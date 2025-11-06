import { InputGroup, InputGroupAddon, InputGroupButton, InputGroupTextarea } from "@/components/ui/input-group";
import { Separator } from "@/components/ui/separator";
import { ArrowUp, ChevronLeft, ChevronRight, Plus } from "lucide-react";

export default function Chat() {
  return <div className="w-full h-full flex flex-col gap-5 md:pb-5">
    <div className="h-full w-full bg-yellow-700">

    </div>

    <div className="w-full items-center text-center justify-center flex">
      <InputGroup className="w-full rounded-b-none md:rounded-b-md max-h-64 md:min-w-[30rem] md:max-w-[75%]">
        <InputGroupTextarea placeholder="Ask, Search or Chat..." />

        <InputGroupAddon
          align="block-start"
          className="cursor-default overflow-y-hidden overflow-x-scroll"
        >
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
          <div className="min-h-20 max-h-20 max-w-20 min-w-20 bg-black rounded-lg" />
        </InputGroupAddon>
        <InputGroupAddon
          align="block-start"
          className="cursor-default my-2 flex px-5"
        >
          <InputGroupButton
            variant="outline"
            className="rounded-full"
            size="icon-xs"
          >
            <ChevronLeft />
          </InputGroupButton>

          <InputGroupButton
            variant="outline"
            className="rounded-full ml-auto"
            size="icon-xs"
          >
            <ChevronRight />
          </InputGroupButton>
        </InputGroupAddon>

        <InputGroupAddon align="block-end" className="cursor-default">
          <InputGroupButton
            variant="outline"
            className="rounded-full"
            size="icon-xs"
          >
            <Plus />
          </InputGroupButton>

          <Separator orientation="vertical" className="!h-4 ml-auto" />

          <InputGroupButton
            variant="default"
            className="rounded-full"
            size="icon-xs"
          >
            <ArrowUp />
            <span className="sr-only">Send</span>
          </InputGroupButton>
        </InputGroupAddon>
      </InputGroup>
    </div>
  </div>
}
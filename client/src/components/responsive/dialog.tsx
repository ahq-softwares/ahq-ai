"use client"

import * as React from "react"

import { useMediaQuery } from "@/hooks/use-media-query"
import { Button } from "@/components/ui/button"
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog"
import {
  Drawer,
  DrawerClose,
  DrawerContent,
  DrawerDescription,
  DrawerFooter,
  DrawerHeader,
  DrawerTitle,
  DrawerTrigger,
} from "@/components/ui/drawer"

interface DialogProps {
  button?: React.ReactNode;
  triggerButtonOverrides?: boolean;
  buttonVariant: "link" | "default" | "destructive" | "outline" | "secondary" | "ghost" | null | undefined;
  title?: string;
  description?: string;
  content: React.ReactNode;
  open: boolean;
  setOpen: (_: boolean) => void;
}

export function ResponsiveDialog(
  {
    button,
    buttonVariant,
    title,
    content,
    description,
    triggerButtonOverrides = false,
    open,
    setOpen
  }: DialogProps
) {
  const isDesktop = useMediaQuery("(min-width: 768px)")

  if (isDesktop) {
    return (
      <Dialog open={open} onOpenChange={setOpen}>
        {button && <DialogTrigger asChild>
          {triggerButtonOverrides ? button : <Button variant={buttonVariant}>{button}</Button>}
        </DialogTrigger>}
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{title}</DialogTitle>
            <DialogDescription className="text-left">
              {description}
            </DialogDescription>
          </DialogHeader>
          {content}
        </DialogContent>
      </Dialog>
    )
  }

  return (
    <Drawer open={open} onOpenChange={setOpen}>
      {button && <DrawerTrigger asChild>
        {triggerButtonOverrides ? button : <Button variant={buttonVariant}>{button}</Button>}
      </DrawerTrigger>}
      <DrawerContent>
        <DrawerHeader className="text-left">
          <DrawerTitle>{title}</DrawerTitle>
          <DrawerDescription>
            {description}
          </DrawerDescription>
        </DrawerHeader>

        <div className="w-full h-full px-4">
          {content}
        </div>

        <DrawerFooter className="pt-2">
          <DrawerClose asChild>
            <Button variant="outline">Cancel</Button>
          </DrawerClose>
        </DrawerFooter>
      </DrawerContent>
    </Drawer>
  )
}
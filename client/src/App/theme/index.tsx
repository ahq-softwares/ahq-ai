import { createContext, ReactNode, useContext, useEffect, useState } from "react";

export enum Theme {
  Light,
  Dark
}

export const ThemeCtx = createContext({
  theme: Theme.Light,
  toggleTheme: (_: Theme) => { }, // Placeholder function
});

export function getThemeCtx() {
  return useContext(ThemeCtx);
}

const getCurrOSTheme = () => {
  return window.matchMedia("(prefers-color-scheme: dark)").matches ? Theme.Dark : Theme.Light;
}

const setTheme = (theme: Theme, os: Theme) => {
  const html = document.querySelector("html")!!;
  // how much is left?
  if (theme == os) {
    html.removeAttribute("no-transparent");
  } else {
    html.setAttribute("no-transparent", "true");
  }

  if (theme == Theme.Dark) {
    html.setAttribute("data-theme", "dark");
    html.classList.add("dark");
  } else {
    html.setAttribute("data-theme", "cupcake");
    html.classList.remove("dark");
  }
}

export function ThemeContext({ children }: { children?: ReactNode }) {
  const [theme, set] = useState(Theme.Dark);

  useEffect(() => {
    const l = localStorage.getItem("theme");

    if (!l) {
      set(getCurrOSTheme());
    } else {
      const t = l == "true" ? Theme.Dark : Theme.Light;

      set(t);
    }

  }, []);

  useEffect(() => {
    setTheme(theme, getCurrOSTheme());
  }, [theme])

  return <ThemeCtx.Provider value={{ theme, toggleTheme: (t) => set(t) }}>
    {children}
  </ThemeCtx.Provider>
}
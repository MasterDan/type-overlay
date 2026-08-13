import { createSignal, createEffect } from "solid-js";
import { useTauriEvent } from "~/hooks/useTauriEvent";
import { commands } from "~/lib/tauri";
import type { AppMode } from "~/types";

export function useAppMode() {
  const [mode, setMode] = createSignal<AppMode>("window");

  useTauriEvent<{ mode: AppMode }>("mode-changed", (e) => setMode(e.mode));

  createEffect(() => {
    document.body.classList.toggle("is-overlay", mode() === "overlay");
  });

  const toMode = (next: AppMode) => {
    setMode(next);
    commands.setMode(next);
  };

  return { mode, toMode };
}

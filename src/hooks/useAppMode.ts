import { createSignal, createEffect } from "solid-js";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { useTauriEvent } from "~/hooks/useTauriEvent";
import { commands } from "~/lib/tauri";
import type { AppMode } from "~/types";

export function useAppMode() {
  // Secondary overlay windows are created after the initial `mode-changed`
  // event was emitted, so they must detect their role from the window label.
  const isOverlayWindow = getCurrentWindow().label.startsWith("overlay-");
  const [mode, setMode] = createSignal<AppMode>(
    isOverlayWindow ? "overlay" : "window",
  );

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

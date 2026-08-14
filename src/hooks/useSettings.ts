import { createSignal, createEffect, onMount } from "solid-js";
import { load, type Store } from "@tauri-apps/plugin-store";
import { debounce } from "@solid-primitives/scheduled";
import { commands } from "~/lib/tauri";
import { DEFAULT_SETTINGS } from "~/lib/settings";
import type { HotkeyAction, Settings } from "~/types";

export function useSettings() {
  const [settings, setSettings] = createSignal<Settings>(DEFAULT_SETTINGS);
  let store: Store | undefined;

  onMount(async () => {
    store = await load("settings.json", { autoSave: false });
    const saved = await store.get<Settings["hotkeys"]>("hotkeys");
    const savedOpacity = await store.get<Settings["overlayOpacity"]>(
      "overlayOpacity",
    );
    const merged: Settings = {
      hotkeys: { ...DEFAULT_SETTINGS.hotkeys, ...(saved ?? {}) },
      overlayOpacity: savedOpacity ?? DEFAULT_SETTINGS.overlayOpacity,
    };
    setSettings(merged);
    await commands.registerHotkeys(merged.hotkeys);
  });

  const persist = debounce(async (next: Settings) => {
    if (!store) return;
    await store.set("hotkeys", next.hotkeys);
    await store.set("overlayOpacity", next.overlayOpacity);
    await store.save();
  }, 300);

  createEffect(() => {
    const { hotkeys } = settings();
    if (!store) return;
    persist(settings());
    void commands.registerHotkeys(hotkeys);
  });

  const setHotkey = (action: HotkeyAction, combo: string) => {
    setSettings((prev) => ({
      ...prev,
      hotkeys: { ...prev.hotkeys, [action]: combo },
    }));
  };

  const setOverlayOpacity = (value: number) => {
    setSettings((prev) => ({
      ...prev,
      overlayOpacity: Math.min(100, Math.max(0, value)),
    }));
  };

  return { settings, setHotkey, setOverlayOpacity };
}

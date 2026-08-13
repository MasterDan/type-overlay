import { invoke } from "@tauri-apps/api/core";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import type { AppMode, CaptureStatus, Layout, Settings } from "~/types";

export const commands = {
  setMode: (mode: AppMode) => invoke<void>("set_app_mode", { mode }),
  setOverlayVisible: (visible: boolean) =>
    invoke<void>("set_overlay_visible", { visible }),
  registerHotkeys: (hotkeys: Settings["hotkeys"]) =>
    invoke<void>("register_hotkeys", { hotkeys }),
  getLayout: () => invoke<Layout>("get_layout"),
  getPlatform: () => invoke<string>("get_platform"),
  checkAccessibility: () => invoke<boolean>("check_accessibility"),
  getCaptureStatus: () => invoke<CaptureStatus>("get_capture_status"),
};

export function onEvent<T = unknown>(
  name: string,
  handler: (payload: T) => void,
): Promise<UnlistenFn> {
  return listen<T>(name, (e) => handler(e.payload));
}

import {
  createContext,
  useContext,
  createSignal,
  createEffect,
  onMount,
  type Accessor,
  type ParentComponent,
} from "solid-js";
import { useLayout } from "~/hooks/useLayout";
import { useAppMode } from "~/hooks/useAppMode";
import { useSettings } from "~/hooks/useSettings";
import { commands } from "~/lib/tauri";
import { applyTheme, platformFromOs, type Platform } from "~/lib/theme";
import type { AppMode, HotkeyAction, Layout, Settings } from "~/types";

interface AppStore {
  layout: Accessor<Layout>;
  cycleLayout: () => void;
  mode: Accessor<AppMode>;
  isOverlay: Accessor<boolean>;
  toMode: (next: AppMode) => void;
  settings: Accessor<Settings>;
  setHotkey: (action: HotkeyAction, combo: string) => void;
  setOverlayOpacity: (value: number) => void;
  platform: Accessor<Platform>;
}

const AppContext = createContext<AppStore>();

const useAppState = (): AppStore => {
  const { layout, cycle } = useLayout();
  const { mode, toMode } = useAppMode();
  const { settings, setHotkey, setOverlayOpacity } = useSettings();
  const [platform, setPlatform] = createSignal<Platform>("unknown");

  onMount(() => {
    commands
      .getPlatform()
      .then((os) => setPlatform(platformFromOs(os)))
      .catch(() => {});
  });
  createEffect(() => applyTheme(platform()));

  return {
    layout,
    cycleLayout: cycle,
    mode,
    isOverlay: () => mode() === "overlay",
    toMode,
    settings,
    setHotkey,
    setOverlayOpacity,
    platform,
  };
};

export const AppProvider: ParentComponent = (props) => (
  <AppContext.Provider value={useAppState()}>
    {props.children}
  </AppContext.Provider>
);

export function useApp(): AppStore {
  const ctx = useContext(AppContext);
  if (!ctx) throw new Error("useApp must be used within AppProvider");
  return ctx;
}

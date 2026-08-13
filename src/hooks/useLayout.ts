import { createSignal, onMount } from "solid-js";
import { useTauriEvent } from "~/hooks/useTauriEvent";
import { commands } from "~/lib/tauri";
import type { Layout } from "~/types";

export function useLayout() {
  const [layout, setLayout] = createSignal<Layout>("en");
  let manual = false;

  onMount(() => {
    commands.getLayout().then(setLayout).catch(() => setLayout("en"));
  });

  useTauriEvent<{ layout: Layout }>("layout-changed", (e) => {
    if (!manual) setLayout(e.layout);
  });

  const cycle = () => {
    manual = true;
    setLayout((l) => (l === "en" ? "ru" : "en"));
  };

  return { layout, cycle };
}

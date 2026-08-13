import { type Component } from "solid-js";
import { Keyboard } from "~/components/keyboard/Keyboard";
import { useApp } from "~/store/app";

export const MainView: Component = () => {
  const { layout } = useApp();
  return (
    <div class="h-full rounded-[var(--kb-plate-radius)] border border-white/10 bg-white/[0.03] p-4">
      <Keyboard layout={layout} />
    </div>
  );
};

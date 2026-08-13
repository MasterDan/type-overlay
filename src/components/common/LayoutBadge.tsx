import { type Component } from "solid-js";
import { useApp } from "~/store/app";
import { LAYOUT_LABELS } from "~/lib/layouts";

export const LayoutBadge: Component = () => {
  const { layout, cycleLayout } = useApp();
  return (
    <button
      type="button"
      onClick={cycleLayout}
      title="Сменить язык (вручную)"
      class="cursor-pointer rounded-md border border-accent/30 bg-accent/10 px-2 py-0.5 text-xs font-bold tracking-wider text-accent-soft tabular-nums transition-colors hover:bg-accent/20"
    >
      {LAYOUT_LABELS[layout()]}
    </button>
  );
};

import { type Component } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { VsGear, VsLayers } from "solid-icons/vs";
import { IconButton } from "~/components/common/IconButton";
import { LayoutBadge } from "~/components/common/LayoutBadge";
import { useApp } from "~/store/app";

export const TopBar: Component = () => {
  const { toMode } = useApp();
  const navigate = useNavigate();
  return (
    <header class="flex items-center gap-3 px-4 py-3">
      <h1 class="text-sm font-semibold text-white/90">Type Overlay</h1>
      <LayoutBadge />
      <div class="flex-1" />
      <IconButton onClick={() => navigate("/settings")} aria-label="Настройки">
        <VsGear size={16} />
      </IconButton>
      <IconButton onClick={() => toMode("overlay")} aria-label="Режим оверлея">
        <VsLayers size={16} />
        <span>Overlay</span>
      </IconButton>
    </header>
  );
};

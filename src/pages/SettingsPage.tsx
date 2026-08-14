import { type Component, For } from "solid-js";
import { useNavigate } from "@solidjs/router";
import { VsArrowLeft } from "solid-icons/vs";
import { IconButton } from "~/components/common/IconButton";
import { HotkeyRecorder } from "~/components/settings/HotkeyRecorder";
import { useApp } from "~/store/app";
import { HOTKEY_ACTIONS, HOTKEY_META } from "~/lib/settings";

export const SettingsPage: Component = () => {
  const navigate = useNavigate();
  const { settings, setHotkey, setOverlayOpacity } = useApp();

  return (
    <div class="mx-auto flex h-full max-w-2xl flex-col gap-4 overflow-auto">
      <div class="flex items-center gap-3">
        <IconButton onClick={() => navigate("/")} aria-label="Назад">
          <VsArrowLeft size={16} />
        </IconButton>
        <h2 class="text-lg font-semibold text-white/90">Горячие клавиши</h2>
      </div>

      <div class="flex items-center justify-between gap-4 rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3">
        <div>
          <div class="text-sm font-medium text-white/85">
            Прозрачность оверлея
          </div>
          <div class="text-xs text-white/45">
            Насколько ярко клавиатура видна поверх окон
          </div>
        </div>
        <div class="flex shrink-0 items-center gap-3">
          <input
            type="range"
            min="0"
            max="100"
            step="5"
            value={settings().overlayOpacity}
            onInput={(e) => setOverlayOpacity(Number(e.currentTarget.value))}
            class="w-40 accent-[var(--color-accent)]"
            aria-label="Прозрачность оверлея"
          />
          <span class="w-10 text-right text-sm tabular-nums text-white/70">
            {settings().overlayOpacity}%
          </span>
        </div>
      </div>

      <div class="flex flex-col gap-3">
        <For each={HOTKEY_ACTIONS}>
          {(action) => (
            <div class="flex items-center justify-between gap-4 rounded-xl border border-white/10 bg-white/[0.03] px-4 py-3">
              <div>
                <div class="text-sm font-medium text-white/85">
                  {HOTKEY_META[action].title}
                </div>
                <div class="text-xs text-white/45">
                  {HOTKEY_META[action].description}
                </div>
              </div>
              <HotkeyRecorder
                combo={settings().hotkeys[action]}
                onRecord={(combo) => setHotkey(action, combo)}
              />
            </div>
          )}
        </For>
      </div>

      <p class="mt-1 text-xs leading-relaxed text-white/35">
        В режиме оверлея клавиатура рисуется поверх всех окон и не перехватывает
        клики. Управлять видимостью можно этими сочетаниями.
      </p>
    </div>
  );
};

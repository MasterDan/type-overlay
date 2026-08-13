import { type Component, Show } from "solid-js";
import { useApp } from "~/store/app";
import { useCaptureStatus } from "~/hooks/useCaptureStatus";

export const PermissionBanner: Component = () => {
  const { platform } = useApp();
  const status = useCaptureStatus();

  return (
    <Show when={platform() === "macos" && status() === "waiting"}>
      <div class="mx-4 mb-3 rounded-xl border border-amber-400/30 bg-amber-400/10 px-4 py-3 text-sm text-amber-100/90">
        <p class="font-medium">Нет доступа к клавиатуре</p>
        <p class="mt-1 text-xs leading-relaxed text-amber-100/70">
          Разрешите <span class="font-semibold">Специальные возможности</span> и{" "}
          <span class="font-semibold">Мониторинг ввода</span> для Type Overlay в
          Системные настройки → Конфиденциальность и безопасность. После этого
          клавиши подсветятся автоматически — иначе перезапустите приложение.
        </p>
      </div>
    </Show>
  );
};

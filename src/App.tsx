import { type ParentComponent, Show } from "solid-js";
import { AppProvider, useApp } from "~/store/app";
import { PressedProvider } from "~/hooks/usePressed";
import { CaptureProvider } from "~/hooks/useCaptureStatus";
import { TopBar } from "~/components/TopBar";
import { Keyboard } from "~/components/keyboard/Keyboard";
import { PermissionBanner } from "~/components/common/PermissionBanner";

const Shell: ParentComponent = (props) => {
  const { isOverlay, layout, settings } = useApp();
  return (
    <div
      class="h-full w-full"
      classList={{
        "bg-slate-950/85": !isOverlay(),
        "bg-transparent": isOverlay(),
      }}
      style={{ opacity: isOverlay() ? settings().overlayOpacity / 100 : 1 }}
    >
      <Show when={!isOverlay()} fallback={<Keyboard layout={layout} />}>
        <div class="flex h-full flex-col">
          <TopBar />
          <PermissionBanner />
          <main class="min-h-0 flex-1 p-4">{props.children}</main>
        </div>
      </Show>
    </div>
  );
};

export const AppLayout: ParentComponent = (props) => (
  <AppProvider>
    <PressedProvider>
      <CaptureProvider>
        <Shell>{props.children}</Shell>
      </CaptureProvider>
    </PressedProvider>
  </AppProvider>
);

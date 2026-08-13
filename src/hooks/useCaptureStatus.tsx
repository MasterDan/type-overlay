import {
  createContext,
  useContext,
  createSignal,
  onMount,
  type Accessor,
  type ParentComponent,
} from "solid-js";
import { useTauriEvent } from "~/hooks/useTauriEvent";
import { commands } from "~/lib/tauri";
import type { CaptureStatus, CaptureStatusEvent } from "~/types";

const CaptureContext = createContext<Accessor<CaptureStatus>>();

export const CaptureProvider: ParentComponent = (props) => {
  const [status, setStatus] = createSignal<CaptureStatus>("active");

  onMount(() => {
    commands.getCaptureStatus().then(setStatus).catch(() => {});
  });
  useTauriEvent<CaptureStatusEvent>("capture-status", (e) => setStatus(e.status));

  return (
    <CaptureContext.Provider value={status}>
      {props.children}
    </CaptureContext.Provider>
  );
};

export function useCaptureStatus(): Accessor<CaptureStatus> {
  const ctx = useContext(CaptureContext);
  if (!ctx) {
    throw new Error("useCaptureStatus must be used within CaptureProvider");
  }
  return ctx;
}

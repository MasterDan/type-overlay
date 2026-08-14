import {
  createContext,
  useContext,
  type ParentComponent,
} from "solid-js";
import { createStore, reconcile } from "solid-js/store";
import { useTauriEvent } from "~/hooks/useTauriEvent";
import type { PressedEvent } from "~/types";

type PressedMap = Record<string, boolean>;

const PressedContext = createContext<PressedMap>();

export const PressedProvider: ParentComponent = (props) => {
  const [pressed, setPressed] = createStore<PressedMap>({});
  // The backend owns the pressed-keys state and sends full snapshots with a
  // monotonically increasing `seq`. Apply only fresh snapshots and replace
  // the store wholesale, so reordered/duplicated/lost IPC events (e.g. when
  // toggling the overlay) can never leave a key visually stuck.
  let lastSeq = 0;
  useTauriEvent<PressedEvent>("pressed-changed", (e) => {
    if (e.seq <= lastSeq) return;
    lastSeq = e.seq;
    const next: PressedMap = {};
    for (const code of e.codes) next[code] = true;
    setPressed(reconcile(next));
  });
  return (
    <PressedContext.Provider value={pressed}>
      {props.children}
    </PressedContext.Provider>
  );
};

export function usePressed(): PressedMap {
  const ctx = useContext(PressedContext);
  if (!ctx) throw new Error("usePressed must be used within PressedProvider");
  return ctx;
}

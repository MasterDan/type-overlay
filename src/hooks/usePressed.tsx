import {
  createContext,
  useContext,
  type ParentComponent,
} from "solid-js";
import { createStore } from "solid-js/store";
import { useTauriEvent } from "~/hooks/useTauriEvent";
import type { KeyEvent } from "~/types";

type PressedMap = Record<string, boolean>;

const PressedContext = createContext<PressedMap>();

export const PressedProvider: ParentComponent = (props) => {
  const [pressed, setPressed] = createStore<PressedMap>({});
  useTauriEvent<KeyEvent>("key-down", (e) => setPressed(e.code, true));
  useTauriEvent<KeyEvent>("key-up", (e) => setPressed(e.code, false));
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

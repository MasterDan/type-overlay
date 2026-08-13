import { onCleanup, onMount } from "solid-js";
import { onEvent } from "~/lib/tauri";

export function useTauriEvent<T = unknown>(
  name: string,
  handler: (payload: T) => void,
): void {
  onMount(() => {
    let unlisten: (() => void) | undefined;
    onEvent<T>(name, handler).then((fn) => {
      unlisten = fn;
    });
    onCleanup(() => unlisten?.());
  });
}

import { type Component, createSignal, createEffect, onCleanup, Show } from "solid-js";
import { comboFromEvent, comboToDisplay } from "~/lib/settings";
import { IconButton } from "~/components/common/IconButton";

type Props = {
  combo: string;
  onRecord: (combo: string) => void;
};

export const HotkeyRecorder: Component<Props> = (props) => {
  const [recording, setRecording] = createSignal(false);
  let el: HTMLDivElement | undefined;

  const onKey = (e: KeyboardEvent) => {
    e.preventDefault();
    e.stopPropagation();
    if (e.code === "Escape") return setRecording(false);
    const combo = comboFromEvent(e);
    if (combo) {
      props.onRecord(combo);
      setRecording(false);
    }
  };

  createEffect(() => {
    if (!recording()) return;
    window.addEventListener("keydown", onKey, true);
    onCleanup(() => window.removeEventListener("keydown", onKey, true));
  });

  return (
    <div class="flex items-center gap-2">
      <div
        ref={el}
        tabindex="0"
        onFocus={() => setRecording(true)}
        onBlur={() => setRecording(false)}
        class="min-w-[128px] cursor-pointer rounded-lg border px-3 py-2 text-sm tabular-nums outline-none"
        classList={{
          "border-accent/60 bg-accent/10 text-white": recording(),
          "border-white/10 bg-black/30 text-white/70": !recording(),
        }}
      >
        <Show when={!recording()} fallback="Нажмите сочетание…">
          {comboToDisplay(props.combo)}
        </Show>
      </div>
      <IconButton onClick={() => (recording() ? setRecording(false) : el?.focus())}>
        <Show when={!recording()} fallback="Отмена">
          Изменить
        </Show>
      </IconButton>
    </div>
  );
};

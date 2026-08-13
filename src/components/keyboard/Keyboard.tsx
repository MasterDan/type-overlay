import { type Component, type Accessor, For, createMemo } from "solid-js";
import { createElementSize } from "@solid-primitives/resize-observer";
import { KEYS, BASE_W, BASE_H } from "~/lib/keyData";
import { Key } from "~/components/keyboard/Key";
import type { Layout } from "~/types";

type Props = {
  layout: Accessor<Layout>;
};

export const Keyboard: Component<Props> = (props) => {
  let wrap: HTMLDivElement | undefined;
  const size = createElementSize(() => wrap);
  const scale = createMemo(() => (size.width || BASE_W) / BASE_W);

  return (
    <div ref={wrap} class="h-full w-full">
      <div
        style={{
          width: `${BASE_W}px`,
          height: `${BASE_H}px`,
          transform: `scale(${scale()})`,
          "transform-origin": "top left",
        }}
      >
        <For each={KEYS}>{(def) => <Key def={def} layout={props.layout} />}</For>
      </div>
    </div>
  );
};

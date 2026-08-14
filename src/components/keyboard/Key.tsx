import { type Component, type Accessor } from "solid-js";
import { Show } from "solid-js";
import type { KeyDef } from "~/lib/keyData";
import { zoneOf, ZONE_COLORS } from "~/lib/keyData";
import { LAYOUTS } from "~/lib/layouts";
import { usePressed } from "~/hooks/usePressed";
import type { Layout } from "~/types";

type Props = {
  def: KeyDef;
  layout: Accessor<Layout>;
};

export const Key: Component<Props> = (props) => {
  const pressed = usePressed();
  const isPressed = () => !!pressed[props.def.code];
  const zoneColor = ZONE_COLORS[zoneOf(props.def.code)];

  return (
    <div
      style={{
        left: `${props.def.x}px`,
        top: `${props.def.y}px`,
        width: `${props.def.w}px`,
        height: `${props.def.h}px`,
        "border-color": isPressed() ? undefined : zoneColor,
        color: isPressed() ? undefined : zoneColor,
      }}
      classList={{
        "absolute box-border flex items-center justify-center border-[0.25px] rounded-[var(--kb-radius)] transition-colors duration-75":
          true,
        "border-accent/70 bg-accent/20 text-white shadow-[0_0_14px_-3px] shadow-accent/50":
          isPressed(),
      }}
    >
      <Show
        when={LAYOUTS[props.layout()][props.def.code]}
        fallback={
          <span class="text-[10px] font-light tracking-wide opacity-70 capitalize">
            {props.def.label}
          </span>
        }
      >
        {(glyph) => (
          <>
            <Show when={glyph().shift !== glyph().main.toUpperCase()}>
              <span class="absolute right-[5px] top-[2px] text-[9px] font-light">
                {glyph().shift}
              </span>
            </Show>
            <span class="text-[13px] font-light">{glyph().main}</span>
          </>
        )}
      </Show>
    </div>
  );
};

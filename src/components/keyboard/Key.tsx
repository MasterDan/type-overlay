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
  const zoneSoft = `color-mix(in srgb, ${zoneColor} 12%, transparent)`;

  return (
    <div
      style={{
        left: `${props.def.x}px`,
        top: `${props.def.y}px`,
        width: `${props.def.w}px`,
        height: `${props.def.h}px`,
        "border-color": isPressed() ? undefined : zoneColor,
        color: isPressed() ? undefined : zoneColor,
        "background-color": isPressed() ? undefined : zoneSoft,
      }}
      classList={{
        "absolute box-border flex items-center justify-center border rounded-[var(--kb-radius)] transition-colors duration-75":
          true,
        "border-accent/70 bg-accent/20 text-white shadow-[0_0_14px_-3px] shadow-accent/50":
          isPressed(),
      }}
    >
      <Show
        when={LAYOUTS[props.layout()][props.def.code]}
        fallback={
          <span class="text-[10px] tracking-wide opacity-70 capitalize">
            {props.def.label}
          </span>
        }
      >
        {(glyph) => (
          <>
            <Show when={glyph().shift !== glyph().main.toUpperCase()}>
              <span class="absolute right-[5px] top-[2px] text-[9px]">
                {glyph().shift}
              </span>
            </Show>
            <span class="text-[13px] font-medium">{glyph().main}</span>
          </>
        )}
      </Show>
    </div>
  );
};

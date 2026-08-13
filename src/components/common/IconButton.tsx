import { type Component, type ComponentProps, splitProps } from "solid-js";

type Props = ComponentProps<"button"> & {
  active?: boolean;
};

export const IconButton: Component<Props> = (fullProps) => {
  const [props, attrs] = splitProps(fullProps, ["active", "class", "children"]);
  return (
    <button
      {...attrs}
      classList={{
        "flex items-center justify-center gap-2 rounded-lg border px-3 py-2 text-sm transition-colors duration-100": true,
        "border-white/10 bg-white/5 text-white/70 hover:bg-white/10 hover:text-white":
          !props.active,
        "border-accent/50 bg-accent/15 text-white": !!props.active,
        [props.class ?? ""]: !!props.class,
      }}
    >
      {props.children}
    </button>
  );
};

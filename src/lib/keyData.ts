export const BASE_W = 878;
export const BASE_H = 403;

export type KeyKind = "glyph" | "wide" | "modifier" | "fn" | "arrow";

export interface KeyDef {
  code: string;
  x: number;
  y: number;
  w: number;
  h: number;
  kind: KeyKind;
  label: string;
}

const K = (
  code: string,
  x: number,
  y: number,
  w: number,
  h: number,
  kind: KeyKind = "glyph",
  label = "",
): KeyDef => ({ code, x, y, w, h, kind, label });

export const KEYS: readonly KeyDef[] = [
  // Digit row
  K("Backquote", 60, 60, 45, 52),
  K("Digit1", 107, 60, 45, 52),
  K("Digit2", 154, 60, 45, 52),
  K("Digit3", 201, 60, 45, 52),
  K("Digit4", 248, 60, 45, 52),
  K("Digit5", 295, 60, 45, 52),
  K("Digit6", 342, 60, 45, 52),
  K("Digit7", 389, 60, 45, 52),
  K("Digit8", 436, 60, 45, 52),
  K("Digit9", 483, 60, 45, 52),
  K("Digit0", 530, 60, 45, 52),
  K("Minus", 577, 60, 45, 52),
  K("Equal", 624, 60, 45, 52),
  K("Backspace", 716, 60, 102, 52, "wide", "⌫"),

  // Tab row
  K("Tab", 60, 128, 67, 52, "wide", "tab"),
  K("KeyQ", 132, 128, 45, 52),
  K("KeyW", 179, 128, 45, 52),
  K("KeyE", 226, 128, 45, 52),
  K("KeyR", 273, 128, 45, 52),
  K("KeyT", 320, 128, 45, 52),
  K("KeyY", 367, 128, 45, 52),
  K("KeyU", 414, 128, 45, 52),
  K("KeyI", 461, 128, 45, 52),
  K("KeyO", 508, 128, 45, 52),
  K("KeyP", 555, 128, 45, 52),
  K("BracketLeft", 602, 128, 45, 52),
  K("BracketRight", 649, 128, 45, 52),
  K("Backslash", 741, 128, 77, 52),

  // Caps row
  K("CapsLock", 60, 196, 82, 52, "modifier", "caps"),
  K("KeyA", 147, 196, 45, 52),
  K("KeyS", 194, 196, 45, 52),
  K("KeyD", 241, 196, 45, 52),
  K("KeyF", 288, 196, 45, 52),
  K("KeyG", 335, 196, 45, 52),
  K("KeyH", 382, 196, 45, 52),
  K("KeyJ", 429, 196, 45, 52),
  K("KeyK", 476, 196, 45, 52),
  K("KeyL", 523, 196, 45, 52),
  K("Semicolon", 570, 196, 45, 52),
  K("Quote", 617, 196, 45, 52),
  K("Enter", 700, 196, 118, 52, "wide", "⏎"),

  // Shift row
  K("ShiftLeft", 60, 260, 105, 52, "modifier", "shift"),
  K("KeyZ", 170, 260, 45, 52),
  K("KeyX", 217, 260, 45, 52),
  K("KeyC", 264, 260, 45, 52),
  K("KeyV", 311, 260, 45, 52),
  K("KeyB", 358, 260, 45, 52),
  K("KeyN", 405, 260, 45, 52),
  K("KeyM", 452, 260, 45, 52),
  K("Comma", 499, 260, 45, 52),
  K("Period", 546, 260, 45, 52),
  K("Slash", 593, 260, 45, 52),
  K("ShiftRight", 676, 260, 142, 52, "modifier", "shift"),

  // Bottom row
  K("Fn", 60, 323, 45, 52, "modifier", "fn"),
  K("ControlLeft", 107, 323, 58, 52, "modifier", "ctrl"),
  K("MetaLeft", 167, 323, 48, 52, "modifier", "⌘"),
  K("AltLeft", 217, 323, 48, 52, "modifier", "⌥"),
  K("Space", 267, 323, 280, 52, "wide", "space"),
  K("AltRight", 549, 323, 48, 52, "modifier", "⌥"),
  K("ContextMenu", 614, 323, 58, 52, "wide", "☰"),
  K("ControlRight", 674, 323, 76, 52, "modifier", "ctrl"),
];

export type Zone =
  | "z1"
  | "z2"
  | "z3"
  | "z4"
  | "z5"
  | "z6"
  | "z7"
  | "z8"
  | "ztl"
  | "ztr"
  | "sp"
  | "zn";

export const ZONE_COLORS: Record<Zone, string> = {
  z1: "#d2691e",
  z2: "#c7aee2",
  z3: "#ddd789",
  z4: "#8cc41e",
  z5: "#e8cfa8",
  z6: "#b48ce2",
  z7: "#ef9430",
  z8: "#a6cedc",
  ztl: "#5b78e0",
  ztr: "#6c2fc0",
  sp: "#6454d0",
  zn: "#b9bec7",
};

const ZONE_OF: Record<string, Zone> = {
  Backquote: "z1",
  Tab: "z1",
  KeyQ: "z1",
  CapsLock: "z1",
  KeyA: "z1",
  ShiftLeft: "z1",
  Fn: "z1",
  ControlLeft: "z1",

  Digit1: "z2",
  Digit2: "z2",
  KeyW: "z2",
  KeyS: "z2",
  KeyX: "z2",

  Digit3: "z3",
  Digit4: "z3",
  KeyE: "z3",
  KeyD: "z3",
  KeyC: "z3",

  Digit5: "z4",
  Digit6: "z4",
  KeyR: "z4",
  KeyT: "z4",
  KeyF: "z4",
  KeyG: "z4",
  KeyV: "z4",
  KeyB: "z4",

  Digit7: "z5",
  KeyY: "z5",
  KeyU: "z5",
  KeyH: "z5",
  KeyJ: "z5",
  KeyN: "z5",
  KeyM: "z5",

  Digit8: "z6",
  Digit9: "z6",
  KeyI: "z6",
  KeyK: "z6",
  Comma: "z6",

  Digit0: "z7",
  KeyO: "z7",
  KeyL: "z7",
  Period: "z7",

  Minus: "z8",
  Equal: "z8",
  Backspace: "z8",
  KeyP: "z8",
  BracketLeft: "z8",
  BracketRight: "z8",
  Backslash: "z8",
  Semicolon: "z8",
  Quote: "z8",
  Enter: "z8",
  Slash: "z8",
  ShiftRight: "z8",

  AltLeft: "ztl",
  AltRight: "ztr",
  Space: "sp",
};

export const zoneOf = (code: string): Zone => ZONE_OF[code] ?? "zn";

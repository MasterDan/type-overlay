import type { HotkeyAction, Settings } from "~/types";

export const DEFAULT_SETTINGS: Settings = {
  hotkeys: {
    "toggle-mode": "Control+Shift+KeyO",
    "toggle-overlay": "Control+Shift+KeyH",
  },
};

export const HOTKEY_META: Record<
  HotkeyAction,
  { title: string; description: string }
> = {
  "toggle-mode": {
    title: "Переключить режим",
    description: "Окно ↔ оверлей",
  },
  "toggle-overlay": {
    title: "Показать / скрыть оверлей",
    description: "Не возвращаясь в окно",
  },
};

export const HOTKEY_ACTIONS = Object.keys(HOTKEY_META) as HotkeyAction[];

const MOD_CODES = new Set([
  "ControlLeft",
  "ControlRight",
  "ShiftLeft",
  "ShiftRight",
  "AltLeft",
  "AltRight",
  "MetaLeft",
  "MetaRight",
]);

const CODE_TO_TOKEN: Record<string, string> = {};

for (let c = 0; c < 26; c++) {
  CODE_TO_TOKEN[`Key${String.fromCharCode(65 + c)}`] = `Key${String.fromCharCode(
    65 + c,
  )}`;
}
for (let n = 0; n <= 9; n++) CODE_TO_TOKEN[`Digit${n}`] = `Digit${n}`;
Object.assign(CODE_TO_TOKEN, {
  Space: "Space",
  Enter: "Enter",
  Tab: "Tab",
  Escape: "Escape",
  Backspace: "Backspace",
  Minus: "Minus",
  Equal: "Equal",
  Backquote: "Backquote",
});

const MOD_FLAGS: Array<[string, keyof KeyboardEvent]> = [
  ["Control", "ctrlKey"],
  ["Alt", "altKey"],
  ["Shift", "shiftKey"],
  ["Super", "metaKey"],
];

export function comboFromEvent(e: KeyboardEvent): string | null {
  if (MOD_CODES.has(e.code)) return null;
  const token = CODE_TO_TOKEN[e.code];
  if (!token) return null;
  const mods = MOD_FLAGS.filter(([, flag]) => e[flag]).map(([m]) => m);
  return [...mods, token].join("+");
}

const TOKEN_DISPLAY: Record<string, string> = {
  Control: "Ctrl",
  Alt: "Alt",
  Shift: "Shift",
  Super: "⌘",
  Space: "Space",
  Enter: "⏎",
  Tab: "Tab",
  Escape: "Esc",
  Backspace: "⌫",
};

export function comboToDisplay(combo: string): string {
  return combo
    .split("+")
    .map((part) => {
      if (part.startsWith("Key")) return part.slice(3);
      if (part.startsWith("Digit")) return part.slice(5);
      return TOKEN_DISPLAY[part] ?? part;
    })
    .join(" + ");
}

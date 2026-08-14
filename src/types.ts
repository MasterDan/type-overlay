export type AppMode = "window" | "overlay";
export type Layout = "en" | "ru";
export type HotkeyAction = "toggle-mode" | "toggle-overlay";
export type CaptureStatus = "active" | "waiting";

export interface Settings {
  hotkeys: Record<HotkeyAction, string>;
  overlayOpacity: number;
}

export interface KeyGlyph {
  main: string;
  shift: string;
}

export type KeyGlyphMap = Partial<Record<string, KeyGlyph>>;

export interface PressedEvent {
  seq: number;
  codes: string[];
}

export interface CaptureStatusEvent {
  status: CaptureStatus;
}

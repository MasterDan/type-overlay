export type Platform = "macos" | "windows" | "linux" | "unknown";

export interface PlatformTheme {
  accent: string;
  accentSoft: string;
  keyRadius: string;
  plateRadius: string;
  font: string;
}

export const PLATFORM_THEMES: Record<Platform, PlatformTheme> = {
  macos: {
    accent: "#0a84ff",
    accentSoft: "#66bfff",
    keyRadius: "8px",
    plateRadius: "22px",
    font: '-apple-system, "SF Pro Text", "SF Pro Display", system-ui, sans-serif',
  },
  windows: {
    accent: "#4cc2ff",
    accentSoft: "#7fd8ff",
    keyRadius: "4px",
    plateRadius: "10px",
    font: '"Segoe UI Variable", "Segoe UI", system-ui, sans-serif',
  },
  linux: {
    accent: "#3584e4",
    accentSoft: "#62a0ea",
    keyRadius: "7px",
    plateRadius: "14px",
    font: '"Cantarell", "Ubuntu", "Inter", system-ui, sans-serif',
  },
  unknown: {
    accent: "#38bdf8",
    accentSoft: "#7dd3fc",
    keyRadius: "7px",
    plateRadius: "16px",
    font: "system-ui, -apple-system, sans-serif",
  },
};

export function platformFromOs(os: string): Platform {
  switch (os) {
    case "macos":
      return "macos";
    case "windows":
      return "windows";
    case "linux":
      return "linux";
    default:
      return "unknown";
  }
}

export function applyTheme(platform: Platform): void {
  const t = PLATFORM_THEMES[platform];
  const root = document.documentElement;
  root.style.setProperty("--color-accent", t.accent);
  root.style.setProperty("--color-accent-soft", t.accentSoft);
  root.style.setProperty("--kb-radius", t.keyRadius);
  root.style.setProperty("--kb-plate-radius", t.plateRadius);
  root.style.setProperty("--font-sans", t.font);
}

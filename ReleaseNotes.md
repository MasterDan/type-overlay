# Release Notes

## v0.2.2
- **minor fixes** fixed github actions

## v0.2.0

### Added

- **Multi-monitor overlay** — secondary overlay windows are spawned on every additional monitor; each shows the keyboard scaled to that screen. They are created and destroyed on mode switches.
- **Configurable overlay opacity** — new slider in Settings (`overlayOpacity`, 0–100%, persisted alongside hotkeys).

### Fixed

- **Stuck keys** — pressed-keys state now lives in the backend (`pressed.rs`). Full snapshots with a monotonically increasing `seq` replace the per-key `key-down`/`key-up` events; the frontend applies only fresh snapshots, so reordered/duplicated/lost IPC events (e.g. when toggling the overlay) can no longer leave a key visually stuck.
- **Hotkey combo keys staying highlighted** — a registered global shortcut swallows the physical key-ups of its combo, so the pressed set is cleared when a hotkey action runs.
- **DWM shadow around the overlay on Windows** — the transparent overlay no longer has a shadow/rounded frame; the shadow is restored in window mode.

### Changed

- `KeyEvent` (`key-down` / `key-up`) events replaced by a single `pressed-changed` snapshot event (`{ seq, codes }`).
- Secondary overlay windows detect their role from the window label at startup instead of relying on the missed initial `mode-changed` event.

### Misc

- Arch package: `options=('!debug')` to strip the debug package.

## v0.1.0

Initial release.

### Features

- **Live key highlighting** — every physical key press lights up instantly.
- **Two modes:**
  - **Window** — a regular resizable app window with settings and keyboard preview.
  - **Overlay** — the same window becomes transparent, frameless, always-on-top and click-through, scaled across the screen.
- **Bilingual layouts** — Russian (ЙЦУКЕН) and English (QWERTY) glyphs, including shifted symbols.
- **Automatic layout detection** — follows the active OS layout (macOS, Windows, X11); manual toggle on Wayland.
- **Per-OS theming** — accent color, key radius and font adapt to the platform (macOS, Windows Fluent, GNOME/Ubuntu).
- **Configurable global hotkeys** — toggle mode / overlay visibility from anywhere; combos are recorded in Settings.

### Platform notes

- macOS: key capture requires Accessibility permission; the build is not code-signed.
- Wayland: always-on-top/click-through may degrade depending on the compositor; layout auto-detection is unreliable.

### CI

- Release workflow publishing Windows (NSIS), macOS (universal DMG) and Linux (deb/rpm/AppImage/Arch) artifacts.

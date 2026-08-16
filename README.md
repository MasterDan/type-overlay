# Type Overlay

A cross-platform on-screen keyboard overlay that visualizes your physical keyboard and highlights keys in real time as you press them. Built for typing demos, streaming, screen recording and teaching.

Switch from the regular **window** to a transparent, always-on-top, click-through **overlay** that stretches across the screen — your viewers see the keys you press without the app stealing focus or clicks.

Built with **Tauri v2 + SolidJS + TypeScript**. Runs on **macOS**, **Windows** and **Linux**.

[![Release](https://github.com/MasterDan/type-overlay/actions/workflows/release.yml/badge.svg)](https://github.com/MasterDan/type-overlay/actions/workflows/release.yml)

---

## Features

- **Live key highlighting** — every key you press lights up instantly.
- **Two modes:**
  - **Window** — a normal resizable app window with a settings panel and the keyboard preview.
  - **Overlay** — the same window becomes transparent, frameless, always-on-top and click-through, scaled across the screen. Perfect for going live.
- **Bilingual layouts** — Russian (ЙЦУКЕН) and English (QWERTY) glyphs, including shifted symbols.
- **Automatic layout detection** — the on-screen keyboard follows your active OS layout (macOS, Windows, X11). On Wayland, where detection is unreliable, use the manual layout toggle in the top bar.
- **Per-OS theming** — accent color, key corner radius and font adapt to the platform (macOS, Windows Fluent, GNOME/Ubuntu) to look native everywhere.
- **Configurable global hotkeys** — switch modes or toggle the overlay visibility from anywhere, even while the app is in the background. Combos are recorded directly in the Settings page.

---

## Download

Pre-built binaries are published on the [Releases page](https://github.com/MasterDan/type-overlay/releases) for every release:

| Platform      | Artifact                              |
| ------------- | ------------------------------------- |
| Windows       | `*-setup.exe` (NSIS installer)        |
| macOS         | `*.dmg` (universal — Intel + Apple Silicon) |
| Linux         | `*.deb`, `*.rpm`, `*.AppImage`        |
| Arch / Manjaro| `*.pkg.tar.zst` (install with `pacman -U`) |

> The macOS build is signed with an **ad-hoc signature** (no Apple Developer certificate), so Gatekeeper will warn on first launch — see [Installing on macOS](#installing-on-macos) below.

---

## Installing on macOS

The app is not notarized (no Apple Developer license), so macOS blocks it by default and the key-capture permissions need to be granted manually. Follow these steps once after installing, and re-do **step 4** after every new downloaded version.

**1. Install**

Download `*.dmg` from the [Releases page](https://github.com/MasterDan/type-overlay/releases), open it and drag **Type Overlay** into `/Applications`.

**2. Bypass Gatekeeper**

Remove the quarantine flag from the installed app (or right-click it → *Open* → *Open*):

```sh
xattr -cr "/Applications/Type Overlay.app"
```

**3. Grant permissions**

Launch the app. It will prompt for two permissions — allow both, then **restart the app** (permissions are only picked up at startup):

- **System Settings → Privacy & Security → Accessibility** → enable *Type Overlay*
- **System Settings → Privacy & Security → Input Monitoring** → enable *Type Overlay*

If the prompts don't appear, add the app manually with the **+** button (you may need to press `Cmd+Shift+G` and type `/Applications/Type Overlay.app`).

**4. After installing a new version**

Ad-hoc signatures change with every build, so macOS treats each new version as a different app and the old permissions no longer match. If the "no access" banner reappears after an update, reset and re-grant:

```sh
tccutil reset Accessibility com.typeoverlay.app
tccutil reset ListenEvent com.typeoverlay.app
```

Then launch the app and grant the two permissions from **step 3** again.

> **Note:** if the banner still won't go away, verify the bundle is signed and re-sign it manually:
> ```sh
> codesign --verify "/Applications/Type Overlay.app" || codesign --force --deep -s - "/Applications/Type Overlay.app"
> ```
> Then repeat **step 4**.

---

## Installing on Windows

The app needs **no special permissions** on Windows — global key capture works out of the box.

**1. Install**

Download the `*-setup.exe` installer from the [Releases page](https://github.com/MasterDan/type-overlay/releases), run it and follow the steps.

**2. Bypass SmartScreen**

The installer is not code-signed, so Windows may show *"Windows protected your PC"*. Click **More info** → **Run anyway**.

---

## Installing on Linux

Pick the package matching your distribution — all are published on the [Releases page](https://github.com/MasterDan/type-overlay/releases). No special permissions are required; on Wayland see the notes below.

**Debian / Ubuntu (`.deb`)**

```sh
sudo apt install ./type-overlay_*_amd64.deb
```

**Fedora / RHEL (`.rpm`)**

```sh
sudo dnf install ./type-overlay-*.rpm
```

**Arch / Manjaro (`.pkg.tar.zst`)**

```sh
sudo pacman -U type-overlay-*.pkg.tar.zst
```

**Any distro (`.AppImage`)**

```sh
chmod +x 'Type Overlay'*_amd64.AppImage
'./Type Overlay'*_amd64.AppImage
```

> On Ubuntu 22.04+ AppImages need libfuse2: `sudo apt install libfuse2`.

**Wayland users**

On Wayland the overlay uses wlr-layer-shell where the compositor supports it (Hyprland, Sway, …) and falls back to sticky windows elsewhere (e.g. GNOME). Layout auto-detection is unreliable on Wayland — use the manual layout toggle in the top bar. See [Platform notes](#platform-notes).

---

## Requirements

To build and run from source you need:

- **[Node.js](https://nodejs.org/)** 20+ and **[pnpm](https://pnpm.io/)** 11+
- **[Rust](https://www.rust-lang.org/)** (stable toolchain)
- OS-specific toolchain (see below)

### OS-specific prerequisites

**macOS** — install the Xcode Command Line Tools:
```sh
xcode-select --install
```

**Windows** — install the [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) (MSVC) and **WebView2** (preinstalled on Windows 11).

**Linux (Debian/Ubuntu):**
```sh
sudo apt install libwebkit2gtk-4.1-dev librsvg2-dev \
  patchelf libssl-dev libgtk-3-dev libayatana-appindicator3-dev \
  libgtk-layer-shell-dev
```

> `libgtk-layer-shell-dev` (needed for the Wayland overlay) is unavailable on Ubuntu 22.04 — build it from source the same way the [CI does](.github/workflows/release.yml): `meson setup build gtk-layer-shell -Dprefix=/usr && sudo ninja -C build install`.

**Linux (Arch / Manjaro):**
```sh
sudo pacman -S webkit2gtk-4.1 gtk3 openssl librsvg libayatana-appindicator base-devel gtk-layer-shell
```

---

## Getting started

Install dependencies:

```sh
pnpm install
```

Run the app in development (hot-reload of the frontend + Rust backend):

```sh
pnpm tauri dev
```

---

## Build

Type-check and build the frontend only:

```sh
pnpm build
```

Build a release bundle (frontend + Rust + installer artifacts in `src-tauri/target/release/bundle/`):

```sh
pnpm tauri build
```

Check the Rust backend without bundling:

```sh
cargo check --manifest-path src-tauri/Cargo.toml
```

Regenerate the app icons from `app-icon.png`:

```sh
pnpm icon
```

---

## Configuration

Open the **Settings** page (gear icon in the top bar) to rebind the global hotkeys. Defaults:

| Action                        | Default combo        |
| ----------------------------- | -------------------- |
| Toggle mode (Window ↔ Overlay)| `Ctrl + Shift + O`   |
| Show / hide the overlay       | `Ctrl + Shift + H`   |

On macOS, replace `Ctrl` with `⌘` (Super) in the recorded combo if you prefer.

---

## Platform notes

- **macOS** — key capture requires **Accessibility** permission (and on some macOS versions also **Input Monitoring**). Grant them under *System Settings → Privacy & Security*. If you launch the raw debug binary directly, code-signing resets the permission — always run via `pnpm tauri dev` / `pnpm tauri build` and re-grant access after a rebuild. Downloaded (ad-hoc signed) builds also require re-granting after each update — see [Installing on macOS](#installing-on-macos). A banner in the app reminds you when access is missing.
- **Windows / Linux** — key capture uses `rdev` and needs no special permissions.
- **Wayland** — always-on-top and click-through are not universally supported by compositors, so the overlay may degrade. Layout auto-detection is also unreliable on Wayland; use the manual layout toggle in the top bar.

---

## How it works

A **single** `main` window powers both modes:

- **Window mode** — renders the top bar, a keyboard preview and the settings page.
- **Overlay mode** — the backend makes the window transparent, frameless, always-on-top and click-through, then scales the keyboard across the screen.

The backend captures key events globally (a hand-rolled `CGEventTap` on macOS, `rdev` elsewhere), detects the active layout, and emits `key-down` / `key-up` / `layout-changed` / `mode-changed` events to the frontend. The frontend never calls `invoke` directly — everything goes through the typed `commands.*` wrappers in `src/lib/tauri.ts`.

### Project layout

```
src/                  SolidJS frontend
  components/keyboard  Keyboard grid + Key
  components/common    IconButton, LayoutBadge, PermissionBanner
  components/settings  HotkeyRecorder
  hooks/               useTauriEvent, usePressed, useLayout, useAppMode, useSettings
  lib/                 keyData, layouts, theme, settings, tauri wrappers
  store/app.tsx        Single useApp() context
src-tauri/src/        Rust backend
  keyboard.rs          Global key capture (CGEventTap / rdev)
  layout.rs            Active layout detection (Carbon / WinAPI / XKB)
  hotkeys.rs           Global shortcut registration
  mode.rs              Window ↔ overlay transform
  commands.rs          Tauri commands
```

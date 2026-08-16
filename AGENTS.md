### Project structure (type-overlay)

- **Stack:** Tauri v2 + SolidJS + TypeScript + Vite + Tailwind v4 + solid-icons. Package manager — **pnpm**.
- **Commands:**
  - `pnpm tauri dev` — run the app.
  - `pnpm build` — typecheck (`tsc --noEmit`) + build the frontend into `dist/`. **Type checking is `pnpm build`.**
  - `cargo check --manifest-path src-tauri/Cargo.toml` — backend check.
  - `pnpm tauri build` — full release bundle (frontend + Rust + installer); artifacts land in `src-tauri/target/release/bundle/`.
  - `pnpm icon` — regenerate icons (`scripts/gen-icon.mjs` draws `app-icon.png`, then `tauri icon`).

#### Frontend (`src/`)

- `index.tsx` — entry: `<Router root={AppLayout}>` with routes `/` (`MainView`) and `/settings` (`SettingsPage`).
- `App.tsx` — `AppLayout`: wraps everything in `AppProvider` + `PressedProvider`, renders `Shell` (window chrome vs. transparent overlay keyboard).
- `store/app.tsx` — single `useApp()` context: creates layout/mode/settings/platform **exactly once** and applies the per-OS theme.
- `hooks/` — `useTauriEvent` (generic backend-event listener), `usePressed` (fine-grained pressed-keys store via context), `useLayout`, `useAppMode`, `useSettings`.
- `lib/` — `keyData` (key coords + `kind` parsed from `Keyboard.md`; `BASE_W`/`BASE_H`), `layouts` (RU/EN glyphs `{ main, shift }`), `tauri` (typed `invoke` wrappers + `onEvent`), `settings` (default hotkeys + combo formatting/recording), `theme` (per-OS accent/radius/font via CSS vars).
- `components/keyboard/` — `Keyboard` (scales to its container) + `Key` (one key; reads pressed state + layout glyph).
- `components/common/` — `IconButton`, `LayoutBadge` (also a manual layout-cycle button).
- `components/settings/HotkeyRecorder` — captures a hotkey combo.
- `components/TopBar.tsx`, `pages/MainView.tsx`, `pages/SettingsPage.tsx`.

#### Backend (`src-tauri/src/`)

- `main.rs` → `lib.rs::run()` — Tauri builder, plugins (`tauri-plugin-store`, `tauri-plugin-global-shortcut`), managed `AppState`, `setup` (starts the key listener + layout poller), `invoke_handler`.
- `models.rs` — serde types for events/commands (`AppMode`, `HotkeyAction`, `KeyEvent`, `ModeEvent`, `LayoutEvent`).
- `state.rs` — `AppState` (`mode`, `overlay_visible`).
- `commands.rs` — `#[tauri::command]`: `set_app_mode`, `set_overlay_visible`, `register_hotkeys`, `get_layout`, `get_platform`, `check_accessibility`.
- `mode.rs` — window↔overlay transform (`apply_mode`) + `set_overlay_visible`. In overlay: macOS = the main window is pinned to all Spaces incl. fullscreen (`spaces.rs`); Linux Wayland = one `layer_shell.rs` layer surface per monitor, the main window stays hidden (fallback: X11 sticky windows).
- `spaces.rs` (macOS) — NSWindow collection behavior (`FullScreenAuxiliary`) + status window level via `objc2`.
- `layer_shell.rs` (Linux) — wlr-layer-shell overlay surfaces (Hyprland/Sway); `gtk_layer_init_for_window` must run before the window is mapped; needs the system `gtk-layer-shell` library to build.
- `actions.rs` — `run_action(action)` — what each hotkey does.
- `hotkeys.rs` — global-shortcut registration via `on_shortcut` (one handler per combo).
- `keyboard.rs` — global key capture: **macOS** = hand-rolled `CGEventTap` on the main run loop (reads only `keyCode`; rdev's macOS backend traps inside HIToolbox/TSM and crashes, so it's not used here); **Windows/Linux** = `rdev`.
- `layout.rs` — layout detection: macOS (Carbon TIS), Windows (`GetKeyboardLayout` of the foreground window), Linux (`XKB_DEFAULT_LAYOUT` heuristic). The poller runs detection **on the main thread** via `run_on_main_thread` (Carbon/HIToolbox asserts the main queue).

#### Data flow

- Backend → frontend (events): `key-down` / `key-up` (`{ code }`), `layout-changed`, `mode-changed`. Listened via `useTauriEvent`.
- Frontend → backend: **only** through the typed wrappers `commands.*` in `lib/tauri.ts` (no bare `invoke` in components).
- Modes: a **single** `main` window. In overlay it becomes transparent + always-on-top + click-through + frameless and scales across the screen; return via the global hotkey.

#### Cross-platform

- All native FFI is behind `#[cfg(target_os = "...")]`; `core-foundation` is a macOS-only target dependency. Build per-OS (Tauri does not cross-compile GUI bundles).
- macOS: `macOSPrivateApi: true` in `tauri.conf.json` (required for a transparent window); key capture needs **Accessibility** (and on some macOS versions also **Input Monitoring**). A debug binary run directly traps on code-signing — always launch via `tauri dev` / `tauri build`.
- Wayland: always-on-top and click-through are not universally supported by compositors — the overlay may degrade; layout auto-detection is unreliable (use the manual toggle in `LayoutBadge`).

---

### SolidJS rules

#### Components

- **Named exports.** Exception — components loaded via `lazy()`.
- **Arrow functions** + a SolidJS type: `Component` (no children), `ParentComponent` (with children), `FlowComponent` (non-standard children).
- **Simple properties — inline**, without a separate type:
  ```tsx
  const MyComponent: Component<{ prop1: string; prop2: number }> = (props) => { ... }
  ```
  A separate `Props` — only when inherited (from `ComponentProps<'div'>`) or exported.
- **Inheriting HTML-element props.** If the root is an HTML tag, `Props` inherits from `ComponentProps<'tag-name'>`, the parameter is named `fullProps` and split with `splitProps(fullProps, [...keys])` into `[props, attrs]`. `props` — "own" keys (listed explicitly; dynamic classes go into `classList`); `attrs` — everything else, spread onto the root via `{...attrs}` without inspection.
  ```tsx
  type Props = ComponentProps<'div'> & { active: boolean };
  const MyComp: Component<Props> = (fullProps) => {
    const [props, attrs] = splitProps(fullProps, ['active', 'class']);
    return (
      <div {...attrs} classList={{ active: props.active, [props.class ?? '']: !!props.class }} />
    );
  };
  ```
- **Do not use `JSX.*` types.** Instead of `JSX.Element` → `jsxElement`, instead of `JSX.HTMLAttributes` → `ComponentProps`.
- **No branching via early `return` or `{cond() && <JSX/>}`.** Only control-flow components: `Show` (there's an obvious main scenario, otherwise `fallback`), `Switch`+`Match` (equivalent variants, even if there are only two), `ErrorBoundary`/`Suspense` (errors and async).
- **CSS classes.** Tailwind/DaisyUI don't parse dynamically assembled strings — don't concatenate from pieces; use `clsx` / conditions via `classList`.
- **`class` and `classList` are mutually exclusive:** static only → `class="..."`; with dynamics → only `classList` (the static part can be set with a `true` flag).
- **Nested components** — a common prefix (`Card`, `CardTitle`, `CardHeader`), **each exported separately**, without a default export with attached subfields.

#### `props` reactivity

The component body runs **once**, therefore:

- reactive reads of `props.x` — only inside JSX, `createEffect`/`createMemo`, or via `() => props.x`;
- a tricky case — a "derived" flag `const isX = props.x !== undefined` fixes the value at mount time. Make it a function: `const isX = () => props.x !== undefined`;
- destructuring in the signature (`{ foo }`) "freezes" — always read from `props`;
- exception: properties that **definitely** don't change by contract (static configs, callbacks without overwriting) — can be read directly. Rare; in doubt — go reactive.
- **Inline literals (`[...]`, `{...}`) with JSX inside — don't pass in props; hoist to `const`/`createMemo`.** SolidJS compiles `items={[...]}` into a getter `get items() { return [...] }` that creates a **new** array (and new `createComponent` calls inside) on **every** read. If the consumer reads the prop more than once (e.g. `<For each={items}>` + `.find()` somewhere else) — components are recreated and remounted. Symptoms: "doubled" DOM, lost focus in inputs, extra `onMount`/`onCleanup`.

#### Effects (`createEffect`)

- **Wire via effects, not manual propagation.** If `y` depends on `x` — `y` is computed in an effect subscribed to `x`, not via two consecutive updates. The link is explicit and works regardless of the source of `x`'s change.
- **A signal call = a subscription.** Every `signal()` inside an effect body registers a dependency. Write it so it doesn't fire idle: `local.onCheckedChange?.(internalChecked())` — no callback → no call → no subscription.
- **The flip side:** "lazy" subscription via a condition loses reactivity when the absence of a value is temporary. If the callback can appear later — read it **before** the dependent signal, unconditionally (idle calls, but guaranteed reactivity). Choose by contract.
- **Don't mutate a signal you just subscribed to** — `A -> setA` runs into an infinite loop. Need a link → an exit condition (compare with the old value) or `on(signal, fn, { defer: true })`.

#### Two-way binding (controlled prop ↔ internal signal)

When a component holds an internal signal synced with a controlled prop (`opened`/`setOpened`, `checked`/`onCheckedChange`, …) — only via the `createModel` hook:

```ts
const [value, setValue] = createModel([props.value, onChange]);
```

Rules:

- **after `createModel` the prop is no longer read** — the entire component works only with the internal `value()` / `setValue`; only the hook itself looks at the prop (and callback);

#### Hooks

1. **Name and file**: `useXxx` / `useXxx.ts`.
2. **Location**: `src/hooks/` — generic (`useFilter`, `useVault`, `useLocalization`); `src/hooks/data/` — those calling `executeCommand`.
3. **Signal sources — from the outside.** A data hook accepts an already created signal as a parameter (`useTypes(filter: Accessor<SimpleFilter>)`), it doesn't create one itself. The signal owner is the page/component. Signal factories — a separate hook (`useFilter`).
4. **`equals` for structural signals.** For objects with fields — a custom `equals` comparing fields; otherwise `setFilter({...})` on a new reference pointlessly triggers `createResource`.
5. **Data — via `createResource(source, fetcher)`** (client; `createAsync`/`query` from SSR frameworks don't apply). Auto-refetch on source change. Exposed: `data` (Resource), actions (`createX`, `refresh`), `loading`, `error`. **Mutations** — via `executeCommand`; afterwards — `refetch`/`refresh` the resource (if needed — a targeted `mutate`). No `revalidate()`/server RPCs — those are SSR constructs.
6. **Don't return inputs back** — the signal parameter is already at the caller's.
7. **One hook — one task.** Don't mix independent resources.

#### `@solid-primitives` and ready-made primitives

Ready-made primitives are the priority for typical tasks (debounce/throttle, resize/viewport, i18n). Before writing your own hook, check the catalog; **self-written duplicates are forbidden**.

1. Each primitive is a separate package (`pnpm add`). In the project: `@solid-primitives/scheduled` (`debounce`/`throttle`/`scheduleIdle`/`createScheduled`), `@solid-primitives/resize-observer` (`createElementSize`/`createResizeObserver`/`createWindowSize`), `@solid-primitives/i18n`.
2. `@elysiumorg/solid-use` is also available — check its set before writing your own.
3. Your own hook — only if the collections lack the needed one, or a domain composite is required.

#### Transitions

- Library — **`solid-transition-group`** (`Transition` / `TransitionGroup`).
- Wrappers over `Transition` — a separate component per effect, placed in `src/components/common/transitions/` (one per effect: `TransitionFade`, …).
- Styles — via Tailwind utility classes directly in `enterClass` / `enterActiveClass` / …; CSS modules — only if the needed class is missing (rarely).

### Структура проекта (type-overlay)

Стек: Tauri v2 + SolidJS + TypeScript + Vite + Tailwind v4 + solid-icons, пакетный менеджер — pnpm.

Команды:

- `pnpm tauri dev` — запуск приложения.
- `pnpm build` — typecheck (`tsc --noEmit`) + сборка фронта в `dist/`. **Проверка типов — это `pnpm build`.**
- `cargo check --manifest-path src-tauri/Cargo.toml` — проверка бэкенда.
- `pnpm tauri build` — релизный бандл целиком (фронт + Rust + установщик), артефакты в `src-tauri/target/release/bundle/`.
- `pnpm icon` — регенерация иконок (`scripts/gen-icon.mjs` рисует `app-icon.png`, далее `tauri icon`).

#### Фронтенд (`src/`)

- `index.tsx` — entry: `<Router root={AppLayout}>` с маршрутами `/` (`MainView`) и `/settings` (`SettingsPage`).
- `App.tsx` — `AppLayout`: оборачивает всё в `AppProvider` + `PressedProvider`, рендерит `Shell` (хром окна vs прозрачный оверлей).
- `store/app.tsx` — единый контекст `useApp()`: создаёт layout/mode/settings/platform **ровно один раз**, применяет тему.
- `hooks/` — `useTauriEvent` (генерик-слушатель событий бэка), `usePressed` (fine-grained стор нажатых клавиш через контекст), `useLayout`, `useAppMode`, `useSettings`.
- `lib/` — `keyData` (координаты клавиш, распарсенные из `Keyboard.md`; `BASE_W`/`BASE_H`), `layouts` (RU/EN глифы `{main, shift}`), `tauri` (обёртки над `invoke` + `onEvent`), `settings` (дефолты хоткеев + форматирование комбо), `theme` (системная тема по ОС).
- `components/keyboard/` — `Keyboard` (масштабируемая по контейнеру сетка) + `Key` (одна клавиша, читает pressed/layout).
- `components/common/` — `IconButton`, `LayoutBadge`.
- `components/settings/HotkeyRecorder` — захват сочетания клавиш.
- `components/TopBar.tsx`, `pages/MainView.tsx`, `pages/SettingsPage.tsx`.

#### Бэкенд (`src-tauri/src/`)

- `main.rs` → `lib.rs::run()` — Tauri builder, плагины (`tauri-plugin-store`, `tauri-plugin-global-shortcut`), `AppState`, `setup` (стартует перехват клавиш и поллер раскладки), `invoke_handler`.
- `models.rs` — serde-типы событий/команд (`AppMode`, `HotkeyAction`, `KeyEvent`, `ModeEvent`, `LayoutEvent`).
- `state.rs` — `AppState` (`mode`, `overlay_visible`).
- `commands.rs` — `#[tauri::command]`: `set_app_mode`, `set_overlay_visible`, `register_hotkeys`, `get_layout`, `get_platform`, `check_accessibility`.
- `mode.rs` — трансформация окна окно↔оверлей (`apply_mode`) и `set_overlay_visible`; в overlay на macOS главное окно пинится на все Spaces вкл. fullscreen (`spaces.rs`), на Linux/Wayland — по одному layer-surface на монитор через `layer_shell.rs`, главное окно скрыто (fallback: X11 sticky).
- `spaces.rs` (macOS) — NSWindow collection behavior (`FullScreenAuxiliary`) + уровень status-окна через `objc2`.
- `layer_shell.rs` (Linux) — wlr-layer-shell оверлей-поверхности (Hyprland/Sway); `gtk_layer_init_for_window` должен вызываться до маппинга окна; для сборки нужна системная библиотека `gtk-layer-shell`.
- `actions.rs` — `run_action(action)` — что делает каждый хоткей.
- `hotkeys.rs` — регистрация глобальных шорткатов через `on_shortcut` (по одному на комбо).
- `keyboard.rs` — глобальный перехват: **macOS** — самописный `CGEventTap` в main run-loop (читает только `keyCode`, без TSM-вызовов); **Windows/Linux** — `rdev`.
- `layout.rs` — определение раскладки: macOS (Carbon TIS), Windows (`GetKeyboardLayout`), Linux (`XKB_DEFAULT_LAYOUT`); детект **обязательно в main-потоке** (`run_on_main_thread`).

#### Поток данных

- Бэк → фронт (события): `key-down` / `key-up` (`{ code }`), `layout-changed`, `mode-changed`. Слушаются хуком `useTauriEvent`.
- Фронт → бэк: только через типизированные обёртки `commands.*` в `lib/tauri.ts` (никаких «голых» `invoke` в компонентах).
- Режимы: **одно** окно `main`. В overlay оно становится transparent + always-on-top + click-through + без декораций и масштабируется по экрану; обратно — по глобальному хоткею.

#### Кроссплатформенность

- Весь нативный FFI — за `#[cfg(target_os = "...")]`. `core-foundation` — зависимость только для macOS (`[target.'cfg(target_os = "macos")'.dependencies]`).
- macOS: `macOSPrivateApi: true` в `tauri.conf.json` (нужно для прозрачного окна); перехват клавиш требует **Accessibility** (для отладочного бинаря — пересборка сбрасывает право, добавлять заново).
- Wayland: always-on-top и click-through поддерживаются композитором не везде — оверлей может деградировать; автоопределение раскладки ненадёжно (есть ручной тогл в `LayoutBadge`).
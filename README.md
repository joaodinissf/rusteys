# rusteys

A small always-on-top overlay that shows the keys and shortcuts you press, for screencasts, live demos and presentations.

![rusteys overlay showing a few key presses](docs/screenshot.png)

> TODO: capture `docs/screenshot.png` after the manual smoke test. The image is not in the repository yet.

## Features

- **Keys and shortcuts as you type them.** Combinations are shown together (`Ctrl + S`, `Cmd + Shift + P`), newest on the right. Each key fades out after about 4 seconds.
- **Platform-aware labels.** macOS shows `Control`, `Option`, `Shift` and `Cmd` in the order the macOS menus use. Windows and Linux show `Ctrl`, `Alt`, `Shift` and `Win`. Punctuation and numpad keys get readable labels (`;`, `[`, `Num 7`) instead of internal names.
- **Smart modifiers.** A modifier on its own (a lone `Shift` tap) is shown only when it was released without being part of a combination. Holding both Shift keys and releasing one still counts as Shift held.
- **Repeat collapsing.** Holding or quickly repeating a key shows a single chip with a counter (`J ×5`) instead of filling the bar.
- **Steady layout.** New keys get a short highlight without changing size, so the row doesn't jump.
- **Clear capture errors.** If the keyboard can't be captured (for example, a missing macOS permission), the bar shows a banner that explains what to do instead of staying silently empty.
- **Placed on your real screen.** The bar is centred near the bottom of the primary display. On Windows there is one bar per monitor.
- **Light on CPU.** The overlay only redraws while something is animating, so it stays idle when nothing is on screen.
- **Draggable and always on top**, semi-transparent with rounded corners.

## Quick start

You need a Rust toolchain, version 1.95 or newer (install it from <https://rustup.rs>).

```sh
git clone https://github.com/joaodinissf/rusteys.git
cd rusteys
cargo run --release
```

Or use the launcher script, which builds and runs the release binary and passes any arguments through:

```sh
./run.sh
```

After a release build, you can also run the binary directly: `./target/release/rusteys` (`target\release\rusteys.exe` on Windows).

On macOS, grant the permission described below **before** the first run.

## Permissions

rusteys reads keyboard events system-wide, which every desktop OS guards differently.

### macOS

The app that launches rusteys needs **Input Monitoring**. When you start it from a terminal, that is the terminal app (Terminal, iTerm2, Ghostty, and so on), not rusteys itself.

1. Open **System Settings > Privacy & Security > Input Monitoring**.
2. Turn on your terminal app (add it with **+** if it isn't listed).
3. macOS may also ask for **Accessibility** under the same Privacy & Security section. Allow it if prompted.
4. Quit and reopen the terminal app, then launch rusteys again. The permission only applies to newly started processes.

Without the permission, rusteys still opens, but the bar shows an amber "Keyboard capture unavailable" banner.

### Windows

No special permission is needed normally. Windows does not deliver keys typed into an elevated (administrator) window to a non-elevated process, so to show keys typed into an elevated app, run rusteys elevated as well.

### Linux

Only X11 is supported: rdev captures keys through the X Record extension. Wayland sessions are not supported, because Wayland doesn't let ordinary clients read global keyboard input.

## Usage

- Launch rusteys. The bar appears near the bottom centre of the screen.
- Type. Keys and shortcuts appear on the right and fade out after a few seconds.
- **Move it:** click and drag anywhere on the bar.
- **Focus it:** click the bar. A small badge in the top-right corner reads `Esc to quit · drag to move`, and the layout of the keys doesn't change.
- **Quit:** with the bar focused, press **Esc**. This closes rusteys from any monitor's bar.

## Configuration

There is no settings file yet. Tunables are constants in the source; edit them and rebuild.

- `src/config.rs`: behaviour and geometry, for example `MAX_KEYS` (how many keys stay on screen), `KEY_DISPLAY_DURATION` and `FADE_OUT_DURATION` (how long a key stays and how long it fades), `WINDOW_WIDTH_FRACTION` (bar width as a fraction of the screen width) and `WINDOW_Y_FRACTION` (vertical position as a fraction of the screen height).
- `src/theme.rs`: colours, corner radii, font sizes and spacing.

## Known limitations

- **One bar on macOS and Linux.** Only the primary display gets a bar there. Windows shows one bar per monitor.
- **Win + D on Windows** ("Show Desktop") minimises the overlay along with everything else. This is OS behaviour.
- **No Wayland support on Linux** (see [Permissions](#linux)).
- **rdev is a git dependency.** The crates.io release of rdev (0.5.3, from 2023) pulls in an old `cocoa`/`block` stack that triggers a Rust future-incompatibility warning. rusteys therefore pins rdev to a specific upstream commit (`b7e201bf80be`) until a new version is released.
- Transparency depends on the platform's compositor. On some systems the bar may be drawn on an opaque dark background.

## Development

Every change should pass the same checks:

```sh
cargo build
cargo test
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

The keyboard logic (key labels, modifier tracking, repeat collapsing, fade timing) is plain Rust with unit tests and doesn't need a display. Anything visual has to be checked by running the app.

For a scripted startup check, set `RUSTEYS_EXIT_AFTER_SECS` and rusteys closes itself after that many seconds with exit code 0:

```sh
cargo build --release
RUSTEYS_EXIT_AFTER_SECS=5 ./target/release/rusteys; echo "exit: $?"
```

This still opens a window and, on macOS, may show a permission prompt.

Source layout:

| File | Purpose |
|---|---|
| `src/main.rs` | Entry point: window options and startup |
| `src/config.rs` | Behaviour and geometry tunables |
| `src/theme.rs` | Colours, sizes and the egui style |
| `src/keys.rs` | Key labels per platform |
| `src/input.rs` | Turns raw key events into labels such as `Ctrl + S` |
| `src/capture.rs` | Keyboard capture thread (rdev) |
| `src/history.rs` | Recent keys, repeat collapsing, fade and highlight timing |
| `src/state.rs` | State shared between the capture thread and the UI |
| `src/overlay.rs` | Draws the bar |
| `src/monitors.rs` | Monitor detection |

Built with [egui/eframe](https://github.com/emilk/egui) and [rdev](https://github.com/Narsil/rdev).

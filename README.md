# Rusteys - Real-time Keypress Overlay

A Rust application that displays a screen overlay showing pressed keys and key combinations in real-time.

## Features

- **Smart overlay display** - Only appears when you press keys, fades away when idle
- **Center screen positioning** - Keys display in the center-bottom of your screen
- **Horizontal key stacking** - Keys appear side-by-side as you type
- **Real-time key capture** - Shows keys instantly as you press them
- **Modifier support** - Displays key combinations like `Ctrl + C`, `Alt + Tab`, etc.
- **Smooth animations** - Individual key fade-out effects and scaling
- **Auto-hide** - Overlay fades away 2.5 seconds after the last keypress
- **Non-intrusive** - Transparent background with modern UI

## Building

```bash
cargo build --release
```

## Running

### From the release directory

```bash
./target/release/rusteys.exe
```

### Or use the provided script

```bash
./run.sh
```

## Usage

1. Launch the application
2. The overlay window is **invisible by default**
3. Press any keys - the overlay appears centered at the bottom of your screen
4. Keys stack horizontally next to each other as you type
5. Key combinations with modifiers (Ctrl, Shift, Alt, Win) are shown together
6. Keys fade out after 2 seconds
7. The entire overlay fades away 2.5 seconds after your last keypress
8. Close the window (if visible) or press Ctrl+C in the terminal to exit

## Configuration

You can modify the following constants in `src/main.rs`:

- `MAX_KEYS` - Maximum number of keys to display at once (default: 15)
- `KEY_DISPLAY_DURATION` - How long individual keys remain visible (default: 2000ms)
- `FADE_OUT_DURATION` - Fade animation duration (default: 500ms)
- `WINDOW_HIDE_DELAY` - When the whole overlay starts fading (default: 2500ms)
- Window position in the `main()` function (currently centered at 75% screen height)

## Dependencies

- **eframe/egui** - GUI framework for the overlay
- **rdev** - Cross-platform keyboard event capture
- **parking_lot** - Efficient synchronization primitives

## Notes

- The application requires proper permissions to capture keyboard events system-wide
- On some systems, you may need to run as administrator for global keyboard hooks to work

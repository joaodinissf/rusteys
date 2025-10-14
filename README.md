# Rusteys - Real-time Keypress Overlay

A Rust application that displays a persistent, draggable screen overlay showing pressed keys and key combinations in real-time.

## Features

- **Always-visible overlay** - Constant semi-transparent background at the bottom of your screen
- **Smart key display** - Shows pressed keys and combinations with smooth fade-out animations
- **Right-aligned layout** - Newest keys appear on the right side
- **Modifier key intelligence** - Modifiers (Ctrl, Shift, Alt, Win) only shown standalone if not used in a combination
- **Draggable window** - Click and drag anywhere on the background to reposition
- **Focus indicator** - Blue outline appears when the overlay is focused
- **Always on top** - Stays above other windows (except system-level commands like Win+D)
- **Easy exit** - Press Escape when focused to close the application
- **Transparent design** - 50% opacity background with rounded corners and shadow
- **Smooth animations** - Individual key fade-out effects and scaling on press

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

### Or manually from the release directory

```bash
./target/release/rusteys.exe
```

## Usage

1. Launch the application - a semi-transparent overlay appears at the bottom-center of your screen
2. The overlay is **always visible** with a constant 50% opacity background
3. Press any keys - they appear in the overlay and fade out after 4 seconds
4. Key combinations with modifiers (Ctrl, Shift, Alt, Win) are shown together (e.g., `Ctrl + S`)
5. Modifier keys only appear standalone if pressed and released without being part of a combination
6. **Drag the overlay** - Click and drag anywhere on the background to reposition it
7. **Focus the overlay** - Click on it to see a blue outline indicating it's focused
8. **Exit** - When the overlay is focused, press Escape to close the application

## Configuration

You can modify the following constants in `src/main.rs`:

- `MAX_KEYS` - Maximum number of keys to display at once (default: 15)
- `KEY_DISPLAY_DURATION` - How long individual keys remain visible (default: 4000ms)
- `FADE_OUT_DURATION` - Fade animation duration (default: 800ms)
- `WINDOW_WIDTH_FRACTION` - Window width as fraction of screen width (default: 0.66 = 2/3)
- `SCREEN_WIDTH` / `SCREEN_HEIGHT` - Your screen resolution (default: 1920x1080)
- Window position in the `main()` function (currently centered at 85% screen height)

## Dependencies

- **eframe/egui 0.33** - Modern GUI framework for the overlay
- **rdev 0.5** - Cross-platform keyboard event capture
- **parking_lot 0.12** - Efficient synchronization primitives

## Technical Notes

- The application captures keyboard events system-wide using `rdev`
- The overlay uses egui's immediate mode rendering with per-pixel transparency
- Background opacity is constant to avoid jarring transitions
- Keys are displayed right-to-left (newest on right) to minimize visual movement
- On Windows, the Win+D "Show Desktop" command will minimize the overlay (OS limitation)

## Platform Support

- **Windows** - Fully supported (tested on Windows 10/11)
- **Linux/macOS** - Should work but may require additional permissions for global keyboard capture

## Notes

- The application requires proper permissions to capture keyboard events system-wide
- On some systems, you may need to run as administrator for global keyboard hooks to work
- The overlay respects system transparency capabilities and may have a dark background on some platforms (limitation of the rendering backend)

//! Tunables for the overlay's behaviour and window geometry.
//!
//! Colours, radii and font sizes live in [`crate::theme`].

use std::time::Duration;

/// Maximum number of key presses kept on screen.
pub const MAX_KEYS: usize = 15;
/// How long a key press stays fully visible.
pub const KEY_DISPLAY_DURATION: Duration = Duration::from_millis(4000);
/// How long a key press takes to fade out after [`KEY_DISPLAY_DURATION`].
pub const FADE_OUT_DURATION: Duration = Duration::from_millis(800);

/// How long the "pop" effect on a new key press lasts.
pub const POP_DURATION: Duration = Duration::from_millis(100);
/// Extra scale applied at the very start of the pop (0.2 = 120 %).
pub const POP_EXTRA_SCALE: f32 = 0.2;

/// Overlay width as a fraction of the monitor width (2/3rds of the screen).
pub const WINDOW_WIDTH_FRACTION: f32 = 0.66;
/// Vertical position of the overlay's top edge as a fraction of the monitor height.
pub const WINDOW_Y_FRACTION: f32 = 0.85;
/// Overlay window height in points.
pub const WINDOW_HEIGHT: f32 = 100.0;

/// Assumed screen size when the real monitor geometry is unknown.
pub const SCREEN_WIDTH: f32 = 1920.0;
pub const SCREEN_HEIGHT: f32 = 1080.0;

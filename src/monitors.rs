//! Monitor detection and per-monitor overlay window placement.
//!
//! On Windows the real monitors are enumerated up front and one overlay is
//! placed on each. On macOS and Linux only the primary screen is used: the
//! window starts at a default geometry and is moved once the real screen size
//! is known (see [`placement_for_screen`]). Multiple monitors are not
//! supported there.

use crate::config::{DEFAULT_SCREEN_SIZE, WINDOW_WIDTH_FRACTION, WINDOW_Y_FRACTION};
use crate::theme;
use eframe::egui::{self, Pos2, Vec2};

/// Height of the overlay window in points, derived from the theme: the
/// bar's content area plus its inner margin above and below, so the window is
/// exactly as tall as the bar the overlay draws (see [`theme::BAR_HEIGHT`]).
pub fn window_height() -> f32 {
    theme::BAR_HEIGHT
}

/// Outer position and inner size of an overlay on a screen of the given
/// size whose top-left corner is the origin.
///
/// The bar is [`WINDOW_WIDTH_FRACTION`] of the screen wide, centred
/// horizontally, with its top edge at [`WINDOW_Y_FRACTION`] of the screen
/// height, moved up if needed so it stays fully on screen.
pub fn placement_for_screen(screen: Vec2) -> (Pos2, Vec2) {
    let width = screen.x * WINDOW_WIDTH_FRACTION;
    let height = window_height().min(screen.y);
    let x = (screen.x - width) / 2.0;
    let y = (screen.y * WINDOW_Y_FRACTION)
        .min(screen.y - height)
        .max(0.0);
    (Pos2::new(x, y), Vec2::new(width, height))
}

/// Position and size of one monitor, in points.
#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl MonitorInfo {
    /// A monitor of the assumed default screen size at the origin, used
    /// until the real screen size is known.
    fn fallback() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: DEFAULT_SCREEN_SIZE.0,
            height: DEFAULT_SCREEN_SIZE.1,
        }
    }

    /// Window settings for an overlay centred horizontally near the bottom of this monitor.
    pub fn viewport_builder(&self) -> egui::ViewportBuilder {
        let (position, size) = placement_for_screen(Vec2::new(self.width, self.height));

        egui::ViewportBuilder::default()
            .with_inner_size(size)
            .with_position(position + Vec2::new(self.x, self.y))
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(false)
            .with_mouse_passthrough(false)
    }
}

/// Lists the monitors to show an overlay on; never empty.
///
/// On Windows this enumerates the real monitors. Elsewhere it returns a
/// single monitor of the default screen size; the app moves the window to
/// the real screen on its first frame.
pub fn detect() -> Vec<MonitorInfo> {
    #[cfg(target_os = "windows")]
    let mut monitors = detect_windows();

    #[cfg(not(target_os = "windows"))]
    let mut monitors = vec![MonitorInfo::fallback()];

    if monitors.is_empty() {
        println!("No monitors detected, using default configuration");
        monitors.push(MonitorInfo::fallback());
    }

    monitors
}

#[cfg(target_os = "windows")]
fn detect_windows() -> Vec<MonitorInfo> {
    use std::ptr;
    use winapi::shared::windef::{HDC, HMONITOR, LPRECT};
    use winapi::um::winuser::EnumDisplayMonitors;

    let mut monitors = Vec::new();

    unsafe extern "system" fn monitor_enum_proc(
        _hmonitor: HMONITOR,
        _hdc: HDC,
        lprect: LPRECT,
        lparam: isize,
    ) -> i32 {
        unsafe {
            let monitors = &mut *(lparam as *mut Vec<MonitorInfo>);
            let rect = &*lprect;

            monitors.push(MonitorInfo {
                x: rect.left as f32,
                y: rect.top as f32,
                width: (rect.right - rect.left) as f32,
                height: (rect.bottom - rect.top) as f32,
            });

            1 // Continue enumeration
        }
    }

    unsafe {
        EnumDisplayMonitors(
            ptr::null_mut(),
            ptr::null(),
            Some(monitor_enum_proc),
            &mut monitors as *mut _ as isize,
        );
    }

    monitors
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_height_fits_one_chip_row() {
        let height = window_height();
        let chip_row = theme::CHIP_FONT_SIZE + 2.0 * f32::from(theme::CHIP_MARGIN.1);
        assert!(height >= chip_row + 2.0 * f32::from(theme::PANEL_MARGIN.1));
        assert!(height <= 100.0, "height = {height}");
    }

    #[test]
    fn placement_is_centred_near_the_bottom() {
        let screen = Vec2::new(1512.0, 982.0);
        let (pos, size) = placement_for_screen(screen);
        assert!((size.x - 1512.0 * WINDOW_WIDTH_FRACTION).abs() < 1e-3);
        assert!((pos.x + size.x / 2.0 - screen.x / 2.0).abs() < 1e-3);
        assert!((pos.y - 982.0 * WINDOW_Y_FRACTION).abs() < 1e-3);
        assert!(pos.y + size.y <= screen.y);
    }

    #[test]
    fn placement_is_clamped_on_short_screens() {
        let screen = Vec2::new(800.0, 400.0);
        let (pos, size) = placement_for_screen(screen);
        assert!((pos.y + size.y - screen.y).abs() < 1e-3);
        assert!(pos.y >= 0.0);
    }

    #[test]
    fn placement_never_goes_above_the_screen() {
        let (pos, size) = placement_for_screen(Vec2::new(300.0, 50.0));
        assert_eq!(pos.y, 0.0);
        assert_eq!(size.y, 50.0);
    }
}

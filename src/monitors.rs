//! Monitor detection and per-monitor overlay window placement.

use crate::config::{
    SCREEN_HEIGHT, SCREEN_WIDTH, WINDOW_HEIGHT, WINDOW_WIDTH_FRACTION, WINDOW_Y_FRACTION,
};
use eframe::egui;

/// Position and size of one monitor, in points.
#[derive(Debug, Clone)]
pub struct MonitorInfo {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl MonitorInfo {
    /// A monitor of the assumed default screen size at the origin.
    fn fallback() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            width: SCREEN_WIDTH,
            height: SCREEN_HEIGHT,
        }
    }

    /// Window settings for an overlay centred horizontally near the bottom of this monitor.
    pub fn viewport_builder(&self) -> egui::ViewportBuilder {
        let window_width = self.width * WINDOW_WIDTH_FRACTION;
        let window_x = self.x + (self.width - window_width) / 2.0;
        let window_y = self.y + self.height * WINDOW_Y_FRACTION;

        egui::ViewportBuilder::default()
            .with_inner_size([window_width, WINDOW_HEIGHT])
            .with_position([window_x, window_y])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(false)
            .with_mouse_passthrough(false)
    }
}

/// Lists the monitors to show an overlay on; never empty.
///
/// On Windows this enumerates the real monitors. Elsewhere it assumes a
/// single monitor of the default screen size.
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

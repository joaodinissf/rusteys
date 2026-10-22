//! Colours, sizes and the global egui style of the overlay.

use eframe::egui::{self, Color32};

/// Background of the overlay bar (constant 50 % opacity).
pub const PANEL_FILL: Color32 = Color32::from_rgba_unmultiplied_const(35, 35, 35, 127);
/// Inner margin of the overlay bar.
pub const PANEL_MARGIN: i8 = 20;
/// Corner radius of the overlay bar and of its focus outline.
pub const PANEL_RADIUS: u8 = 12;
/// Drop shadow under the overlay bar.
pub const PANEL_SHADOW: egui::epaint::Shadow = egui::epaint::Shadow {
    offset: [0, 4],
    blur: 16,
    spread: 0,
    color: Color32::from_rgba_unmultiplied_const(0, 0, 0, 115),
};
/// Minimum height of the bar's content, so it doesn't collapse when empty.
pub const PANEL_MIN_CONTENT_HEIGHT: f32 = 60.0;

/// Outline drawn around the bar while the window has focus.
pub const FOCUS_OUTLINE_COLOR: Color32 = Color32::from_rgb(100, 150, 255);
pub const FOCUS_OUTLINE_WIDTH: f32 = 3.0;
/// How far the focus outline is inset so it stays visible.
pub const FOCUS_OUTLINE_INSET: f32 = 1.5;

/// The "press Esc to exit" hint shown while the window has focus.
pub const FOCUS_LABEL_COLOR: Color32 = Color32::from_rgb(180, 200, 255);
pub const FOCUS_LABEL_SIZE: f32 = 14.0;
pub const FOCUS_LABEL_SPACING: f32 = 8.0;

/// Horizontal gap between key chips.
pub const CHIP_SPACING: f32 = 12.0;
/// Font size of a key chip's label (before the pop scale).
pub const CHIP_FONT_SIZE: f32 = 28.0;
pub const CHIP_RADIUS: u8 = 6;
pub const CHIP_BORDER_WIDTH: f32 = 1.5;
/// Inner margin of a key chip: horizontal, vertical.
pub const CHIP_MARGIN: (i8, i8) = (12, 8);

/// RGB colours of a key chip; the alpha comes from the fade-out.
pub struct ChipPalette {
    pub fill: [u8; 3],
    pub border: [u8; 3],
    pub text: [u8; 3],
}

impl ChipPalette {
    /// Returns (fill, border, text) with the given alpha applied.
    pub fn colors(&self, alpha: u8) -> (Color32, Color32, Color32) {
        let with_alpha = |[r, g, b]: [u8; 3]| Color32::from_rgba_unmultiplied(r, g, b, alpha);
        (
            with_alpha(self.fill),
            with_alpha(self.border),
            with_alpha(self.text),
        )
    }
}

/// The most recent key: blue accent.
pub const RECENT_CHIP: ChipPalette = ChipPalette {
    fill: [70, 110, 200],
    border: [100, 140, 220],
    text: [255, 255, 255],
};

/// Older keys: grey.
pub const OLDER_CHIP: ChipPalette = ChipPalette {
    fill: [70, 75, 85],
    border: [140, 150, 170],
    text: [255, 255, 255],
};

/// Makes the default egui backgrounds transparent so only the bar is drawn.
pub fn apply_transparent_style(ctx: &egui::Context) {
    ctx.all_styles_mut(|style| {
        style.visuals.window_fill = Color32::TRANSPARENT;
        style.visuals.panel_fill = Color32::TRANSPARENT;
        style.visuals.window_stroke = egui::Stroke::NONE;
        style.visuals.extreme_bg_color = Color32::TRANSPARENT;
        style.visuals.faint_bg_color = Color32::TRANSPARENT;
    });
}

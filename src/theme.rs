//! Colours, sizes and the global egui style of the overlay.
//!
//! Every visual tunable of the bar lives here so the look can be adjusted in
//! one place.

use eframe::egui::{self, Color32};

// --- Bar ---------------------------------------------------------------------

/// Background of the overlay bar: dark and fairly opaque, so the chips stay
/// legible on light backgrounds too.
pub const PANEL_FILL: Color32 = Color32::from_rgba_unmultiplied_const(24, 24, 28, 184);
/// Inner margin of the overlay bar, horizontal and vertical.
///
/// The vertical margin also leaves room for the focus badge in the top-right
/// corner, so the badge never overlaps the chips.
pub const PANEL_MARGIN: (i8, i8) = (16, 16);
/// Corner radius of the overlay bar and of its focus outline.
pub const PANEL_RADIUS: u8 = 12;
/// Drop shadow under the overlay bar.
pub const PANEL_SHADOW: egui::epaint::Shadow = egui::epaint::Shadow {
    offset: [0, 4],
    blur: 16,
    spread: 0,
    color: Color32::from_rgba_unmultiplied_const(0, 0, 0, 115),
};

/// Height of the bar's content area: one row of chips plus a little slack.
pub const CONTENT_HEIGHT: f32 = 56.0;
/// Overall bar (window) height in points: content plus the vertical margins.
///
/// The window should be exactly this tall so nothing is clipped or stretched.
// Meant for the window setup, which still uses `config::WINDOW_HEIGHT`.
#[allow(dead_code)]
pub const BAR_HEIGHT: f32 = CONTENT_HEIGHT + 2.0 * PANEL_MARGIN.1 as f32;

// --- Focus -------------------------------------------------------------------

/// Outline drawn around the bar while the window has focus.
pub const FOCUS_OUTLINE_COLOR: Color32 = Color32::from_rgb(100, 150, 255);
pub const FOCUS_OUTLINE_WIDTH: f32 = 2.0;
/// How far the focus outline is inset so it stays visible.
pub const FOCUS_OUTLINE_INSET: f32 = 1.0;

/// Text of the badge painted in the top-right corner while focused.
pub const FOCUS_BADGE_TEXT: &str = "Esc to quit · drag to move";
pub const FOCUS_BADGE_SIZE: f32 = 11.0;
pub const FOCUS_BADGE_COLOR: Color32 = Color32::from_rgb(180, 200, 255);
/// Offset of the badge's top-right corner from the bar's top-right corner.
pub const FOCUS_BADGE_OFFSET: (f32, f32) = (14.0, 3.0);

// --- Empty state and errors --------------------------------------------------

/// Hint shown in an empty bar shortly after launch.
pub const EMPTY_HINT_TEXT: &str = "Waiting for keys…";
pub const EMPTY_HINT_SIZE: f32 = 16.0;
pub const EMPTY_HINT_COLOR: Color32 = Color32::from_rgba_unmultiplied_const(255, 255, 255, 90);

/// Banner shown when the keyboard capture failed.
pub const BANNER_FILL: Color32 = Color32::from_rgb(120, 78, 10);
pub const BANNER_BORDER: Color32 = Color32::from_rgb(245, 166, 35);
pub const BANNER_TEXT: Color32 = Color32::from_rgb(255, 236, 200);
pub const BANNER_TEXT_SIZE: f32 = 14.0;
pub const BANNER_RADIUS: u8 = 8;
/// Inner margin of the banner: horizontal, vertical.
pub const BANNER_MARGIN: (i8, i8) = (12, 6);

// --- Chips -------------------------------------------------------------------

/// Horizontal gap between key chips.
pub const CHIP_SPACING: f32 = 10.0;
/// Font size of a key chip's label. Fixed, so chips never change width.
pub const CHIP_FONT_SIZE: f32 = 26.0;
pub const CHIP_RADIUS: u8 = 8;
/// Border width of a chip. Fixed: in egui the stroke is part of the frame's
/// size, so animating it would move the row.
pub const CHIP_BORDER_WIDTH: f32 = 1.0;
/// Inner margin of a key chip: horizontal, vertical.
pub const CHIP_MARGIN: (i8, i8) = (12, 6);

/// How much the newest chip lights up towards white at the start of its pop
/// (0.0 = no change, 1.0 = white). Applied to the fill and the border; it
/// fades back to the normal colours over the pop duration.
pub const POP_GLOW_FILL: f32 = 0.30;
pub const POP_GLOW_BORDER: f32 = 0.85;

/// RGB colours of a key chip; the alpha comes from the fade-out.
pub struct ChipPalette {
    pub fill: [u8; 3],
    pub border: [u8; 3],
    pub text: [u8; 3],
}

impl ChipPalette {
    /// Returns (fill, border, text) with the given alpha applied and the fill
    /// and border brightened by `glow` (0.0 to 1.0, see [`POP_GLOW_FILL`]).
    pub fn colors(&self, alpha: u8, glow: f32) -> (Color32, Color32, Color32) {
        let with_alpha = |[r, g, b]: [u8; 3]| Color32::from_rgba_unmultiplied(r, g, b, alpha);
        (
            with_alpha(brighten(self.fill, glow * POP_GLOW_FILL)),
            with_alpha(brighten(self.border, glow * POP_GLOW_BORDER)),
            with_alpha(self.text),
        )
    }
}

/// Mixes `rgb` towards white by `amount` (clamped to 0.0..=1.0).
pub fn brighten(rgb: [u8; 3], amount: f32) -> [u8; 3] {
    let amount = amount.clamp(0.0, 1.0);
    rgb.map(|c| {
        let c = f32::from(c);
        (c + (255.0 - c) * amount).round() as u8
    })
}

/// The most recent key: blue accent.
pub const RECENT_CHIP: ChipPalette = ChipPalette {
    fill: [76, 125, 255],
    border: [130, 165, 255],
    text: [255, 255, 255],
};

/// Older keys: neutral grey.
pub const OLDER_CHIP: ChipPalette = ChipPalette {
    fill: [58, 60, 68],
    border: [110, 114, 128],
    text: [240, 240, 245],
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn brighten_endpoints() {
        assert_eq!(brighten([10, 20, 30], 0.0), [10, 20, 30]);
        assert_eq!(brighten([10, 20, 30], 1.0), [255, 255, 255]);
        assert_eq!(brighten([0, 0, 0], 0.5), [128, 128, 128]);
    }

    #[test]
    fn brighten_clamps_amount() {
        assert_eq!(brighten([10, 20, 30], -1.0), [10, 20, 30]);
        assert_eq!(brighten([10, 20, 30], 2.0), [255, 255, 255]);
    }

    #[test]
    fn chip_colors_apply_alpha_and_keep_text() {
        let (fill, border, text) = RECENT_CHIP.colors(255, 0.0);
        assert_eq!(fill, Color32::from_rgb(76, 125, 255));
        assert_eq!(border, Color32::from_rgb(130, 165, 255));
        assert_eq!(text, Color32::WHITE);
        let (_, _, faded_text) = RECENT_CHIP.colors(0, 1.0);
        assert_eq!(faded_text.a(), 0);
    }

    #[test]
    fn bar_height_fits_content_and_margins() {
        assert_eq!(BAR_HEIGHT, 88.0);
        // A chip (font row + margins + border) must fit the content area.
        let chip_height =
            CHIP_FONT_SIZE * 1.4 + 2.0 * f32::from(CHIP_MARGIN.1) + 2.0 * CHIP_BORDER_WIDTH;
        assert!(chip_height <= CONTENT_HEIGHT, "{chip_height}");
    }
}

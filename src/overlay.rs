//! Drawing of the overlay bar and its key chips.

use crate::history::{fade_alpha, pop_scale};
use crate::state::SharedState;
use crate::theme;
use eframe::egui;
use std::time::Instant;

/// Draws one frame of the overlay into `ui` (the root or a per-monitor viewport).
pub fn render(ui: &mut egui::Ui, state: &SharedState, now: Instant) {
    let ctx = ui.ctx().clone();
    // Check if window is focused and Escape is pressed
    if ctx.input(|i| i.focused && i.key_pressed(egui::Key::Escape)) {
        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
    }

    // Clean up old key presses
    state.lock().history.prune(now);

    let history = state.lock().history.clone();

    // Request repaint for smooth animations
    ctx.request_repaint();

    // Check if window is focused for visual indication
    let is_focused = ctx.input(|i| i.focused);

    egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(theme::PANEL_FILL)
                .inner_margin(egui::Margin::same(theme::PANEL_MARGIN))
                .corner_radius(egui::CornerRadius::same(theme::PANEL_RADIUS))
                .shadow(theme::PANEL_SHADOW),
        )
        .show(ui, |ui| {
            // Draw focus indicator outline on top of everything
            if is_focused {
                let rect = ui.max_rect().shrink(theme::FOCUS_OUTLINE_INSET);
                ui.painter().rect_stroke(
                    rect,
                    egui::CornerRadius::same(theme::PANEL_RADIUS),
                    egui::Stroke::new(theme::FOCUS_OUTLINE_WIDTH, theme::FOCUS_OUTLINE_COLOR),
                    egui::StrokeKind::Outside,
                );
            }

            // Make the window draggable by detecting drag on the background
            let response = ui.interact(
                ui.max_rect(),
                ui.scope_id().with("drag_overlay"),
                egui::Sense::drag(),
            );
            if response.dragged() {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }

            // Always allocate minimum height to prevent layout shift when empty
            ui.set_min_height(theme::PANEL_MIN_CONTENT_HEIGHT);

            // Show focus message when window is focused (centered)
            if is_focused {
                ui.vertical_centered(|ui| {
                    ui.add_space(theme::FOCUS_LABEL_SPACING);
                    ui.label(
                        egui::RichText::new("Window in focus; press Esc to exit")
                            .size(theme::FOCUS_LABEL_SIZE)
                            .color(theme::FOCUS_LABEL_COLOR),
                    );
                    ui.add_space(theme::FOCUS_LABEL_SPACING);
                });
            }

            // Use a scroll area that auto-scrolls to the right (most recent keys)
            egui::ScrollArea::horizontal()
                .auto_shrink(false)
                .stick_to_right(true)
                .show(ui, |ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.spacing_mut().item_spacing.x = theme::CHIP_SPACING;

                        // Newest first: the first chip laid out right-to-left is the most recent.
                        for (index, key_press) in history.iter().rev().enumerate() {
                            let age = now.duration_since(key_press.timestamp);
                            let palette = if index == 0 {
                                &theme::RECENT_CHIP
                            } else {
                                &theme::OLDER_CHIP
                            };
                            let alpha = (fade_alpha(age) * 255.0) as u8;
                            let (fill, border, text_color) = palette.colors(alpha);
                            let font_size = theme::CHIP_FONT_SIZE * pop_scale(age);

                            let (margin_x, margin_y) = theme::CHIP_MARGIN;
                            egui::Frame::new()
                                .fill(fill)
                                .corner_radius(egui::CornerRadius::same(theme::CHIP_RADIUS))
                                .stroke(egui::Stroke::new(theme::CHIP_BORDER_WIDTH, border))
                                .inner_margin(egui::Margin::symmetric(margin_x, margin_y))
                                .show(ui, |ui| {
                                    let text = egui::RichText::new(&key_press.text)
                                        .size(font_size)
                                        .strong()
                                        .color(text_color);
                                    ui.add(
                                        egui::Label::new(text)
                                            .wrap_mode(egui::TextWrapMode::Extend),
                                    );
                                });
                        }
                    });
                });
        });
}

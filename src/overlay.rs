//! Drawing of the overlay bar and its key chips.
//!
//! The layout is the same whether or not the window has focus: the focus
//! badge is painted over the bar's top margin instead of taking up space, and
//! chips have a fixed size (the "pop" on a new key is a colour glow, not a
//! size change), so the row never jumps.

use crate::config::{FADE_OUT_DURATION, KEY_DISPLAY_DURATION, POP_DURATION, POP_EXTRA_SCALE};
use crate::history::{fade_alpha, pop_scale};
use crate::state::{CaptureStatus, SharedState};
use crate::theme;
use eframe::egui;
use std::time::{Duration, Instant};

/// How long the "Waiting for keys…" hint is shown after launch.
const EMPTY_HINT_DURATION: Duration = Duration::from_secs(5);
/// Repaint interval while something is animating (about 60 fps).
const ANIMATION_FRAME: Duration = Duration::from_millis(16);

/// What one chip needs to be drawn, copied out of the shared state.
struct Chip {
    text: String,
    age: Duration,
}

/// Draws one frame of the overlay into `ui` (the root or a per-monitor viewport).
pub fn render(ui: &mut egui::Ui, state: &SharedState, now: Instant) {
    let ctx = ui.ctx().clone();
    let is_focused = ctx.input(|i| i.focused);

    // Esc quits the whole app, whichever monitor's bar has focus. Closing only
    // this viewport would do nothing on a secondary monitor, whose immediate
    // viewport is recreated on the next frame.
    if is_focused && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
        ctx.send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::Close);
    }

    // Lock once: prune, then copy only what is drawn (newest first).
    let (chips, capture, next_deadline) = {
        let mut state = state.lock();
        state.history.prune(now);
        let chips: Vec<Chip> = state
            .history
            .iter()
            .rev()
            .map(|kp| Chip {
                text: kp.display().into_owned(),
                age: now.saturating_duration_since(kp.timestamp),
            })
            .collect();
        let next_deadline = state.history.next_deadline(now);
        (chips, state.capture.clone(), next_deadline)
    };

    let since_launch = now.saturating_duration_since(launch_time(&ctx, now));
    let show_empty_hint = chips.is_empty()
        && !matches!(capture, CaptureStatus::Failed(_))
        && since_launch < EMPTY_HINT_DURATION;

    // Repaint only when something will change on screen. Key events wake the
    // UI through the context held by the capture thread.
    let hint_deadline = show_empty_hint.then(|| EMPTY_HINT_DURATION - since_launch);
    if let Some(delay) = earliest(
        repaint_delay(chips.iter().map(|chip| chip.age), next_deadline),
        hint_deadline,
    ) {
        ctx.request_repaint_after(delay);
    }

    let (margin_x, margin_y) = theme::PANEL_MARGIN;
    egui::CentralPanel::default()
        .frame(
            egui::Frame::new()
                .fill(theme::PANEL_FILL)
                .inner_margin(egui::Margin::symmetric(margin_x, margin_y))
                .corner_radius(egui::CornerRadius::same(theme::PANEL_RADIUS))
                .shadow(theme::PANEL_SHADOW),
        )
        .show(ui, |ui| {
            let bar_rect = ui
                .max_rect()
                .expand2(egui::vec2(f32::from(margin_x), f32::from(margin_y)));

            // Make the window draggable by detecting drag on the background.
            let response = ui.interact(
                bar_rect,
                ui.scope_id().with("drag_overlay"),
                egui::Sense::drag(),
            );
            if response.dragged() {
                ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
            }

            ui.set_min_height(theme::CONTENT_HEIGHT);

            if let CaptureStatus::Failed(message) = &capture {
                draw_capture_banner(ui, message);
            } else if show_empty_hint {
                draw_empty_hint(ui);
            } else {
                draw_chips(ui, &chips);
            }

            // Painted last and outside the layout, so focus changes nothing else.
            if is_focused {
                draw_focus_decorations(ui, bar_rect);
            }
        });
}

/// The time of the first frame, remembered in the egui context.
fn launch_time(ctx: &egui::Context, now: Instant) -> Instant {
    ctx.data_mut(|data| {
        *data.get_temp_mut_or_insert_with(egui::Id::unique("rusteys_launch"), || now)
    })
}

/// The row of key chips, newest on the right.
fn draw_chips(ui: &mut egui::Ui, chips: &[Chip]) {
    egui::ScrollArea::horizontal()
        .auto_shrink(false)
        .stick_to_right(true)
        .show(ui, |ui| {
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = theme::CHIP_SPACING;

                // Newest first: the first chip laid out right-to-left is the most recent.
                for (index, chip) in chips.iter().enumerate() {
                    let newest = index == 0;
                    let palette = if newest {
                        &theme::RECENT_CHIP
                    } else {
                        &theme::OLDER_CHIP
                    };
                    let alpha = (fade_alpha(chip.age) * 255.0).round() as u8;
                    let glow = if newest { pop_glow(chip.age) } else { 0.0 };
                    let (fill, border, text_color) = palette.colors(alpha, glow);

                    let (margin_x, margin_y) = theme::CHIP_MARGIN;
                    egui::Frame::new()
                        .fill(fill)
                        .corner_radius(egui::CornerRadius::same(theme::CHIP_RADIUS))
                        .stroke(egui::Stroke::new(theme::CHIP_BORDER_WIDTH, border))
                        .inner_margin(egui::Margin::symmetric(margin_x, margin_y))
                        .show(ui, |ui| {
                            let text = egui::RichText::new(&chip.text)
                                .size(theme::CHIP_FONT_SIZE)
                                .strong()
                                .color(text_color);
                            ui.add(egui::Label::new(text).wrap_mode(egui::TextWrapMode::Extend));
                        });
                }
            });
        });
}

/// Faint centred hint for an empty bar right after launch.
fn draw_empty_hint(ui: &mut egui::Ui) {
    ui.centered_and_justified(|ui| {
        ui.label(
            egui::RichText::new(theme::EMPTY_HINT_TEXT)
                .size(theme::EMPTY_HINT_SIZE)
                .color(theme::EMPTY_HINT_COLOR),
        );
    });
}

/// Amber banner explaining that no keys can be captured.
fn draw_capture_banner(ui: &mut egui::Ui, message: &str) {
    let text = capture_failure_text(message, launcher_app_name().as_deref());
    let (margin_x, margin_y) = theme::BANNER_MARGIN;
    ui.with_layout(
        egui::Layout::top_down(egui::Align::Center).with_main_align(egui::Align::Center),
        |ui| {
            egui::Frame::new()
                .fill(theme::BANNER_FILL)
                .stroke(egui::Stroke::new(1.0, theme::BANNER_BORDER))
                .corner_radius(egui::CornerRadius::same(theme::BANNER_RADIUS))
                .inner_margin(egui::Margin::symmetric(margin_x, margin_y))
                .show(ui, |ui| {
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new(text)
                                .size(theme::BANNER_TEXT_SIZE)
                                .color(theme::BANNER_TEXT),
                        )
                        .wrap_mode(egui::TextWrapMode::Wrap),
                    );
                });
        },
    );
}

/// Focus outline around the bar and the hint badge in its top-right corner.
fn draw_focus_decorations(ui: &egui::Ui, bar_rect: egui::Rect) {
    let painter = ui.painter();
    painter.rect_stroke(
        bar_rect.shrink(theme::FOCUS_OUTLINE_INSET),
        egui::CornerRadius::same(theme::PANEL_RADIUS),
        egui::Stroke::new(theme::FOCUS_OUTLINE_WIDTH, theme::FOCUS_OUTLINE_COLOR),
        egui::StrokeKind::Inside,
    );
    let (dx, dy) = theme::FOCUS_BADGE_OFFSET;
    painter.text(
        bar_rect.right_top() + egui::vec2(-dx, dy),
        egui::Align2::RIGHT_TOP,
        theme::FOCUS_BADGE_TEXT,
        egui::FontId::proportional(theme::FOCUS_BADGE_SIZE),
        theme::FOCUS_BADGE_COLOR,
    );
}

/// Glow strength (1.0 at the keypress, 0.0 once the pop is over).
fn pop_glow(age: Duration) -> f32 {
    ((pop_scale(age) - 1.0) / POP_EXTRA_SCALE).clamp(0.0, 1.0)
}

/// The text of the capture-failure banner.
///
/// On macOS the usual cause is a missing Input Monitoring permission, so the
/// banner says how to grant it; elsewhere it shows the listener's error.
fn capture_failure_text(message: &str, app: Option<&str>) -> String {
    if cfg!(target_os = "macos") {
        let app = app.unwrap_or("the app you launched rusteys from");
        format!(
            "Keyboard capture unavailable: grant Input Monitoring to {app} \
             in System Settings > Privacy & Security, then relaunch"
        )
    } else {
        format!("Keyboard capture unavailable: {message}")
    }
}

/// A readable name for the terminal that launched us, from `TERM_PROGRAM`.
fn launcher_app_name() -> Option<String> {
    std::env::var("TERM_PROGRAM")
        .ok()
        .map(|program| terminal_display_name(&program))
}

fn terminal_display_name(term_program: &str) -> String {
    match term_program {
        "Apple_Terminal" => "Terminal".to_owned(),
        "iTerm.app" => "iTerm".to_owned(),
        "vscode" => "Visual Studio Code".to_owned(),
        other => other.to_owned(),
    }
}

/// How long until the chips with the given ages next need a repaint.
///
/// While a chip pops or fades the bar animates every frame; otherwise it
/// sleeps until `next_deadline`, the history's next pop/fade boundary (see
/// [`crate::history::KeyHistory::next_deadline`]). `None` means nothing will
/// change.
fn repaint_delay(
    ages: impl IntoIterator<Item = Duration>,
    next_deadline: Option<Duration>,
) -> Option<Duration> {
    let fade_end = KEY_DISPLAY_DURATION + FADE_OUT_DURATION;
    let animating = ages
        .into_iter()
        .any(|age| age < POP_DURATION || (KEY_DISPLAY_DURATION..fade_end).contains(&age));
    if animating {
        Some(ANIMATION_FRAME)
    } else {
        next_deadline
    }
}

fn earliest(a: Option<Duration>, b: Option<Duration>) -> Option<Duration> {
    match (a, b) {
        (Some(a), Some(b)) => Some(a.min(b)),
        (a, b) => a.or(b),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_chips_needs_no_repaint() {
        assert_eq!(repaint_delay([], None), None);
    }

    #[test]
    fn popping_or_fading_chip_animates() {
        let later = Some(Duration::from_secs(3));
        assert_eq!(
            repaint_delay([Duration::ZERO], later),
            Some(ANIMATION_FRAME)
        );
        assert_eq!(
            repaint_delay([KEY_DISPLAY_DURATION + Duration::from_millis(1)], later),
            Some(ANIMATION_FRAME)
        );
    }

    #[test]
    fn steady_chips_sleep_until_the_next_deadline() {
        let deadline = Some(Duration::from_secs(1));
        assert_eq!(repaint_delay([Duration::from_secs(1)], deadline), deadline);
        assert_eq!(
            repaint_delay([Duration::from_secs(1), Duration::from_secs(3)], deadline),
            deadline
        );
    }

    #[test]
    fn steady_chip_with_real_history_sleeps_until_fade_starts() {
        let start = Instant::now();
        let mut history = crate::history::KeyHistory::default();
        history.push("A".to_owned(), start);
        let now = start + Duration::from_secs(1);
        assert_eq!(
            repaint_delay([Duration::from_secs(1)], history.next_deadline(now)),
            Some(KEY_DISPLAY_DURATION - Duration::from_secs(1))
        );
    }

    #[test]
    fn faded_chip_needs_no_repaint() {
        assert_eq!(
            repaint_delay([KEY_DISPLAY_DURATION + FADE_OUT_DURATION], None),
            None
        );
    }

    #[test]
    fn earliest_picks_the_smaller() {
        let s = Duration::from_secs;
        assert_eq!(earliest(Some(s(2)), Some(s(1))), Some(s(1)));
        assert_eq!(earliest(None, Some(s(1))), Some(s(1)));
        assert_eq!(earliest(Some(s(2)), None), Some(s(2)));
        assert_eq!(earliest(None, None), None);
    }

    #[test]
    fn pop_glow_fades_out() {
        assert!((pop_glow(Duration::ZERO) - 1.0).abs() < 1e-6);
        assert_eq!(pop_glow(POP_DURATION), 0.0);
    }

    #[test]
    fn capture_failure_text_by_platform() {
        let text = capture_failure_text("EventTapError", Some("Terminal"));
        if cfg!(target_os = "macos") {
            assert!(
                text.contains("grant Input Monitoring to Terminal"),
                "{text}"
            );
            assert!(text.contains("System Settings > Privacy & Security"));
        } else {
            assert_eq!(text, "Keyboard capture unavailable: EventTapError");
        }
    }

    #[test]
    fn terminal_names_are_readable() {
        assert_eq!(terminal_display_name("Apple_Terminal"), "Terminal");
        assert_eq!(terminal_display_name("iTerm.app"), "iTerm");
        assert_eq!(terminal_display_name("ghostty"), "ghostty");
    }
}

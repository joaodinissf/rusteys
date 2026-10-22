mod capture;
mod config;
mod history;
mod input;
mod keys;
mod monitors;
mod overlay;
mod state;
mod theme;

use eframe::egui;
use monitors::MonitorInfo;
use state::SharedState;
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Environment variable that makes the app quit by itself after this many
/// seconds, so a scripted run can check startup without anyone pressing Esc.
const EXIT_AFTER_ENV: &str = "RUSTEYS_EXIT_AFTER_SECS";

/// Whether the root window still has to be moved to the real screen once its
/// size is known. Windows places every overlay from `EnumDisplayMonitors`.
const PLACE_ON_FIRST_FRAME: bool = cfg!(not(target_os = "windows"));

struct KeyDisplayApp {
    state: SharedState,
    monitors: Vec<MonitorInfo>,
    needs_placement: bool,
    started: Instant,
    exit_after: Option<Duration>,
}

impl KeyDisplayApp {
    /// Moves the root window to the real screen once its size is reported.
    fn place_on_screen(&mut self, ctx: &egui::Context) {
        let Some(screen) = ctx.input(|i| i.viewport().monitor_size) else {
            return;
        };
        let (position, size) = monitors::placement_for_screen(screen);
        ctx.send_viewport_cmd(egui::ViewportCommand::InnerSize(size));
        ctx.send_viewport_cmd(egui::ViewportCommand::OuterPosition(position));
        self.needs_placement = false;
    }

    /// Closes the app once `exit_after` has elapsed, if it is set.
    fn check_exit_timer(&self, ctx: &egui::Context, now: Instant) {
        let Some(exit_after) = self.exit_after else {
            return;
        };
        let elapsed = now.duration_since(self.started);
        if elapsed >= exit_after {
            ctx.send_viewport_cmd_to(egui::ViewportId::ROOT, egui::ViewportCommand::Close);
        } else {
            ctx.request_repaint_after(exit_after - elapsed);
        }
    }
}

impl eframe::App for KeyDisplayApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();
        let ctx = ui.ctx().clone();

        if self.needs_placement {
            self.place_on_screen(&ctx);
        }
        self.check_exit_timer(&ctx, now);

        // Create secondary viewports for additional monitors (every frame)
        for (i, monitor) in self.monitors.iter().enumerate().skip(1) {
            let state = Arc::clone(&self.state);
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of(format!("monitor_{i}")),
                monitor
                    .viewport_builder()
                    .with_title(format!("Key Display Overlay - Monitor {}", i + 1)),
                move |ui, _class| overlay::render(ui, &state, now),
            );
        }

        // Render the main window (first monitor)
        overlay::render(ui, &self.state, now);
    }
}

/// Parses the value of [`EXIT_AFTER_ENV`]: a non-negative number of seconds.
fn parse_exit_after(value: &str) -> Option<Duration> {
    let secs: f64 = value.trim().parse().ok()?;
    Duration::try_from_secs_f64(secs).ok()
}

/// Reads [`EXIT_AFTER_ENV`], warning about values that can't be used.
fn exit_after_from_env() -> Option<Duration> {
    let value = std::env::var(EXIT_AFTER_ENV).ok()?;
    let parsed = parse_exit_after(&value);
    if parsed.is_none() {
        eprintln!("Ignoring {EXIT_AFTER_ENV}={value:?}: expected a number of seconds");
    }
    parsed
}

fn main() -> Result<(), eframe::Error> {
    let state = SharedState::default();
    let monitors = monitors::detect();
    let exit_after = exit_after_from_env();
    println!(
        "Detected {} monitor(s), creating overlay on each...",
        monitors.len()
    );

    let options = eframe::NativeOptions {
        viewport: monitors[0].viewport_builder(),
        ..Default::default()
    };

    eframe::run_native(
        "Key Display Overlay - Monitor 1",
        options,
        Box::new(move |cc| {
            theme::apply_transparent_style(&cc.egui_ctx);
            capture::spawn_capture(Arc::clone(&state), cc.egui_ctx.clone());
            Ok(Box::new(KeyDisplayApp {
                state,
                monitors,
                needs_placement: PLACE_ON_FIRST_FRAME,
                started: Instant::now(),
                exit_after,
            }))
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_whole_and_fractional_seconds() {
        assert_eq!(parse_exit_after("5"), Some(Duration::from_secs(5)));
        assert_eq!(parse_exit_after(" 0.5 "), Some(Duration::from_millis(500)));
        assert_eq!(parse_exit_after("0"), Some(Duration::ZERO));
    }

    #[test]
    fn rejects_unusable_values() {
        assert_eq!(parse_exit_after(""), None);
        assert_eq!(parse_exit_after("soon"), None);
        assert_eq!(parse_exit_after("-1"), None);
        assert_eq!(parse_exit_after("NaN"), None);
        assert_eq!(parse_exit_after("inf"), None);
    }
}

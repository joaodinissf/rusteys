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
use std::time::Instant;

struct KeyDisplayApp {
    state: SharedState,
    monitors: Vec<MonitorInfo>,
}

impl eframe::App for KeyDisplayApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let now = Instant::now();

        // Create secondary viewports for additional monitors (every frame)
        for (i, monitor) in self.monitors.iter().enumerate().skip(1) {
            let state = Arc::clone(&self.state);
            ui.ctx().show_viewport_immediate(
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

fn main() -> Result<(), eframe::Error> {
    let state = SharedState::default();
    let monitors = monitors::detect();
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
            Ok(Box::new(KeyDisplayApp { state, monitors }))
        }),
    )
}

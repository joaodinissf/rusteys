//! Global keyboard capture on a background thread.

use crate::input::ComboTracker;
use crate::state::{CaptureStatus, SharedState};
use eframe::egui;
use rdev::{Event, listen};
use std::thread;
use std::time::Instant;

/// Starts listening to global key events and records them in `state`.
///
/// The capture status in `state` becomes [`CaptureStatus::Running`] on the
/// first event and [`CaptureStatus::Failed`] if the listener can't start
/// (on macOS, typically because Input Monitoring hasn't been granted).
/// `ctx` is woken whenever something visible changes, so the UI doesn't
/// have to poll.
pub fn spawn_capture(state: SharedState, ctx: egui::Context) {
    thread::spawn(move || {
        let mut tracker = ComboTracker::default();
        let mut seen_event = false;
        let callback = {
            let state = state.clone();
            let ctx = ctx.clone();
            move |event: Event| {
                let text = tracker.on_event(&event.event_type);
                if seen_event && text.is_none() {
                    return;
                }
                {
                    let mut state = state.lock();
                    if !seen_event {
                        seen_event = true;
                        state.capture = CaptureStatus::Running;
                    }
                    if let Some(text) = text {
                        state.history.push(text, Instant::now());
                    }
                }
                ctx.request_repaint();
            }
        };

        // rdev asks to be told when `listen` runs off the main thread.
        #[cfg(target_os = "macos")]
        rdev::set_is_main_thread(false);

        if let Err(error) = listen(callback) {
            eprintln!("Error listening to keyboard events: {error:?}");
            state.lock().capture = CaptureStatus::Failed(format!("{error:?}"));
            ctx.request_repaint();
        }
    });
}

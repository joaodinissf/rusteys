//! Global keyboard capture on a background thread.

use crate::input::ComboTracker;
use crate::state::SharedState;
use eframe::egui;
use rdev::{Event, listen};
use std::thread;
use std::time::Instant;

/// Starts listening to global key events and records them in `state`.
///
/// `_ctx` is the UI context, kept in the signature so the capture thread
/// can wake the UI when something changes.
pub fn spawn_capture(state: SharedState, _ctx: egui::Context) {
    thread::spawn(move || {
        let mut tracker = ComboTracker::default();
        let callback = move |event: Event| {
            if let Some(text) = tracker.on_event(&event.event_type) {
                state.lock().history.push(text, Instant::now());
            }
        };

        // rdev asks to be told when `listen` runs off the main thread.
        #[cfg(target_os = "macos")]
        rdev::set_is_main_thread(false);

        if let Err(error) = listen(callback) {
            eprintln!("Error listening to keyboard events: {error:?}");
        }
    });
}

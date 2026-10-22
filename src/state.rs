//! State shared between the input capture thread and the UI.

use crate::history::KeyHistory;
use parking_lot::Mutex;
use std::sync::Arc;

/// Whether the global keyboard capture is working.
// Not reported anywhere yet; the capture thread and the overlay will use it.
#[allow(dead_code)]
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum CaptureStatus {
    /// The listener has been started but no event has arrived yet.
    #[default]
    Starting,
    /// At least one event has been received.
    Running,
    /// The listener failed, with a description of the error.
    Failed(String),
}

/// Everything the overlay needs to draw a frame.
#[derive(Debug, Default)]
pub struct OverlayState {
    pub history: KeyHistory,
    // Not read yet; see `CaptureStatus`.
    #[allow(dead_code)]
    pub capture: CaptureStatus,
}

/// [`OverlayState`] shared between threads.
pub type SharedState = Arc<Mutex<OverlayState>>;

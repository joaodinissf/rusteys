//! The list of recent key presses and their fade/pop animation curves.

use crate::config::{
    FADE_OUT_DURATION, KEY_DISPLAY_DURATION, MAX_KEYS, POP_DURATION, POP_EXTRA_SCALE,
};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// One displayed key press or combination.
#[derive(Debug, Clone)]
pub struct KeyPress {
    pub text: String,
    pub timestamp: Instant,
}

/// Recent key presses, oldest first, capped at [`MAX_KEYS`].
#[derive(Debug, Clone, Default)]
pub struct KeyHistory {
    entries: VecDeque<KeyPress>,
}

impl KeyHistory {
    /// Appends a key press, dropping the oldest ones beyond [`MAX_KEYS`].
    pub fn push(&mut self, text: String, now: Instant) {
        self.entries.push_back(KeyPress {
            text,
            timestamp: now,
        });
        while self.entries.len() > MAX_KEYS {
            self.entries.pop_front();
        }
    }

    /// Drops the key presses that have fully faded out at `now`.
    pub fn prune(&mut self, now: Instant) {
        self.entries.retain(|kp| {
            now.duration_since(kp.timestamp) < KEY_DISPLAY_DURATION + FADE_OUT_DURATION
        });
    }

    /// Iterates over the key presses, oldest first.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &KeyPress> {
        self.entries.iter()
    }
}

/// Opacity (0.0 to 1.0) of a key press of the given age.
pub fn fade_alpha(age: Duration) -> f32 {
    if age > KEY_DISPLAY_DURATION {
        let fade_progress =
            (age - KEY_DISPLAY_DURATION).as_secs_f32() / FADE_OUT_DURATION.as_secs_f32();
        1.0 - fade_progress.min(1.0)
    } else {
        1.0
    }
}

/// Size multiplier of a key press of the given age: slightly larger when first pressed.
pub fn pop_scale(age: Duration) -> f32 {
    if age < POP_DURATION {
        1.0 + (1.0 - age.as_secs_f32() / POP_DURATION.as_secs_f32()) * POP_EXTRA_SCALE
    } else {
        1.0
    }
}

//! Turns raw key events into the labels shown on the overlay.
//!
//! This is pure logic with no I/O, so it can be unit-tested.

use crate::keys::{ModKind, key_label, modifier_kind};
use rdev::EventType;

/// Tracks held modifiers and builds labels such as `Ctrl + Shift + S`.
///
/// A non-modifier key press yields its label prefixed with the held
/// modifiers. A modifier yields its own label on release, but only if it
/// was not used in a combination while it was held.
#[derive(Debug, Default)]
pub struct ComboTracker {
    /// Whether each modifier (indexed by [`ModKind::index`]) is held.
    held: [bool; ModKind::ALL.len()],
    /// Whether each held modifier has been used in a combination.
    used: [bool; ModKind::ALL.len()],
}

impl ComboTracker {
    /// Feeds one input event and returns the label to display, if any.
    pub fn on_event(&mut self, event: &EventType) -> Option<String> {
        match *event {
            EventType::KeyPress(key) => {
                if let Some(kind) = modifier_kind(key) {
                    self.held[kind.index()] = true;
                    return None;
                }

                let mut text = String::new();
                let mods = self.format_held();
                if !mods.is_empty() {
                    text.push_str(&mods);
                    text.push_str(" + ");
                    // Mark the held modifiers as used in a combination.
                    for kind in ModKind::ALL {
                        if self.held[kind.index()] {
                            self.used[kind.index()] = true;
                        }
                    }
                }
                text.push_str(&key_label(key));
                Some(text)
            }
            EventType::KeyRelease(key) => {
                let kind = modifier_kind(key)?;
                // Show the modifier on its own only if it wasn't part of a combination.
                let label = (!self.used[kind.index()]).then(|| key_label(key).into_owned());
                self.held[kind.index()] = false;
                self.used[kind.index()] = false;
                label
            }
            _ => None,
        }
    }

    /// Joins the held modifiers, e.g. `Ctrl + Shift`, or returns an empty string.
    fn format_held(&self) -> String {
        ModKind::ALL
            .into_iter()
            .filter(|kind| self.held[kind.index()])
            .map(ModKind::label)
            .collect::<Vec<_>>()
            .join(" + ")
    }
}

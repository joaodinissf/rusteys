//! Turns raw key events into the labels shown on the overlay.
//!
//! This is pure logic with no I/O, so it can be unit-tested.

use crate::keys::{LabelStyle, ModKind, key_label_for, modifier_kind};
use rdev::{EventType, Key};

/// Tracks held modifiers and builds labels such as `Ctrl + Shift + S`.
///
/// A non-modifier key press yields its label prefixed with the held
/// modifiers. A modifier yields its own label on release, but only if it
/// was not used in a combination while it was held.
///
/// Left and right modifier keys are tracked separately: a modifier family
/// counts as held while either of its keys is down, so releasing one Shift
/// while the other is still held keeps Shift active.
#[derive(Debug)]
pub struct ComboTracker {
    /// Naming convention for the produced labels.
    style: LabelStyle,
    /// Whether the left (`0`) and right (`1`) key of each modifier family,
    /// indexed by [`ModKind::index`], is held.
    held: [[bool; 2]; ModKind::ALL.len()],
    /// Whether each held modifier family has been used in a combination.
    used: [bool; ModKind::ALL.len()],
}

impl Default for ComboTracker {
    fn default() -> Self {
        Self::with_style(LabelStyle::current())
    }
}

impl ComboTracker {
    /// Creates a tracker that labels keys in the given `style`.
    pub fn with_style(style: LabelStyle) -> Self {
        Self {
            style,
            held: Default::default(),
            used: Default::default(),
        }
    }

    /// Feeds one input event and returns the label to display, if any.
    pub fn on_event(&mut self, event: &EventType) -> Option<String> {
        match *event {
            EventType::KeyPress(key) => {
                if let Some(kind) = modifier_kind(key) {
                    // Auto-repeat of a held modifier just sets the flag again.
                    self.held[kind.index()][side(key)] = true;
                    return None;
                }

                let mut text = String::new();
                let mods = self.format_held();
                if !mods.is_empty() {
                    text.push_str(&mods);
                    text.push_str(" + ");
                    // Mark the held modifiers as used in a combination.
                    for kind in ModKind::ALL {
                        if self.is_held(kind) {
                            self.used[kind.index()] = true;
                        }
                    }
                }
                text.push_str(&key_label_for(key, self.style));
                Some(text)
            }
            EventType::KeyRelease(key) => {
                let kind = modifier_kind(key)?;
                self.held[kind.index()][side(key)] = false;
                if self.is_held(kind) {
                    // The other key of the same family is still down.
                    return None;
                }
                // Show the modifier on its own only if it wasn't part of a combination.
                let label =
                    (!self.used[kind.index()]).then(|| key_label_for(key, self.style).into_owned());
                self.used[kind.index()] = false;
                label
            }
            _ => None,
        }
    }

    /// Whether either key of the `kind` modifier family is held.
    fn is_held(&self, kind: ModKind) -> bool {
        self.held[kind.index()].contains(&true)
    }

    /// Joins the held modifiers, e.g. `Ctrl + Shift`, or returns an empty string.
    fn format_held(&self) -> String {
        ModKind::display_order(self.style)
            .into_iter()
            .filter(|&kind| self.is_held(kind))
            .map(|kind| kind.label_for(self.style))
            .collect::<Vec<_>>()
            .join(" + ")
    }
}

/// Which physical key of a modifier family `key` is: `0` for left, `1` for right.
fn side(key: Key) -> usize {
    match key {
        Key::ControlRight | Key::ShiftRight | Key::AltGr | Key::MetaRight => 1,
        _ => 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rdev::EventType::{KeyPress, KeyRelease};

    /// Feeds `events` to a fresh tracker and collects every produced label.
    fn run(style: LabelStyle, events: &[EventType]) -> Vec<String> {
        let mut tracker = ComboTracker::with_style(style);
        events.iter().filter_map(|e| tracker.on_event(e)).collect()
    }

    #[test]
    fn plain_key_press_yields_label_and_release_yields_nothing() {
        let out = run(
            LabelStyle::Pc,
            &[KeyPress(Key::KeyA), KeyRelease(Key::KeyA)],
        );
        assert_eq!(out, ["A"]);
    }

    #[test]
    fn ctrl_s_yields_only_the_combo() {
        let out = run(
            LabelStyle::Pc,
            &[
                KeyPress(Key::ControlLeft),
                KeyPress(Key::KeyS),
                KeyRelease(Key::KeyS),
                KeyRelease(Key::ControlLeft),
            ],
        );
        assert_eq!(out, ["Ctrl + S"]);
    }

    #[test]
    fn cmd_s_on_mac_uses_words() {
        let out = run(
            LabelStyle::Mac,
            &[
                KeyPress(Key::MetaLeft),
                KeyPress(Key::KeyS),
                KeyRelease(Key::KeyS),
                KeyRelease(Key::MetaLeft),
            ],
        );
        assert_eq!(out, ["Cmd + S"]);
    }

    #[test]
    fn lone_shift_tap_yields_shift_on_release() {
        let mut tracker = ComboTracker::with_style(LabelStyle::Pc);
        assert_eq!(tracker.on_event(&KeyPress(Key::ShiftLeft)), None);
        assert_eq!(
            tracker.on_event(&KeyRelease(Key::ShiftLeft)).as_deref(),
            Some("Shift")
        );
    }

    #[test]
    fn shift_held_through_a_yields_only_the_combo() {
        let out = run(
            LabelStyle::Pc,
            &[
                KeyPress(Key::ShiftLeft),
                KeyPress(Key::KeyA),
                KeyRelease(Key::KeyA),
                KeyRelease(Key::ShiftLeft),
            ],
        );
        assert_eq!(out, ["Shift + A"]);
    }

    #[test]
    fn releasing_one_of_two_shifts_keeps_shift_applied() {
        let out = run(
            LabelStyle::Pc,
            &[
                KeyPress(Key::ShiftLeft),
                KeyPress(Key::ShiftRight),
                KeyRelease(Key::ShiftLeft),
                KeyPress(Key::KeyA),
                KeyRelease(Key::KeyA),
                KeyRelease(Key::ShiftRight),
            ],
        );
        assert_eq!(out, ["Shift + A"]);
    }

    #[test]
    fn tapping_both_shifts_yields_one_shift() {
        let out = run(
            LabelStyle::Pc,
            &[
                KeyPress(Key::ShiftLeft),
                KeyPress(Key::ShiftRight),
                KeyRelease(Key::ShiftRight),
                KeyRelease(Key::ShiftLeft),
            ],
        );
        assert_eq!(out, ["Shift"]);
    }

    #[test]
    fn auto_repeated_modifier_press_is_released_by_one_release() {
        let out = run(
            LabelStyle::Pc,
            &[
                KeyPress(Key::ControlLeft),
                KeyPress(Key::ControlLeft),
                KeyPress(Key::ControlLeft),
                KeyRelease(Key::ControlLeft),
                KeyPress(Key::KeyC),
            ],
        );
        assert_eq!(out, ["Ctrl", "C"]);
    }

    #[test]
    fn modifier_order_follows_platform() {
        let events = [
            KeyPress(Key::MetaLeft),
            KeyPress(Key::ShiftLeft),
            KeyPress(Key::Alt),
            KeyPress(Key::ControlLeft),
            KeyPress(Key::KeyK),
        ];
        assert_eq!(
            run(LabelStyle::Pc, &events),
            ["Ctrl + Shift + Alt + Win + K"]
        );
        assert_eq!(
            run(LabelStyle::Mac, &events),
            ["Control + Option + Shift + Cmd + K"]
        );
    }

    #[test]
    fn right_option_on_mac_is_option() {
        let out = run(
            LabelStyle::Mac,
            &[
                KeyPress(Key::AltGr),
                KeyPress(Key::KeyE),
                KeyRelease(Key::KeyE),
                KeyRelease(Key::AltGr),
                KeyPress(Key::AltGr),
                KeyRelease(Key::AltGr),
            ],
        );
        assert_eq!(out, ["Option + E", "Option"]);
    }

    #[test]
    fn modifier_used_then_released_resets_for_next_tap() {
        let out = run(
            LabelStyle::Pc,
            &[
                KeyPress(Key::ControlLeft),
                KeyPress(Key::KeyZ),
                KeyRelease(Key::ControlLeft),
                KeyPress(Key::ControlRight),
                KeyRelease(Key::ControlRight),
            ],
        );
        assert_eq!(out, ["Ctrl + Z", "Ctrl"]);
    }

    #[test]
    fn non_key_events_are_ignored() {
        let out = run(
            LabelStyle::Pc,
            &[
                EventType::MouseMove { x: 1.0, y: 2.0 },
                EventType::ButtonPress(rdev::Button::Left),
            ],
        );
        assert!(out.is_empty());
    }

    #[test]
    fn punctuation_in_combo_uses_symbol() {
        let out = run(
            LabelStyle::Pc,
            &[KeyPress(Key::ControlLeft), KeyPress(Key::SemiColon)],
        );
        assert_eq!(out, ["Ctrl + ;"]);
    }
}

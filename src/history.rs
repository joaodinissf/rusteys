//! The list of recent key presses and their fade/pop animation curves.

use crate::config::{
    FADE_OUT_DURATION, KEY_DISPLAY_DURATION, MAX_KEYS, POP_DURATION, POP_EXTRA_SCALE,
};
use std::borrow::Cow;
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// A repeat of the newest label arriving within this window is collapsed into it
/// (shown as `"A ×3"`) instead of taking a new slot. Holding a key auto-repeats
/// well inside this window, so a held key becomes one growing chip.
pub const REPEAT_WINDOW: Duration = Duration::from_millis(700);

/// One displayed key press or combination, possibly repeated.
#[derive(Debug, Clone)]
pub struct KeyPress {
    /// The key or combination label, without the repeat count.
    pub text: String,
    /// How many consecutive times this label was pressed (at least 1).
    pub count: u32,
    /// When this label was last pressed.
    pub timestamp: Instant,
}

impl KeyPress {
    /// The text to draw: the label, followed by `×N` when it was repeated.
    pub fn display(&self) -> Cow<'_, str> {
        if self.count > 1 {
            Cow::Owned(format!("{} ×{}", self.text, self.count))
        } else {
            Cow::Borrowed(&self.text)
        }
    }
}

/// Recent key presses, oldest first, capped at [`MAX_KEYS`].
#[derive(Debug, Clone, Default)]
pub struct KeyHistory {
    entries: VecDeque<KeyPress>,
}

impl KeyHistory {
    /// Records a key press at `now`.
    ///
    /// If it repeats the newest label within [`REPEAT_WINDOW`], that entry's count is
    /// bumped and its timestamp refreshed. Otherwise a new entry is appended and the
    /// oldest ones beyond [`MAX_KEYS`] are dropped.
    pub fn push(&mut self, text: String, now: Instant) {
        if let Some(newest) = self.entries.back_mut()
            && newest.text == text
            && now.saturating_duration_since(newest.timestamp) <= REPEAT_WINDOW
        {
            newest.count = newest.count.saturating_add(1);
            newest.timestamp = now;
            return;
        }
        self.entries.push_back(KeyPress {
            text,
            count: 1,
            timestamp: now,
        });
        while self.entries.len() > MAX_KEYS {
            self.entries.pop_front();
        }
    }

    /// Drops the key presses that have fully faded out at `now`.
    pub fn prune(&mut self, now: Instant) {
        self.entries.retain(|kp| {
            now.saturating_duration_since(kp.timestamp) < KEY_DISPLAY_DURATION + FADE_OUT_DURATION
        });
    }

    /// Iterates over the key presses, oldest first.
    pub fn iter(&self) -> impl DoubleEndedIterator<Item = &KeyPress> {
        self.entries.iter()
    }

    /// Time from `now` until the next point where any entry's appearance changes
    /// in a way an idle UI must repaint for: the end of its pop, the start of its
    /// fade, or the end of its fade (when it should disappear).
    ///
    /// Returns `None` when there is nothing left to animate (empty history, or every
    /// entry is already past the end of its fade and only awaits [`Self::prune`]).
    /// A returned `Duration::ZERO` never happens: boundaries exactly at `now` are
    /// treated as passed.
    pub fn next_deadline(&self, now: Instant) -> Option<Duration> {
        let milestones = [
            POP_DURATION,
            KEY_DISPLAY_DURATION,
            KEY_DISPLAY_DURATION + FADE_OUT_DURATION,
        ];
        self.entries
            .iter()
            .filter_map(|kp| {
                let age = now.saturating_duration_since(kp.timestamp);
                milestones
                    .iter()
                    .filter(|&&m| m > age)
                    .map(|&m| m - age)
                    .min()
            })
            .min()
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

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f32 = 1e-5;

    fn ms(n: u64) -> Duration {
        Duration::from_millis(n)
    }

    fn labels(h: &KeyHistory) -> Vec<String> {
        h.iter().map(|kp| kp.display().into_owned()).collect()
    }

    #[test]
    fn push_caps_at_max_keys_dropping_oldest() {
        let t0 = Instant::now();
        let mut h = KeyHistory::default();
        for i in 0..MAX_KEYS + 5 {
            h.push(format!("K{i}"), t0 + ms(i as u64));
        }
        assert_eq!(h.iter().count(), MAX_KEYS);
        assert_eq!(h.iter().next().unwrap().text, "K5");
        assert_eq!(
            h.iter().next_back().unwrap().text,
            format!("K{}", MAX_KEYS + 4)
        );
    }

    #[test]
    fn prune_keeps_entry_just_before_boundary_and_drops_at_boundary() {
        let t0 = Instant::now();
        let total = KEY_DISPLAY_DURATION + FADE_OUT_DURATION;
        let mut h = KeyHistory::default();
        h.push("A".into(), t0);

        h.prune(t0 + total - ms(1));
        assert_eq!(h.iter().count(), 1);

        h.prune(t0 + total);
        assert_eq!(h.iter().count(), 0);
    }

    #[test]
    fn prune_drops_only_expired_entries() {
        let t0 = Instant::now();
        let total = KEY_DISPLAY_DURATION + FADE_OUT_DURATION;
        let mut h = KeyHistory::default();
        h.push("A".into(), t0);
        h.push("B".into(), t0 + ms(1000));
        h.prune(t0 + total);
        assert_eq!(labels(&h), ["B"]);
    }

    #[test]
    fn repeats_inside_window_collapse_into_one_entry() {
        let t0 = Instant::now();
        let mut h = KeyHistory::default();
        h.push("A".into(), t0);
        h.push("A".into(), t0 + ms(30));
        h.push("A".into(), t0 + ms(60));
        assert_eq!(labels(&h), ["A ×3"]);
        let kp = h.iter().next().unwrap();
        assert_eq!(kp.count, 3);
        assert_eq!(kp.timestamp, t0 + ms(60), "timestamp is refreshed");
    }

    #[test]
    fn repeat_window_is_measured_from_the_latest_repeat() {
        let t0 = Instant::now();
        let mut h = KeyHistory::default();
        // Each step is within the window of the previous one, though the last
        // is far beyond the window of the first: a long hold stays one chip.
        for i in 0..5 {
            h.push("J".into(), t0 + REPEAT_WINDOW * i);
        }
        assert_eq!(labels(&h), ["J ×5"]);
    }

    #[test]
    fn repeat_exactly_at_window_edge_collapses() {
        let t0 = Instant::now();
        let mut h = KeyHistory::default();
        h.push("A".into(), t0);
        h.push("A".into(), t0 + REPEAT_WINDOW);
        assert_eq!(labels(&h), ["A ×2"]);
    }

    #[test]
    fn repeat_outside_window_pushes_new_entry() {
        let t0 = Instant::now();
        let mut h = KeyHistory::default();
        h.push("A".into(), t0);
        h.push("A".into(), t0 + REPEAT_WINDOW + ms(1));
        assert_eq!(labels(&h), ["A", "A"]);
    }

    #[test]
    fn different_label_breaks_the_repeat() {
        let t0 = Instant::now();
        let mut h = KeyHistory::default();
        h.push("A".into(), t0);
        h.push("B".into(), t0 + ms(10));
        h.push("A".into(), t0 + ms(20));
        assert_eq!(labels(&h), ["A", "B", "A"]);
    }

    #[test]
    fn collapsing_does_not_consume_slots() {
        let t0 = Instant::now();
        let mut h = KeyHistory::default();
        h.push("X".into(), t0);
        for i in 1..=(MAX_KEYS as u64 * 2) {
            h.push("J".into(), t0 + ms(i * 30));
        }
        assert_eq!(
            labels(&h),
            ["X".to_string(), format!("J ×{}", MAX_KEYS * 2)]
        );
    }

    #[test]
    fn display_without_repeat_borrows_label() {
        let kp = KeyPress {
            text: "Ctrl + S".into(),
            count: 1,
            timestamp: Instant::now(),
        };
        assert!(matches!(kp.display(), Cow::Borrowed("Ctrl + S")));
    }

    #[test]
    fn fade_alpha_at_start_mid_fade_and_end() {
        assert!((fade_alpha(Duration::ZERO) - 1.0).abs() < EPS);
        assert!((fade_alpha(KEY_DISPLAY_DURATION) - 1.0).abs() < EPS);
        assert!((fade_alpha(KEY_DISPLAY_DURATION + FADE_OUT_DURATION / 2) - 0.5).abs() < EPS);
        assert!(fade_alpha(KEY_DISPLAY_DURATION + FADE_OUT_DURATION).abs() < EPS);
        assert!(fade_alpha(KEY_DISPLAY_DURATION + FADE_OUT_DURATION * 3).abs() < EPS);
    }

    #[test]
    fn pop_scale_endpoints() {
        assert!((pop_scale(Duration::ZERO) - (1.0 + POP_EXTRA_SCALE)).abs() < EPS);
        assert!((pop_scale(POP_DURATION / 2) - (1.0 + POP_EXTRA_SCALE / 2.0)).abs() < EPS);
        assert!((pop_scale(POP_DURATION) - 1.0).abs() < EPS);
        assert!((pop_scale(ms(10_000)) - 1.0).abs() < EPS);
    }

    #[test]
    fn next_deadline_empty_is_none() {
        assert_eq!(KeyHistory::default().next_deadline(Instant::now()), None);
    }

    #[test]
    fn next_deadline_walks_pop_end_fade_start_fade_end() {
        let t0 = Instant::now();
        let mut h = KeyHistory::default();
        h.push("A".into(), t0);

        assert_eq!(h.next_deadline(t0), Some(POP_DURATION));
        assert_eq!(
            h.next_deadline(t0 + POP_DURATION),
            Some(KEY_DISPLAY_DURATION - POP_DURATION)
        );
        assert_eq!(
            h.next_deadline(t0 + KEY_DISPLAY_DURATION),
            Some(FADE_OUT_DURATION)
        );
        assert_eq!(
            h.next_deadline(t0 + KEY_DISPLAY_DURATION + FADE_OUT_DURATION),
            None
        );
    }

    #[test]
    fn next_deadline_is_soonest_across_entries() {
        let t0 = Instant::now();
        let mut h = KeyHistory::default();
        h.push("A".into(), t0);
        h.push("B".into(), t0 + ms(3000));
        let now = t0 + ms(3500);
        // A starts fading at 4000 (500 ms away); B's next milestone is its fade
        // start at 7000 (3500 ms away). The soonest wins.
        assert_eq!(h.next_deadline(now), Some(ms(500)));
    }
}

//! Mapping from `rdev` keys to display labels.
//!
//! Labels are plain words (`Cmd + S`, not `⌘S`). Which words are used depends
//! on the [`LabelStyle`]: macOS names (`Control`, `Option`, `Cmd`, `Return`)
//! or the PC names used on Windows and Linux (`Ctrl`, `Alt`, `Win`, `Enter`).

use rdev::Key;
use std::borrow::Cow;

/// The naming convention used for key labels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LabelStyle {
    /// macOS names: `Control`, `Option`, `Shift`, `Cmd`, `Return`, ...
    Mac,
    /// PC names used on Windows and Linux: `Ctrl`, `Shift`, `Alt`, `Win`, `Enter`, ...
    Pc,
}

impl LabelStyle {
    /// The style that matches the platform this binary was built for.
    pub const fn current() -> Self {
        if cfg!(target_os = "macos") {
            LabelStyle::Mac
        } else {
            LabelStyle::Pc
        }
    }
}

/// The modifier families tracked when building key combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModKind {
    Ctrl,
    Shift,
    Alt,
    Meta,
}

impl ModKind {
    /// All modifiers, indexed by [`ModKind::index`].
    pub const ALL: [ModKind; 4] = [ModKind::Ctrl, ModKind::Shift, ModKind::Alt, ModKind::Meta];

    /// The order modifiers appear in a combination label for `style`.
    ///
    /// macOS menus list modifiers as Control, Option, Shift, Cmd; elsewhere the
    /// usual order is Ctrl, Shift, Alt, Win.
    pub const fn display_order(style: LabelStyle) -> [ModKind; 4] {
        match style {
            LabelStyle::Mac => [ModKind::Ctrl, ModKind::Alt, ModKind::Shift, ModKind::Meta],
            LabelStyle::Pc => [ModKind::Ctrl, ModKind::Shift, ModKind::Alt, ModKind::Meta],
        }
    }

    /// Label used for this modifier inside a combination such as `Ctrl + S`,
    /// in the platform's [`LabelStyle::current`].
    #[allow(
        dead_code,
        reason = "public convenience API from the module split; no caller outside tests yet"
    )]
    pub fn label(self) -> &'static str {
        self.label_for(LabelStyle::current())
    }

    /// Label used for this modifier in the given `style`.
    pub const fn label_for(self, style: LabelStyle) -> &'static str {
        match (style, self) {
            (LabelStyle::Mac, ModKind::Ctrl) => "Control",
            (LabelStyle::Mac, ModKind::Alt) => "Option",
            (LabelStyle::Mac, ModKind::Meta) => "Cmd",
            (LabelStyle::Pc, ModKind::Ctrl) => "Ctrl",
            (LabelStyle::Pc, ModKind::Alt) => "Alt",
            (LabelStyle::Pc, ModKind::Meta) => "Win",
            (_, ModKind::Shift) => "Shift",
        }
    }

    /// Position of this modifier in [`ModKind::ALL`].
    pub fn index(self) -> usize {
        self as usize
    }
}

/// Returns the modifier family of `key`, or `None` if it is not a modifier.
pub fn modifier_kind(key: Key) -> Option<ModKind> {
    match key {
        Key::ControlLeft | Key::ControlRight => Some(ModKind::Ctrl),
        Key::ShiftLeft | Key::ShiftRight => Some(ModKind::Shift),
        // On macOS rdev reports the right Option key as `AltGr`.
        Key::Alt | Key::AltGr => Some(ModKind::Alt),
        Key::MetaLeft | Key::MetaRight => Some(ModKind::Meta),
        _ => None,
    }
}

/// Returns the label shown on screen for `key`, in the platform's
/// [`LabelStyle::current`].
#[allow(
    dead_code,
    reason = "public convenience API from the module split; no caller outside tests yet"
)]
pub fn key_label(key: Key) -> Cow<'static, str> {
    key_label_for(key, LabelStyle::current())
}

/// Returns the label shown on screen for `key` in the given `style`.
pub fn key_label_for(key: Key, style: LabelStyle) -> Cow<'static, str> {
    let mac = style == LabelStyle::Mac;
    let label = match key {
        // Modifiers. macOS has no AltGr: rdev reports the right Option key as `AltGr`.
        Key::AltGr if !mac => "AltGr",
        Key::Alt | Key::AltGr => ModKind::Alt.label_for(style),
        Key::ControlLeft | Key::ControlRight => ModKind::Ctrl.label_for(style),
        Key::ShiftLeft | Key::ShiftRight => ModKind::Shift.label_for(style),
        Key::MetaLeft | Key::MetaRight => ModKind::Meta.label_for(style),

        // Editing and navigation.
        // The Mac "delete" key is Backspace; the Mac "forward delete" key is Delete.
        Key::Backspace if mac => "Delete",
        Key::Backspace => "Backspace",
        Key::Delete if mac => "Fwd Delete",
        Key::Delete => "Delete",
        Key::Return if mac => "Return",
        Key::Return => "Enter",
        Key::Escape => "Esc",
        Key::Tab => "Tab",
        Key::Space => "Space",
        Key::CapsLock => "CapsLock",
        Key::Insert => "Insert",
        Key::Home => "Home",
        Key::End => "End",
        Key::PageUp => "PgUp",
        Key::PageDown => "PgDn",
        Key::UpArrow => "Up",
        Key::DownArrow => "Down",
        Key::LeftArrow => "Left",
        Key::RightArrow => "Right",
        Key::PrintScreen => "PrtSc",
        Key::ScrollLock => "ScrollLock",
        Key::Pause => "Pause",
        Key::NumLock => "NumLock",
        Key::Function => "Fn",

        // Function keys.
        Key::F1 => "F1",
        Key::F2 => "F2",
        Key::F3 => "F3",
        Key::F4 => "F4",
        Key::F5 => "F5",
        Key::F6 => "F6",
        Key::F7 => "F7",
        Key::F8 => "F8",
        Key::F9 => "F9",
        Key::F10 => "F10",
        Key::F11 => "F11",
        Key::F12 => "F12",
        Key::F13 => "F13",
        Key::F14 => "F14",
        Key::F15 => "F15",
        Key::F16 => "F16",
        Key::F17 => "F17",
        Key::F18 => "F18",
        Key::F19 => "F19",
        Key::F20 => "F20",
        Key::F21 => "F21",
        Key::F22 => "F22",
        Key::F23 => "F23",
        Key::F24 => "F24",

        // Digit row.
        Key::Num0 => "0",
        Key::Num1 => "1",
        Key::Num2 => "2",
        Key::Num3 => "3",
        Key::Num4 => "4",
        Key::Num5 => "5",
        Key::Num6 => "6",
        Key::Num7 => "7",
        Key::Num8 => "8",
        Key::Num9 => "9",

        // Letters.
        Key::KeyA => "A",
        Key::KeyB => "B",
        Key::KeyC => "C",
        Key::KeyD => "D",
        Key::KeyE => "E",
        Key::KeyF => "F",
        Key::KeyG => "G",
        Key::KeyH => "H",
        Key::KeyI => "I",
        Key::KeyJ => "J",
        Key::KeyK => "K",
        Key::KeyL => "L",
        Key::KeyM => "M",
        Key::KeyN => "N",
        Key::KeyO => "O",
        Key::KeyP => "P",
        Key::KeyQ => "Q",
        Key::KeyR => "R",
        Key::KeyS => "S",
        Key::KeyT => "T",
        Key::KeyU => "U",
        Key::KeyV => "V",
        Key::KeyW => "W",
        Key::KeyX => "X",
        Key::KeyY => "Y",
        Key::KeyZ => "Z",

        // Punctuation (US layout positions, as rdev reports them).
        Key::Minus => "-",
        Key::Equal => "=",
        Key::LeftBracket => "[",
        Key::RightBracket => "]",
        Key::SemiColon => ";",
        Key::Quote => "'",
        Key::BackQuote => "`",
        Key::BackSlash | Key::IntlBackslash => "\\",
        Key::Comma => ",",
        Key::Dot => ".",
        Key::Slash => "/",

        // Numeric keypad.
        Key::Kp0 => "Num 0",
        Key::Kp1 => "Num 1",
        Key::Kp2 => "Num 2",
        Key::Kp3 => "Num 3",
        Key::Kp4 => "Num 4",
        Key::Kp5 => "Num 5",
        Key::Kp6 => "Num 6",
        Key::Kp7 => "Num 7",
        Key::Kp8 => "Num 8",
        Key::Kp9 => "Num 9",
        Key::KpPlus => "Num +",
        Key::KpMinus => "Num -",
        Key::KpMultiply => "Num *",
        Key::KpDivide => "Num /",
        Key::KpReturn if mac => "Num Return",
        Key::KpReturn => "Num Enter",
        Key::KpDelete => "Num .",

        // Media keys.
        Key::VolumeUp => "Volume Up",
        Key::VolumeDown => "Volume Down",
        Key::VolumeMute => "Mute",
        Key::BrightnessUp => "Brightness Up",
        Key::BrightnessDown => "Brightness Down",
        Key::PreviousTrack => "Previous Track",
        Key::PlayPause => "Play/Pause",
        Key::PlayCd => "Play",
        Key::NextTrack => "Next Track",

        Key::Unknown(code) => return Cow::Owned(format!("Key {code}")),
    };
    Cow::Borrowed(label)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn modifier_labels_per_style() {
        assert_eq!(ModKind::Ctrl.label_for(LabelStyle::Pc), "Ctrl");
        assert_eq!(ModKind::Alt.label_for(LabelStyle::Pc), "Alt");
        assert_eq!(ModKind::Meta.label_for(LabelStyle::Pc), "Win");
        assert_eq!(ModKind::Ctrl.label_for(LabelStyle::Mac), "Control");
        assert_eq!(ModKind::Alt.label_for(LabelStyle::Mac), "Option");
        assert_eq!(ModKind::Meta.label_for(LabelStyle::Mac), "Cmd");
        assert_eq!(ModKind::Shift.label_for(LabelStyle::Mac), "Shift");
        assert_eq!(ModKind::Shift.label_for(LabelStyle::Pc), "Shift");
    }

    #[test]
    fn modifier_display_order_per_style() {
        use ModKind::*;
        assert_eq!(
            ModKind::display_order(LabelStyle::Mac),
            [Ctrl, Alt, Shift, Meta]
        );
        assert_eq!(
            ModKind::display_order(LabelStyle::Pc),
            [Ctrl, Shift, Alt, Meta]
        );
    }

    #[test]
    fn modifier_key_labels() {
        assert_eq!(key_label_for(Key::MetaLeft, LabelStyle::Mac), "Cmd");
        assert_eq!(key_label_for(Key::MetaRight, LabelStyle::Pc), "Win");
        assert_eq!(key_label_for(Key::Alt, LabelStyle::Mac), "Option");
        assert_eq!(key_label_for(Key::AltGr, LabelStyle::Mac), "Option");
        assert_eq!(key_label_for(Key::AltGr, LabelStyle::Pc), "AltGr");
        assert_eq!(key_label_for(Key::ControlRight, LabelStyle::Pc), "Ctrl");
        assert_eq!(key_label_for(Key::ShiftRight, LabelStyle::Mac), "Shift");
    }

    #[test]
    fn punctuation_labels() {
        let cases = [
            (Key::Minus, "-"),
            (Key::Equal, "="),
            (Key::LeftBracket, "["),
            (Key::RightBracket, "]"),
            (Key::SemiColon, ";"),
            (Key::Quote, "'"),
            (Key::BackQuote, "`"),
            (Key::BackSlash, "\\"),
            (Key::IntlBackslash, "\\"),
            (Key::Comma, ","),
            (Key::Dot, "."),
            (Key::Slash, "/"),
        ];
        for (key, expected) in cases {
            assert_eq!(key_label_for(key, LabelStyle::Pc), expected, "{key:?}");
            assert_eq!(key_label_for(key, LabelStyle::Mac), expected, "{key:?}");
        }
    }

    #[test]
    fn keypad_labels() {
        assert_eq!(key_label_for(Key::Kp0, LabelStyle::Pc), "Num 0");
        assert_eq!(key_label_for(Key::Kp7, LabelStyle::Pc), "Num 7");
        assert_eq!(key_label_for(Key::KpPlus, LabelStyle::Pc), "Num +");
        assert_eq!(key_label_for(Key::KpMinus, LabelStyle::Pc), "Num -");
        assert_eq!(key_label_for(Key::KpMultiply, LabelStyle::Pc), "Num *");
        assert_eq!(key_label_for(Key::KpDivide, LabelStyle::Pc), "Num /");
        assert_eq!(key_label_for(Key::KpDelete, LabelStyle::Pc), "Num .");
        assert_eq!(key_label_for(Key::KpReturn, LabelStyle::Pc), "Num Enter");
        assert_eq!(key_label_for(Key::KpReturn, LabelStyle::Mac), "Num Return");
        assert_eq!(key_label_for(Key::NumLock, LabelStyle::Pc), "NumLock");
    }

    #[test]
    fn platform_specific_key_labels() {
        assert_eq!(key_label_for(Key::Return, LabelStyle::Pc), "Enter");
        assert_eq!(key_label_for(Key::Return, LabelStyle::Mac), "Return");
        assert_eq!(key_label_for(Key::Backspace, LabelStyle::Pc), "Backspace");
        assert_eq!(key_label_for(Key::Backspace, LabelStyle::Mac), "Delete");
        assert_eq!(key_label_for(Key::Delete, LabelStyle::Pc), "Delete");
        assert_eq!(key_label_for(Key::Delete, LabelStyle::Mac), "Fwd Delete");
    }

    #[test]
    fn misc_key_labels() {
        assert_eq!(key_label_for(Key::KeyS, LabelStyle::Pc), "S");
        assert_eq!(key_label_for(Key::Num4, LabelStyle::Mac), "4");
        assert_eq!(key_label_for(Key::F13, LabelStyle::Pc), "F13");
        assert_eq!(key_label_for(Key::F24, LabelStyle::Mac), "F24");
        assert_eq!(key_label_for(Key::Function, LabelStyle::Mac), "Fn");
        assert_eq!(key_label_for(Key::Escape, LabelStyle::Mac), "Esc");
        assert_eq!(key_label_for(Key::VolumeMute, LabelStyle::Pc), "Mute");
        assert_eq!(key_label_for(Key::Unknown(123), LabelStyle::Pc), "Key 123");
    }

    #[test]
    fn current_style_matches_target() {
        #[cfg(target_os = "macos")]
        {
            assert_eq!(LabelStyle::current(), LabelStyle::Mac);
            assert_eq!(key_label(Key::MetaLeft), "Cmd");
            assert_eq!(ModKind::Alt.label(), "Option");
        }
        #[cfg(not(target_os = "macos"))]
        {
            assert_eq!(LabelStyle::current(), LabelStyle::Pc);
            assert_eq!(key_label(Key::MetaLeft), "Win");
            assert_eq!(ModKind::Alt.label(), "Alt");
        }
    }

    #[test]
    fn no_label_falls_back_to_debug_names() {
        // Labels never contain rdev's Debug-style identifiers.
        let keys = [
            Key::SemiColon,
            Key::BackQuote,
            Key::Kp7,
            Key::IntlBackslash,
            Key::KpReturn,
            Key::PlayPause,
        ];
        for key in keys {
            for style in [LabelStyle::Mac, LabelStyle::Pc] {
                let label = key_label_for(key, style);
                assert_ne!(label, format!("{key:?}"), "{key:?} in {style:?}");
            }
        }
    }
}

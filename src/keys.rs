//! Mapping from `rdev` keys to display labels.

use rdev::Key;
use std::borrow::Cow;

/// The modifier families tracked when building key combinations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModKind {
    Ctrl,
    Shift,
    Alt,
    Meta,
}

impl ModKind {
    /// All modifiers, in the order they appear in a combination label.
    pub const ALL: [ModKind; 4] = [ModKind::Ctrl, ModKind::Shift, ModKind::Alt, ModKind::Meta];

    /// Label used for this modifier inside a combination such as `Ctrl + S`.
    pub fn label(self) -> &'static str {
        match self {
            ModKind::Ctrl => "Ctrl",
            ModKind::Shift => "Shift",
            ModKind::Alt => "Alt",
            ModKind::Meta => "Win",
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
        Key::Alt | Key::AltGr => Some(ModKind::Alt),
        Key::MetaLeft | Key::MetaRight => Some(ModKind::Meta),
        _ => None,
    }
}

/// Returns the label shown on screen for `key`.
pub fn key_label(key: Key) -> Cow<'static, str> {
    let label = match key {
        Key::Alt => "Alt",
        Key::AltGr => "AltGr",
        Key::Backspace => "Backspace",
        Key::CapsLock => "CapsLock",
        Key::ControlLeft | Key::ControlRight => "Ctrl",
        Key::Delete => "Delete",
        Key::DownArrow => "Down",
        Key::End => "End",
        Key::Escape => "Esc",
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
        Key::Home => "Home",
        Key::LeftArrow => "Left",
        Key::MetaLeft | Key::MetaRight => "Win",
        Key::PageDown => "PgDn",
        Key::PageUp => "PgUp",
        Key::Return => "Enter",
        Key::RightArrow => "Right",
        Key::ShiftLeft | Key::ShiftRight => "Shift",
        Key::Space => "Space",
        Key::Tab => "Tab",
        Key::UpArrow => "Up",
        Key::PrintScreen => "PrtSc",
        Key::ScrollLock => "ScrollLock",
        Key::Pause => "Pause",
        Key::Insert => "Insert",
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
        _ => return Cow::Owned(format!("{key:?}")),
    };
    Cow::Borrowed(label)
}

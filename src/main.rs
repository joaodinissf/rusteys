use eframe::egui;
use parking_lot::Mutex;
use rdev::{listen, Event, EventType, Key};
use std::collections::VecDeque;
use std::sync::Arc;
use std::thread;
use std::time::{Duration, Instant};

const MAX_KEYS: usize = 15;
const KEY_DISPLAY_DURATION: Duration = Duration::from_millis(4000);
const FADE_OUT_DURATION: Duration = Duration::from_millis(800);

// Window sizing (as fraction of screen width)
const WINDOW_WIDTH_FRACTION: f32 = 0.66; // 2/3rds of screen
const SCREEN_WIDTH: f32 = 1920.0;
const SCREEN_HEIGHT: f32 = 1080.0;

#[derive(Clone)]
struct KeyPress {
    text: String,
    timestamp: Instant,
}

struct KeyDisplayApp {
    key_presses: Arc<Mutex<VecDeque<KeyPress>>>,
}

#[derive(Default, Clone)]
struct Modifiers {
    ctrl: bool,
    shift: bool,
    alt: bool,
    meta: bool,
    // Track if modifier was used in a combination
    ctrl_used: bool,
    shift_used: bool,
    alt_used: bool,
    meta_used: bool,
}

impl Modifiers {
    fn format(&self) -> String {
        let mut parts = Vec::new();
        if self.ctrl {
            parts.push("Ctrl");
        }
        if self.shift {
            parts.push("Shift");
        }
        if self.alt {
            parts.push("Alt");
        }
        if self.meta {
            parts.push("Win");
        }
        if parts.is_empty() {
            String::new()
        } else {
            parts.join(" + ")
        }
    }
}

fn key_to_string(key: Key) -> String {
    match key {
        Key::Alt => "Alt".to_string(),
        Key::AltGr => "AltGr".to_string(),
        Key::Backspace => "Backspace".to_string(),
        Key::CapsLock => "CapsLock".to_string(),
        Key::ControlLeft | Key::ControlRight => "Ctrl".to_string(),
        Key::Delete => "Delete".to_string(),
        Key::DownArrow => "Down".to_string(),
        Key::End => "End".to_string(),
        Key::Escape => "Esc".to_string(),
        Key::F1 => "F1".to_string(),
        Key::F2 => "F2".to_string(),
        Key::F3 => "F3".to_string(),
        Key::F4 => "F4".to_string(),
        Key::F5 => "F5".to_string(),
        Key::F6 => "F6".to_string(),
        Key::F7 => "F7".to_string(),
        Key::F8 => "F8".to_string(),
        Key::F9 => "F9".to_string(),
        Key::F10 => "F10".to_string(),
        Key::F11 => "F11".to_string(),
        Key::F12 => "F12".to_string(),
        Key::Home => "Home".to_string(),
        Key::LeftArrow => "Left".to_string(),
        Key::MetaLeft | Key::MetaRight => "Win".to_string(),
        Key::PageDown => "PgDn".to_string(),
        Key::PageUp => "PgUp".to_string(),
        Key::Return => "Enter".to_string(),
        Key::RightArrow => "Right".to_string(),
        Key::ShiftLeft | Key::ShiftRight => "Shift".to_string(),
        Key::Space => "Space".to_string(),
        Key::Tab => "Tab".to_string(),
        Key::UpArrow => "Up".to_string(),
        Key::PrintScreen => "PrtSc".to_string(),
        Key::ScrollLock => "ScrollLock".to_string(),
        Key::Pause => "Pause".to_string(),
        Key::Insert => "Insert".to_string(),
        Key::Num0 => "0".to_string(),
        Key::Num1 => "1".to_string(),
        Key::Num2 => "2".to_string(),
        Key::Num3 => "3".to_string(),
        Key::Num4 => "4".to_string(),
        Key::Num5 => "5".to_string(),
        Key::Num6 => "6".to_string(),
        Key::Num7 => "7".to_string(),
        Key::Num8 => "8".to_string(),
        Key::Num9 => "9".to_string(),
        Key::KeyA => "A".to_string(),
        Key::KeyB => "B".to_string(),
        Key::KeyC => "C".to_string(),
        Key::KeyD => "D".to_string(),
        Key::KeyE => "E".to_string(),
        Key::KeyF => "F".to_string(),
        Key::KeyG => "G".to_string(),
        Key::KeyH => "H".to_string(),
        Key::KeyI => "I".to_string(),
        Key::KeyJ => "J".to_string(),
        Key::KeyK => "K".to_string(),
        Key::KeyL => "L".to_string(),
        Key::KeyM => "M".to_string(),
        Key::KeyN => "N".to_string(),
        Key::KeyO => "O".to_string(),
        Key::KeyP => "P".to_string(),
        Key::KeyQ => "Q".to_string(),
        Key::KeyR => "R".to_string(),
        Key::KeyS => "S".to_string(),
        Key::KeyT => "T".to_string(),
        Key::KeyU => "U".to_string(),
        Key::KeyV => "V".to_string(),
        Key::KeyW => "W".to_string(),
        Key::KeyX => "X".to_string(),
        Key::KeyY => "Y".to_string(),
        Key::KeyZ => "Z".to_string(),
        _ => format!("{:?}", key),
    }
}

impl KeyDisplayApp {
    fn new(key_presses: Arc<Mutex<VecDeque<KeyPress>>>) -> Self {
        Self { key_presses }
    }
}

impl eframe::App for KeyDisplayApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let now = Instant::now();

        // Check if window is focused and Escape is pressed
        if ctx.input(|i| i.focused && i.key_pressed(egui::Key::Escape)) {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
        }

        // Clean up old key presses
        {
            let mut key_presses = self.key_presses.lock();
            key_presses.retain(|kp| {
                now.duration_since(kp.timestamp) < KEY_DISPLAY_DURATION + FADE_OUT_DURATION
            });
        }

        let key_presses = self.key_presses.lock().clone();

        // Always show the window with constant background opacity
        // No longer hide when empty to avoid jarring transitions
        
        // Request repaint for smooth animations
        ctx.request_repaint();

        // Check if window is focused for visual indication
        let is_focused = ctx.input(|i| i.focused);

        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(egui::Color32::from_rgba_unmultiplied(
                        35,
                        35,
                        35,
                        127, // Constant 50% opacity (255 * 0.5)
                    ))
                    .inner_margin(egui::Margin::same(20))
                    .corner_radius(egui::CornerRadius::same(12))
                    .shadow(egui::epaint::Shadow {
                        offset: [0, 4],
                        blur: 16,
                        spread: 0,
                        color: egui::Color32::from_rgba_unmultiplied(0, 0, 0, 115), // Constant shadow opacity
                    }),
            )
            .show(ctx, |ui| {
                // Draw focus indicator outline on top of everything
                if is_focused {
                    let rect = ui.max_rect().shrink(1.5); // Shrink slightly to ensure outline is visible
                    ui.painter().rect_stroke(
                        rect,
                        egui::CornerRadius::same(12),
                        egui::Stroke::new(3.0, egui::Color32::from_rgb(100, 150, 255)),
                        egui::StrokeKind::Outside,
                    );
                }
                
                // Make the window draggable by detecting drag on the background
                let response = ui.interact(ui.max_rect(), ui.id().with("drag_overlay"), egui::Sense::drag());
                if response.dragged() {
                    ctx.send_viewport_cmd(egui::ViewportCommand::StartDrag);
                }
                
                // Always allocate minimum height to prevent layout shift when empty
                ui.set_min_height(60.0);
                
                // Use a scroll area that auto-scrolls to the right (most recent keys)
                egui::ScrollArea::horizontal()
                    .auto_shrink(false)
                    .stick_to_right(true)
                    .show(ui, |ui| {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.spacing_mut().item_spacing.x = 12.0;

                            for key_press in key_presses.iter().rev() {
                                let age = now.duration_since(key_press.timestamp);
                        
                        // Calculate fade for individual keys
                        let alpha = if age > KEY_DISPLAY_DURATION {
                            let fade_progress = (age.as_millis() - KEY_DISPLAY_DURATION.as_millis())
                                as f32
                                / FADE_OUT_DURATION.as_millis() as f32;
                            ((1.0 - fade_progress.min(1.0)) * 255.0) as u8
                        } else {
                            255
                        };

                        // Scale effect: slightly larger when first pressed
                        let scale = if age.as_millis() < 100 {
                            1.0 + (1.0 - age.as_millis() as f32 / 100.0) * 0.2
                        } else {
                            1.0
                        };

                        let font_size = 28.0 * scale;

                        // Use a Frame to draw background behind the text
                        egui::Frame::new()
                            .fill(egui::Color32::from_rgba_unmultiplied(70, 75, 85, alpha))
                            .corner_radius(egui::CornerRadius::same(6))
                            .stroke(egui::Stroke::new(
                                1.5,
                                egui::Color32::from_rgba_unmultiplied(140, 150, 170, alpha),
                            ))
                            .inner_margin(egui::Margin::symmetric(12, 8))
                            .show(ui, |ui| {
                                // Draw key text on top of the frame
                                let text = egui::RichText::new(&key_press.text)
                                    .size(font_size)
                                    .strong()
                                    .color(egui::Color32::from_rgba_unmultiplied(255, 255, 255, alpha));

                                ui.add(egui::Label::new(text).wrap_mode(egui::TextWrapMode::Extend));
                            });
                        }
                    });
                });
        });
    }
}

fn main() -> Result<(), eframe::Error> {
    let key_presses = Arc::new(Mutex::new(VecDeque::new()));
    let modifiers = Arc::new(Mutex::new(Modifiers::default()));

    let key_presses_clone = Arc::clone(&key_presses);
    let modifiers_clone = Arc::clone(&modifiers);

    // Spawn keyboard listener thread
    thread::spawn(move || {
        let callback = move |event: Event| {
            match event.event_type {
                EventType::KeyPress(key) => {
                    let mut mods = modifiers_clone.lock();

                    // Update modifier state
                    match key {
                        Key::ControlLeft | Key::ControlRight => mods.ctrl = true,
                        Key::ShiftLeft | Key::ShiftRight => mods.shift = true,
                        Key::Alt | Key::AltGr => mods.alt = true,
                        Key::MetaLeft | Key::MetaRight => mods.meta = true,
                        _ => {}
                    }

                    // Build the key combination string
                    let mut key_text = String::new();
                    
                    // Add modifiers if present and this isn't a modifier key itself
                    let is_modifier = matches!(
                        key,
                        Key::ControlLeft
                            | Key::ControlRight
                            | Key::ShiftLeft
                            | Key::ShiftRight
                            | Key::Alt
                            | Key::AltGr
                            | Key::MetaLeft
                            | Key::MetaRight
                    );

                    if !is_modifier {
                        let mod_str = mods.format();
                        if !mod_str.is_empty() {
                            key_text.push_str(&mod_str);
                            key_text.push_str(" + ");
                            
                            // Mark modifiers as used in combination
                            if mods.ctrl { mods.ctrl_used = true; }
                            if mods.shift { mods.shift_used = true; }
                            if mods.alt { mods.alt_used = true; }
                            if mods.meta { mods.meta_used = true; }
                        }
                    }

                    key_text.push_str(&key_to_string(key));

                    let mut key_presses = key_presses_clone.lock();
                    
                    // Only add non-modifier keys
                    if !is_modifier {
                        key_presses.push_back(KeyPress {
                            text: key_text,
                            timestamp: Instant::now(),
                        });

                        // Keep only the most recent keys
                        while key_presses.len() > MAX_KEYS {
                            key_presses.pop_front();
                        }
                    }
                }
                EventType::KeyRelease(key) => {
                    let mut mods = modifiers_clone.lock();
                    
                    // Check if this modifier was used in a combination
                    let is_modifier = matches!(
                        key,
                        Key::ControlLeft
                            | Key::ControlRight
                            | Key::ShiftLeft
                            | Key::ShiftRight
                            | Key::Alt
                            | Key::AltGr
                            | Key::MetaLeft
                            | Key::MetaRight
                    );
                    
                    // Show standalone modifier only if it wasn't used in combination
                    if is_modifier {
                        let was_used = match key {
                            Key::ControlLeft | Key::ControlRight => mods.ctrl_used,
                            Key::ShiftLeft | Key::ShiftRight => mods.shift_used,
                            Key::Alt | Key::AltGr => mods.alt_used,
                            Key::MetaLeft | Key::MetaRight => mods.meta_used,
                            _ => false,
                        };
                        
                        if !was_used {
                            // Show standalone modifier key
                            let mut key_presses = key_presses_clone.lock();
                            key_presses.push_back(KeyPress {
                                text: key_to_string(key),
                                timestamp: Instant::now(),
                            });
                            
                            while key_presses.len() > MAX_KEYS {
                                key_presses.pop_front();
                            }
                        }
                    }
                    
                    // Update modifier state on release
                    match key {
                        Key::ControlLeft | Key::ControlRight => {
                            mods.ctrl = false;
                            mods.ctrl_used = false;
                        }
                        Key::ShiftLeft | Key::ShiftRight => {
                            mods.shift = false;
                            mods.shift_used = false;
                        }
                        Key::Alt | Key::AltGr => {
                            mods.alt = false;
                            mods.alt_used = false;
                        }
                        Key::MetaLeft | Key::MetaRight => {
                            mods.meta = false;
                            mods.meta_used = false;
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        };

        if let Err(error) = listen(callback) {
            eprintln!("Error listening to keyboard events: {:?}", error);
        }
    });

    let window_width = SCREEN_WIDTH * WINDOW_WIDTH_FRACTION;
    let window_x = (SCREEN_WIDTH - window_width) / 2.0;
    let window_y = SCREEN_HEIGHT * 0.85;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([window_width, 100.0])
            .with_position([window_x, window_y])
            .with_decorations(false)
            .with_transparent(true)
            .with_always_on_top()
            .with_resizable(false)
            .with_mouse_passthrough(false),
        ..Default::default()
    };

    eframe::run_native(
        "Key Display Overlay",
        options,
        Box::new(|cc| {
            // Make visuals as transparent as egui allows; true per-pixel window transparency
            // relies on with_transparent(true) above. The platform may still composite a base layer.
            let mut style = (*cc.egui_ctx.style()).clone();
            style.visuals.window_fill = egui::Color32::TRANSPARENT;
            style.visuals.panel_fill = egui::Color32::TRANSPARENT;
            style.visuals.window_stroke = egui::Stroke::NONE;
            style.visuals.extreme_bg_color = egui::Color32::TRANSPARENT;
            style.visuals.faint_bg_color = egui::Color32::TRANSPARENT;
            cc.egui_ctx.set_style(style);

            // NOTE: egui 0.33 does not expose a ViewportCommand to change the clear color.
            // The wgpu/gl backend clears to a dark default; to fully eliminate it you'd need
            // a custom clear via a fork or use a transparent Area layered on a *smaller* window
            // sized to content. Future improvement: dynamically shrink window when empty.

            Ok(Box::new(KeyDisplayApp::new(key_presses)))
        }),
    )
}

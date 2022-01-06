use std::collections::HashMap;
use wezterm_input_types::PhysKeyCode;
use winapi::shared::minwindef::WPARAM;
use winapi::um::winuser::*;

/*
// Determine the raw, underlying key event
raw = match wparam as i32 {
    0 => None,
    VK_CANCEL => Some(KeyCode::Cancel),
    VK_BACK => Some(KeyCode::Char('\u{8}')),
    VK_TAB => Some(KeyCode::Char('\t')),
    VK_CLEAR => Some(KeyCode::Clear),
    VK_RETURN => Some(KeyCode::Char('\r')),
    VK_SHIFT => Some(KeyCode::Shift),
    VK_CONTROL => Some(KeyCode::Control),
    VK_MENU => Some(KeyCode::Alt),
    VK_PAUSE => Some(KeyCode::Pause),
    VK_CAPITAL => Some(KeyCode::CapsLock),
    VK_ESCAPE => Some(KeyCode::Char('\u{1b}')),
    VK_SPACE => Some(KeyCode::Char(' ')),
    VK_PRIOR => Some(KeyCode::PageUp),
    VK_NEXT => Some(KeyCode::PageDown),
    VK_END => Some(KeyCode::End),
    VK_HOME => Some(KeyCode::Home),
    VK_LEFT => Some(KeyCode::LeftArrow),
    VK_UP => Some(KeyCode::UpArrow),
    VK_RIGHT => Some(KeyCode::RightArrow),
    VK_DOWN => Some(KeyCode::DownArrow),
    VK_SELECT => Some(KeyCode::Select),
    VK_PRINT => Some(KeyCode::Print),
    VK_EXECUTE => Some(KeyCode::Execute),
    VK_SNAPSHOT => Some(KeyCode::PrintScreen),
    VK_INSERT => Some(KeyCode::Insert),
    VK_DELETE => Some(KeyCode::Char('\u{7f}')),
    VK_HELP => Some(KeyCode::Help),
    // 0-9 happen to overlap with ascii
    i @ 0x30..=0x39 => Some(KeyCode::Char(i as u8 as char)),
    // a-z also overlap with ascii
    i @ 0x41..=0x5a => Some(KeyCode::Char((i as u8 as char).to_ascii_lowercase())),
    VK_LWIN => Some(KeyCode::LeftWindows),
    VK_RWIN => Some(KeyCode::RightWindows),
    VK_APPS => Some(KeyCode::Applications),
    VK_SLEEP => Some(KeyCode::Sleep),
    i @ VK_NUMPAD0..=VK_NUMPAD9 => Some(KeyCode::Numpad((i - VK_NUMPAD0) as u8)),
    VK_MULTIPLY => Some(KeyCode::Multiply),
    VK_ADD => Some(KeyCode::Add),
    VK_SEPARATOR => Some(KeyCode::Separator),
    VK_SUBTRACT => Some(KeyCode::Subtract),
    VK_DECIMAL => Some(KeyCode::Decimal),
    VK_DIVIDE => Some(KeyCode::Divide),
    i @ VK_F1..=VK_F24 => Some(KeyCode::Function((1 + i - VK_F1) as u8)),
    VK_NUMLOCK => Some(KeyCode::NumLock),
    VK_SCROLL => Some(KeyCode::ScrollLock),
    VK_LSHIFT => Some(KeyCode::LeftShift),
    VK_RSHIFT => Some(KeyCode::RightShift),
    VK_LCONTROL => Some(KeyCode::LeftControl),
    VK_RCONTROL => Some(KeyCode::RightControl),
    VK_LMENU => Some(KeyCode::LeftAlt),
    VK_RMENU => Some(KeyCode::RightAlt),
    VK_BROWSER_BACK => Some(KeyCode::BrowserBack),
    VK_BROWSER_FORWARD => Some(KeyCode::BrowserForward),
    VK_BROWSER_REFRESH => Some(KeyCode::BrowserRefresh),
    VK_BROWSER_STOP => Some(KeyCode::BrowserStop),
    VK_BROWSER_SEARCH => Some(KeyCode::BrowserSearch),
    VK_BROWSER_FAVORITES => Some(KeyCode::BrowserFavorites),
    VK_BROWSER_HOME => Some(KeyCode::BrowserHome),
    VK_VOLUME_MUTE => Some(KeyCode::VolumeMute),
    VK_VOLUME_DOWN => Some(KeyCode::VolumeDown),
    VK_VOLUME_UP => Some(KeyCode::VolumeUp),
    VK_MEDIA_NEXT_TRACK => Some(KeyCode::MediaNextTrack),
    VK_MEDIA_PREV_TRACK => Some(KeyCode::MediaPrevTrack),
    VK_MEDIA_STOP => Some(KeyCode::MediaStop),
    VK_MEDIA_PLAY_PAUSE => Some(KeyCode::MediaPlayPause),
    _ => None,
};
*/

fn build_map() -> HashMap<WPARAM, PhysKeyCode> {
    [
        (0x41, PhysKeyCode::A),
        (0x53, PhysKeyCode::S),
        (0x44, PhysKeyCode::D),
        (0x46, PhysKeyCode::F),
        (0x48, PhysKeyCode::H),
        (0x47, PhysKeyCode::G),
        (0x5a, PhysKeyCode::Z),
        (0x58, PhysKeyCode::X),
        (0x43, PhysKeyCode::C),
        (0x56, PhysKeyCode::V),
        (0x42, PhysKeyCode::B),
        (0x51, PhysKeyCode::Q),
        (0x57, PhysKeyCode::W),
        (0x45, PhysKeyCode::E),
        (0x52, PhysKeyCode::R),
        (0x59, PhysKeyCode::Y),
        (0x54, PhysKeyCode::T),
        (0x31, PhysKeyCode::K1),
        (0x32, PhysKeyCode::K2),
        (0x33, PhysKeyCode::K3),
        (0x34, PhysKeyCode::K4),
        (0x36, PhysKeyCode::K6),
        (0x35, PhysKeyCode::K5),
        (VK_OEM_PLUS, PhysKeyCode::Equal),
        (0x39, PhysKeyCode::K9),
        (0x37, PhysKeyCode::K7),
        (VK_OEM_MINUS, PhysKeyCode::Minus),
        (0x38, PhysKeyCode::K8),
        (0x30, PhysKeyCode::K0),
        (VK_OEM_6, PhysKeyCode::RightBracket),
        (0x4f, PhysKeyCode::O),
        (0x55, PhysKeyCode::U),
        (VK_OEM_4, PhysKeyCode::LeftBracket),
        (0x49, PhysKeyCode::I),
        (0x50, PhysKeyCode::P),
        (0x4c, PhysKeyCode::L),
        (0x4a, PhysKeyCode::J),
        (VK_OEM_7, PhysKeyCode::Quote),
        (0x4b, PhysKeyCode::K),
        (VK_OEM_1, PhysKeyCode::Semicolon), // FIXME: OEM "can vary by keyboard!"
        (VK_OEM_5, PhysKeyCode::Backslash),
        (VK_OEM_COMMA, PhysKeyCode::Comma),
        (VK_OEM_2, PhysKeyCode::Slash),
        (0x4e, PhysKeyCode::N),
        (0x4d, PhysKeyCode::M),
        (VK_OEM_PERIOD, PhysKeyCode::Period),
        (VK_OEM_3, PhysKeyCode::Grave),
        (VK_DECIMAL, PhysKeyCode::KeypadDecimal),
        (VK_MULTIPLY, PhysKeyCode::KeypadMultiply),
        (VK_INSERT, PhysKeyCode::Insert),
        (VK_ADD, PhysKeyCode::KeypadAdd),
        (VK_CLEAR, PhysKeyCode::KeypadClear),
        (VK_DIVIDE, PhysKeyCode::KeypadDivide),
        //        (kVK_ANSI_KeypadEnter, PhysKeyCode::KeypadEnter),
        (VK_SUBTRACT, PhysKeyCode::KeypadSubtract),
        //        (kVK_ANSI_KeypadEquals, PhysKeyCode::KeypadEquals),
        (VK_NUMPAD0, PhysKeyCode::Keypad0),
        (VK_NUMPAD1, PhysKeyCode::Keypad1),
        (VK_NUMPAD2, PhysKeyCode::Keypad2),
        (VK_NUMPAD3, PhysKeyCode::Keypad3),
        (VK_NUMPAD4, PhysKeyCode::Keypad4),
        (VK_NUMPAD5, PhysKeyCode::Keypad5),
        (VK_NUMPAD6, PhysKeyCode::Keypad6),
        (VK_NUMPAD7, PhysKeyCode::Keypad7),
        (VK_NUMPAD8, PhysKeyCode::Keypad8),
        (VK_NUMPAD9, PhysKeyCode::Keypad9),
        (VK_RETURN, PhysKeyCode::Return),
        (VK_TAB, PhysKeyCode::Tab),
        (VK_SPACE, PhysKeyCode::Space),
        (VK_BACK, PhysKeyCode::Backspace),
        (VK_ESCAPE, PhysKeyCode::Escape),
        (VK_LWIN, PhysKeyCode::LeftWindows),
        (VK_SHIFT, PhysKeyCode::LeftShift),
        (VK_LSHIFT, PhysKeyCode::LeftShift),
        (VK_CAPITAL, PhysKeyCode::CapsLock),
        (VK_MENU, PhysKeyCode::LeftAlt),
        (VK_LMENU, PhysKeyCode::LeftAlt),
        (VK_LCONTROL, PhysKeyCode::LeftControl),
        (VK_RWIN, PhysKeyCode::RightWindows),
        (VK_RSHIFT, PhysKeyCode::RightShift),
        (VK_RMENU, PhysKeyCode::RightAlt),
        (VK_RCONTROL, PhysKeyCode::RightControl),
        //        (VK_Function, PhysKeyCode::Function),
        (VK_F17, PhysKeyCode::F17),
        (VK_VOLUME_UP, PhysKeyCode::VolumeUp),
        (VK_VOLUME_DOWN, PhysKeyCode::VolumeDown),
        (VK_VOLUME_MUTE, PhysKeyCode::VolumeMute),
        (VK_F18, PhysKeyCode::F18),
        (VK_F19, PhysKeyCode::F19),
        (VK_F20, PhysKeyCode::F20),
        (VK_F5, PhysKeyCode::F5),
        (VK_F6, PhysKeyCode::F6),
        (VK_F7, PhysKeyCode::F7),
        (VK_F3, PhysKeyCode::F3),
        (VK_F8, PhysKeyCode::F8),
        (VK_F9, PhysKeyCode::F9),
        (VK_F11, PhysKeyCode::F11),
        (VK_F13, PhysKeyCode::F13),
        (VK_F16, PhysKeyCode::F16),
        (VK_F14, PhysKeyCode::F14),
        (VK_F10, PhysKeyCode::F10),
        (VK_F12, PhysKeyCode::F12),
        (VK_F15, PhysKeyCode::F15),
        (VK_HELP, PhysKeyCode::Help),
        (VK_HOME, PhysKeyCode::Home),
        (VK_PRIOR, PhysKeyCode::PageUp),
        (VK_DELETE, PhysKeyCode::Delete),
        (VK_F4, PhysKeyCode::F4),
        (VK_END, PhysKeyCode::End),
        (VK_F2, PhysKeyCode::F2),
        (VK_NEXT, PhysKeyCode::PageDown),
        (VK_F1, PhysKeyCode::F1),
        (VK_LEFT, PhysKeyCode::LeftArrow),
        (VK_RIGHT, PhysKeyCode::RightArrow),
        (VK_DOWN, PhysKeyCode::DownArrow),
        (VK_UP, PhysKeyCode::UpArrow),
    ]
    .iter()
    .map(|&(k, v)| (k as WPARAM, v))
    .collect()
}

lazy_static::lazy_static! {
    static ref MAP: HashMap<WPARAM, PhysKeyCode> = build_map();
}

pub fn vkey_to_phys(vkey: WPARAM) -> Option<PhysKeyCode> {
    MAP.get(&vkey).copied()
}

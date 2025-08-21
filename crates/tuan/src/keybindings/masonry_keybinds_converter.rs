use keybinds::{Key as KeybindsKey, Mods};
use masonry::core::keyboard::{
    Key as MasonryKey, Modifiers as MasonryModifiers, NamedKey as MasonryNamedKey,
};

pub(crate) fn masonry_key_to_keybinds_key(key: &MasonryKey) -> KeybindsKey {
    match key {
        MasonryKey::Named(named) => match named {
            MasonryNamedKey::ArrowUp => KeybindsKey::Up,
            MasonryNamedKey::ArrowRight => KeybindsKey::Right,
            MasonryNamedKey::ArrowDown => KeybindsKey::Down,
            MasonryNamedKey::ArrowLeft => KeybindsKey::Left,
            MasonryNamedKey::Enter => KeybindsKey::Enter,
            MasonryNamedKey::Backspace => KeybindsKey::Backspace,
            MasonryNamedKey::Delete => KeybindsKey::Delete,
            MasonryNamedKey::Home => KeybindsKey::Home,
            MasonryNamedKey::End => KeybindsKey::End,
            MasonryNamedKey::PageUp => KeybindsKey::PageUp,
            MasonryNamedKey::PageDown => KeybindsKey::PageDown,
            MasonryNamedKey::Escape => KeybindsKey::Esc,
            MasonryNamedKey::Tab => KeybindsKey::Tab,
            MasonryNamedKey::Insert => KeybindsKey::Insert,
            MasonryNamedKey::Copy => KeybindsKey::Copy,
            MasonryNamedKey::Cut => KeybindsKey::Cut,
            MasonryNamedKey::Paste => KeybindsKey::Paste,
            MasonryNamedKey::Clear => KeybindsKey::Clear,
            MasonryNamedKey::Undo => KeybindsKey::Undo,
            MasonryNamedKey::Redo => KeybindsKey::Redo,
            MasonryNamedKey::Help => KeybindsKey::Help,
            MasonryNamedKey::ZoomIn => KeybindsKey::ZoomIn,
            MasonryNamedKey::ZoomOut => KeybindsKey::ZoomOut,
            MasonryNamedKey::ZoomToggle => KeybindsKey::ZoomToggle,
            MasonryNamedKey::ScrollLock => KeybindsKey::ScrollLock,
            MasonryNamedKey::NumLock => KeybindsKey::NumLock,
            MasonryNamedKey::PrintScreen => KeybindsKey::PrintScreen,
            MasonryNamedKey::ContextMenu => KeybindsKey::Menu,
            MasonryNamedKey::MediaPlay => KeybindsKey::Play,
            MasonryNamedKey::MediaPause => KeybindsKey::Pause,
            MasonryNamedKey::MediaPlayPause => KeybindsKey::PlayPause,
            MasonryNamedKey::MediaStop => KeybindsKey::Stop,
            MasonryNamedKey::MediaRewind => KeybindsKey::Rewind,
            MasonryNamedKey::MediaTrackNext => KeybindsKey::NextTrack,
            MasonryNamedKey::MediaTrackPrevious => KeybindsKey::PrevTrack,
            MasonryNamedKey::AudioVolumeUp => KeybindsKey::VolumeUp,
            MasonryNamedKey::AudioVolumeDown => KeybindsKey::VolumeDown,
            MasonryNamedKey::AudioVolumeMute => KeybindsKey::Mute,
            MasonryNamedKey::F1 => KeybindsKey::F1,
            MasonryNamedKey::F2 => KeybindsKey::F2,
            MasonryNamedKey::F3 => KeybindsKey::F3,
            MasonryNamedKey::F4 => KeybindsKey::F4,
            MasonryNamedKey::F5 => KeybindsKey::F5,
            MasonryNamedKey::F6 => KeybindsKey::F6,
            MasonryNamedKey::F7 => KeybindsKey::F7,
            MasonryNamedKey::F8 => KeybindsKey::F8,
            MasonryNamedKey::F9 => KeybindsKey::F9,
            MasonryNamedKey::F10 => KeybindsKey::F10,
            MasonryNamedKey::F11 => KeybindsKey::F11,
            MasonryNamedKey::F12 => KeybindsKey::F12,
            MasonryNamedKey::F13 => KeybindsKey::F13,
            MasonryNamedKey::F14 => KeybindsKey::F14,
            MasonryNamedKey::F15 => KeybindsKey::F15,
            MasonryNamedKey::F16 => KeybindsKey::F16,
            MasonryNamedKey::F17 => KeybindsKey::F17,
            MasonryNamedKey::F18 => KeybindsKey::F18,
            MasonryNamedKey::F19 => KeybindsKey::F19,
            MasonryNamedKey::F20 => KeybindsKey::F20,
            MasonryNamedKey::F21 => KeybindsKey::F21,
            MasonryNamedKey::F22 => KeybindsKey::F22,
            MasonryNamedKey::F23 => KeybindsKey::F23,
            MasonryNamedKey::F24 => KeybindsKey::F24,
            MasonryNamedKey::F25 => KeybindsKey::F25,
            MasonryNamedKey::F26 => KeybindsKey::F26,
            MasonryNamedKey::F27 => KeybindsKey::F27,
            MasonryNamedKey::F28 => KeybindsKey::F28,
            MasonryNamedKey::F29 => KeybindsKey::F29,
            MasonryNamedKey::F30 => KeybindsKey::F30,
            MasonryNamedKey::F31 => KeybindsKey::F31,
            MasonryNamedKey::F32 => KeybindsKey::F32,
            MasonryNamedKey::F33 => KeybindsKey::F33,
            MasonryNamedKey::F34 => KeybindsKey::F34,
            MasonryNamedKey::F35 => KeybindsKey::F35,
            MasonryNamedKey::Alt
            | MasonryNamedKey::Control
            | MasonryNamedKey::Shift
            | MasonryNamedKey::Super
            | MasonryNamedKey::Hyper
            | MasonryNamedKey::Meta
            | MasonryNamedKey::Symbol => KeybindsKey::Ignored,
            _ => KeybindsKey::Unidentified,
        },
        MasonryKey::Character(s) => {
            let mut chars = s.chars();
            if let (Some(c), None) = (chars.next(), chars.next()) {
                KeybindsKey::Char(c)
            } else {
                KeybindsKey::Unidentified
            }
        }
        _ => KeybindsKey::Unidentified,
    }
}

pub(crate) fn masonry_modifier_to_keybinds_mods(state: &MasonryModifiers) -> Mods {
    let mut mods = Mods::NONE;
    if state.contains(MasonryModifiers::CONTROL) {
        mods |= Mods::CTRL;
    }
    if state.contains(MasonryModifiers::ALT) {
        mods |= Mods::ALT;
    }
    if state.contains(MasonryModifiers::META) {
        mods |= Mods::SUPER;
    }
    if state.contains(MasonryModifiers::SHIFT) {
        mods |= Mods::SHIFT;
    }
    mods
}

use keybinds::{KeyInput, Key as KeybindsKey};
use masonry::core::{Modifiers, keyboard::Key};

use crate::keybindings;

impl super::EditorState {
    pub fn press_key(&mut self, key: Key, modifiers: Modifiers) {
        let key = keybindings::masonry_keybinds_converter::masonry_key_to_keybinds_key(&key);
        let mods =
            keybindings::masonry_keybinds_converter::masonry_modifier_to_keybinds_mods(&modifiers);

        let mut state = self.clone();
        let action = state
            .keybindings
            .keybinds
            .dispatch(KeyInput::new(key, mods));

        tracing::debug!("Key pressed: {:?} with modifiers: {:?}", key, mods);
        tracing::debug!("Action dispatched: {:?}", action);

        if let Some(action) = action {
            self.handle_action(&action);
        } else if modifiers == Modifiers::empty() && let KeybindsKey::Char(c) = key {
            tracing::debug!("Insert character: {:?}", c);
            self.insert_character(c);
        }
    }
}

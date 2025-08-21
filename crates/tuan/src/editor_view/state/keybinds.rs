use crate::editor_view::state::masonry_keybinds_converter::{
    masonry_key_to_keybinds_key, masonry_modifier_to_keybinds_mods,
};
use keybinds::KeyInput;
use masonry::core::{Modifiers, keyboard::Key};
use serde::Deserialize;
use std::fs;
use super::action::EditorAction;

#[derive(Clone)]
pub(super) struct Keybinds {
    keybinds: keybinds::Keybinds<EditorAction>,
}

impl Keybinds {
    pub fn new() -> Result<Self, keybinds::Error> {
        let mut keybinds = keybinds::Keybinds::default();

        let config_path = "/Users/arthurfontaine/Developer/code/github.com/arthur-fontaine/tuan/crates/tuan/assets/keybind_config.json";
        let config_data =
            fs::read_to_string(config_path).expect("Failed to read keybind_config.json");
        let config = serde_json::from_str::<KeybindsConfig>(&config_data)
            .expect("Failed to parse keybind_config.json");

        for keybind in config.keybinds {
            keybinds.bind(&keybind.key, keybind.action)?;
        }

        Ok(Self { keybinds })
    }
}

impl super::EditorState {
    pub fn press_key(&mut self, key: Key, modifiers: Modifiers) {
        let key = masonry_key_to_keybinds_key(&key);
        let mods = masonry_modifier_to_keybinds_mods(&modifiers);

        let mut state = self.clone();
        let action = state.keybinds.keybinds.dispatch(KeyInput::new(key, mods));

        println!("Key pressed: {:?} with modifiers: {:?}", key, mods);
        println!("Action dispatched: {:?}", action);

        if let Some(action) = action {
            self.handle_action(&action);
        }
    }
}

#[derive(Debug, Deserialize)]
struct KeybindsConfig {
    pub keybinds: Vec<KeybindConfig>,
}

#[derive(Debug, Deserialize)]
struct KeybindConfig {
    pub key: String,
    pub action: EditorAction,
}

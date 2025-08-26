use crate::{editor_view::EditorAction, keybindings::keybindings_config::KeybindingsConfig};
use std::fs;

#[derive(Clone)]
pub struct Keybindings {
    pub(crate) keybinds: keybinds::Keybinds<EditorAction>,
}

impl Keybindings {
    pub fn new() -> Result<Self, keybinds::Error> {
        let mut keybinds = keybinds::Keybinds::default();

        let config_path = "/Users/arthur-fontaine/Developer/code/github.com/tuan-ide/tuan/crates/tuan/assets/keybind_config.json";
        let config_data =
            fs::read_to_string(config_path).expect("Failed to read keybind_config.json");
        let config = serde_json::from_str::<KeybindingsConfig>(&config_data)
            .expect("Failed to parse keybind_config.json");

        for keybind in config.keybindings {
            keybinds.bind(&keybind.key, keybind.action)?;
        }

        Ok(Self { keybinds })
    }
}

use crate::editor_view::EditorAction;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct KeybindingsConfig {
    pub(super) keybindings: Vec<KeybindingConfig>,
}

#[derive(Debug, Deserialize)]
pub(super) struct KeybindingConfig {
    pub(super) key: String,
    pub(super) action: EditorAction,
}

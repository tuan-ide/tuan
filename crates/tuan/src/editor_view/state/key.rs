use std::collections::HashSet;

use masonry::core::keyboard::{Key, NamedKey};

impl super::EditorState {
    pub fn press_key(&mut self, key: Key) {
        self.keys_pressed.insert(key);
    }

    pub fn release_key(&mut self, key: Key) {
        self.keys_pressed.remove(&key);
    }

    pub fn handle_keybind(&mut self) -> Option<()> {
        if self.keys_pressed == HashSet::from([Key::Named(NamedKey::ArrowLeft)]) {
            let focused_path = self.focused_document_path.clone()?;
            let cursors = self.document_cursors.get_mut(&focused_path)?;

            for cursor in cursors {
                cursor.move_left(1);
            }
        } else {
            return None;
        }
        None
    }
}

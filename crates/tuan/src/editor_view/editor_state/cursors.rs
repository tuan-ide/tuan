use crate::editor_view::paint::cursor;
use std::path::PathBuf;

impl super::EditorState {
    pub fn get_document_cursors(&self, path: &PathBuf) -> Option<Vec<cursor::Cursor>> {
        self.document_cursors.get(path).cloned()
    }

    pub fn add_cursor(&mut self, path: PathBuf, position: &(usize, usize)) {
        self.document_cursors
            .entry(path)
            .or_insert_with(Vec::new)
            .push(cursor::Cursor::new(
                position.0,
                position.1,
                self.documents.clone(),
                self.focused_document_path.clone(),
                self.config.clone(),
            ));
    }

    pub fn clear_cursors(&mut self, path: PathBuf) {
        if let Some(cursors) = self.document_cursors.get_mut(&path) {
            cursors.clear();
        }
    }

    pub fn tick_cursors(&mut self) {
        if let Some(focused_path) = &self.focused_document_path {
            if let Some(cursors) = self.document_cursors.get_mut(focused_path) {
                for cursor in cursors {
                    cursor.tick();
                }
            }
        }
    }
}

use crate::editor_view::paint::cursor::Cursor;

impl super::EditorState {
    pub fn insert_character(&mut self, ch: char) {
        let focused_document_path = match self.focused_document_path.clone() {
            Some(p) => p,
            None => return,
        };

        let mut documents = self.documents.lock().unwrap();
        let document = match documents.get_mut(&focused_document_path) {
            Some(d) => d,
            None => return,
        };

        let cursors: Vec<Cursor> = self
            .document_cursors
            .get(&focused_document_path)
            .cloned()
            .unwrap_or_default();

        let s = ch.to_string();
        for cursor in &cursors {
            document.insert_character_at(cursor, &s);
        }

        if let Some(cursors) = self.document_cursors.get_mut(&focused_document_path) {
            for c in cursors {
                c.move_x_at(c.column + 1);
            }
        }
    }

    pub fn delete_character(&mut self) {
        let focused_document_path = match self.focused_document_path.clone() {
            Some(p) => p,
            None => return,
        };

        let mut documents = self.documents.lock().unwrap();
        let document = match documents.get_mut(&focused_document_path) {
            Some(d) => d,
            None => return,
        };

        let cursors: Vec<Cursor> = self
            .document_cursors
            .get(&focused_document_path)
            .cloned()
            .unwrap_or_default();

        for cursor in &cursors {
            document.delete_character_at(cursor);
        }

        if let Some(cursors) = self.document_cursors.get_mut(&focused_document_path) {
            for c in cursors {
                c.move_x_at(c.column - 1);
            }
        }
    }
}

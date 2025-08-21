impl super::EditorState {
    pub fn insert_character(&mut self, ch: char) {
        let focused_document_path = match self.focused_document_path.clone() {
            Some(p) => p,
            None => return,
        };

        let cursors = self
            .document_cursors
            .get(&focused_document_path)
            .cloned()
            .unwrap_or_default();

        let positions = cursors
            .iter()
            .map(|c| c.get_cursor_offset())
            .collect::<Vec<_>>();

        let mut documents = self.documents.lock().unwrap();
        let document = match documents.get_mut(&focused_document_path) {
            Some(d) => d,
            None => return,
        };

        let s = ch.to_string();
        for position in positions {
            document.insert_character_at(position, &s);
        }

        drop(documents);

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

        let cursors = self
            .document_cursors
            .get(&focused_document_path)
            .cloned()
            .unwrap_or_default();

        let positions = cursors
            .iter()
            .map(|c| c.get_cursor_offset())
            .collect::<Vec<_>>();

        let mut documents = self.documents.lock().unwrap();
        let document = match documents.get_mut(&focused_document_path) {
            Some(d) => d,
            None => return,
        };

        for position in positions {
            document.delete_character_at(position);
        }

        drop(documents);

        if let Some(cursors) = self.document_cursors.get_mut(&focused_document_path) {
            for c in cursors {
                c.move_x_at(c.column - 1);
            }
        }
    }
}

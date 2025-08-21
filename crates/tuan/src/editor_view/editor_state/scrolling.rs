use std::path::PathBuf;

impl super::EditorState {
    pub fn scroll_document(&mut self, path: &PathBuf, delta: (f64, f64)) {
        let (x, y) = self
            .document_scrollings
            .entry(path.clone())
            .or_insert((0.0, 0.0));
        *x += delta.0;
        *y += delta.1;
    }

    pub fn get_document_scroll(&self, path: &PathBuf) -> Option<(f64, f64)> {
        self.document_scrollings.get(path).cloned()
    }
}

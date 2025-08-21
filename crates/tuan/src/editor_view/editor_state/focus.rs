use std::path::PathBuf;
use crate::document;

impl super::EditorState {
    pub fn focus_document(&mut self, path: PathBuf) {
        if self.documents.lock().unwrap().contains_key(&path) {
            self.focused_document_path = Some(path.clone());
            tracing::debug!("Focused document: {:?}", path);
        } else {
            tracing::debug!("Document not found: {:?}", path);
        }
    }

    pub fn get_focused_document(&self) -> Option<document::Document> {
        let focused_path = self.focused_document_path.clone();
        focused_path.and_then(|path| self.documents.lock().unwrap().get(&path).cloned())
    }
}

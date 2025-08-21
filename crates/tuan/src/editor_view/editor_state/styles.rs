use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex, RwLock},
};

use crate::document;

impl super::EditorState {
    pub(super) fn update_styles_with_syntax(
        documents: Arc<Mutex<HashMap<PathBuf, document::Document>>>,
        path: PathBuf,
    ) {
        std::thread::spawn(move || {
            let mut docs = documents.lock().unwrap();
            if let Some(document) = docs.get_mut(&path) {
                document.update_styles_with_syntax();
            } else {
                tracing::debug!("Document not found for path: {:?}", path);
            }
        });
    }
}

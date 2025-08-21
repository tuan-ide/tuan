use std::path::PathBuf;

use tuan_rpc::{buffer::BufferId, proxy::ProxyResponse};

use crate::document;

impl super::EditorState {
    pub fn open_file(&self, path: PathBuf) {
        if let Some(document) = self.documents.lock().unwrap().get(&path) {
            return;
        }

        self.proxy
            .proxy_rpc
            .new_buffer(BufferId::next(), path.clone(), {
                let documents = self.documents.clone();
                let config = self.config.clone();
                let path = path.clone();
                {
                    move |result| {
                        if let Ok(ProxyResponse::NewBufferResponse { content, read_only }) = result
                        {
                            let document =
                                document::Document::new(path.clone(), content, read_only, config);
                            documents.lock().unwrap().insert(path.clone(), document);

                            Self::update_styles_with_syntax(documents, path);
                        }
                    }
                }
            });
    }
}

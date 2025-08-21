use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use tuan_rpc::{buffer::BufferId, proxy::ProxyResponse};

use super::{EditorConfig, cursor};
use crate::{document, proxy, workspace};

pub struct EditorState {
    pub initialized: bool,
    proxy: proxy::ProxyData,
    pub config: Arc<EditorConfig>,
    pub documents: Arc<Mutex<HashMap<PathBuf, document::Document>>>,
    pub focused_document_path: Option<PathBuf>,
    pub document_scrollings: HashMap<PathBuf, (f64, f64)>,
    pub document_cursors: HashMap<PathBuf, Vec<cursor::Cursor>>,
}

impl EditorState {
    pub fn new(workspace_path: PathBuf, editor_config: Arc<EditorConfig>) -> Self {
        let workspace = Arc::new(workspace::LapceWorkspace {
            kind: workspace::LapceWorkspaceType::Local,
            path: Some(workspace_path),
            ..Default::default()
        });
        let (term_tx, term_rx) = crossbeam_channel::unbounded();

        let proxy = proxy::new_proxy(workspace, vec![], vec![], HashMap::new(), term_tx);

        // Spawn a thread to log all data received in proxy.notification_rx
        std::thread::spawn({
            let notification_rx = proxy.notification_rx.clone();
            move || {
                while let Ok(data) = notification_rx.recv() {
                    tracing::debug!("Received notification: {:?}", data);
                }
            }
        });

        std::thread::spawn({
            let core_rpc = proxy.core_rpc.clone();
            let core_rx = core_rpc.rx().clone();
            move || {
                while let Ok(notification) = core_rx.recv() {
                    match notification {
                        tuan_rpc::core::CoreRpc::Request(id, req) => {
                            tracing::debug!("CoreRpc::Request - id: {:?}, req: {:?}", id, req);
                        }
                        tuan_rpc::core::CoreRpc::Notification(notif) => {
                            tracing::debug!("CoreRpc::Notification - notif: {:?}", notif);
                        }
                        tuan_rpc::core::CoreRpc::Shutdown => {
                            tracing::debug!("CoreRpc::Shutdown");
                        }
                    }
                }
            }
        });

        std::thread::spawn({
            let proxy_rpc = proxy.proxy_rpc.clone();
            let proxy_rx = proxy_rpc.rx().clone();
            move || {
                // while let Ok(notification) = proxy_rx.recv() {
                //     match notification {
                //         tuan_rpc::proxy::ProxyRpc::Request(id, req) => {
                //             tracing::debug!("ProxyRpc::Request - id: {:?}, req: {:?}", id, req);
                //         }
                //         tuan_rpc::proxy::ProxyRpc::Notification(notif) => {
                //             tracing::debug!("ProxyRpc::Notification - notif: {:?}", notif);
                //         }
                //         tuan_rpc::proxy::ProxyRpc::Shutdown => {
                //             tracing::debug!("ProxyRpc::Shutdown");
                //         }
                //     }
                // }
            }
        });

        Self {
            initialized: true,
            proxy,
            config: editor_config,
            documents: Arc::new(Mutex::new(HashMap::new())),
            focused_document_path: None,
            document_scrollings: HashMap::new(),
            document_cursors: HashMap::new(),
        }
    }

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

    fn update_styles_with_syntax(
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

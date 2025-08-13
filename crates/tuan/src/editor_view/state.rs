use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use tuan_rpc::{buffer::BufferId, proxy::ProxyResponse};

use super::EditorConfig;
use crate::{document, proxy, workspace};

#[derive(Clone)]
pub struct EditorState {
    proxy: proxy::ProxyData,
    pub config: Arc<EditorConfig>,
    pub documents: Arc<Mutex<HashMap<PathBuf, document::Document>>>,
    pub focused_document_path: Option<PathBuf>,
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
                    println!("Received notification: {:?}", data);
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
                            println!("CoreRpc::Request - id: {:?}, req: {:?}", id, req);
                        }
                        tuan_rpc::core::CoreRpc::Notification(notif) => {
                            println!("CoreRpc::Notification - notif: {:?}", notif);
                        }
                        tuan_rpc::core::CoreRpc::Shutdown => {
                            println!("CoreRpc::Shutdown");
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
                //             println!("ProxyRpc::Request - id: {:?}, req: {:?}", id, req);
                //         }
                //         tuan_rpc::proxy::ProxyRpc::Notification(notif) => {
                //             println!("ProxyRpc::Notification - notif: {:?}", notif);
                //         }
                //         tuan_rpc::proxy::ProxyRpc::Shutdown => {
                //             println!("ProxyRpc::Shutdown");
                //         }
                //     }
                // }
            }
        });

        Self {
            proxy,
            config: editor_config,
            documents: Arc::new(Mutex::new(HashMap::new())),
            focused_document_path: None,
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
                println!("Document not found for path: {:?}", path);
            }
        });
    }

    pub fn focus_document(&mut self, path: PathBuf) {
        if self.documents.lock().unwrap().contains_key(&path) {
            self.focused_document_path = Some(path.clone());
            println!("Focused document: {:?}", path);
        } else {
            println!("Document not found: {:?}", path);
        }
    }

    pub fn get_focused_document(&self) -> Option<document::Document> {
        let focused_path = self.focused_document_path.clone();
        focused_path.and_then(|path| self.documents.lock().unwrap().get(&path).cloned())
    }
}

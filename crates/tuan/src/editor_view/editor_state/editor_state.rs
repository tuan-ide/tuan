use crate::keybindings::Keybindings;
use crate::{
    document,
    editor_view::{EditorConfig, paint::cursor},
    proxy, workspace,
};
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct EditorState {
    pub(super) proxy: proxy::ProxyData,
    pub config: Arc<EditorConfig>,
    pub documents: Arc<Mutex<HashMap<PathBuf, document::Document>>>,
    pub focused_document_path: Option<PathBuf>,
    pub document_scrollings: HashMap<PathBuf, (f64, f64)>,
    pub document_cursors: HashMap<PathBuf, Vec<cursor::Cursor>>,
    pub keybindings: Keybindings,
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

        let keybinds = Keybindings::new().expect("Failed to create keybinds");

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

        Self {
            proxy,
            keybindings: keybinds,
            config: editor_config,
            documents: Arc::new(Mutex::new(HashMap::new())),
            focused_document_path: None,
            document_scrollings: HashMap::new(),
            document_cursors: HashMap::new(),
        }
    }
}

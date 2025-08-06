use std::{collections::HashMap, path::PathBuf, sync::Arc};

use crate::{proxy, workspace};

#[derive(Clone)]
pub struct EditorState {
    proxy: proxy::ProxyData,
}

impl EditorState {
    pub fn new(workspace_path: PathBuf) -> Self {
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
                while let Ok(notification) = proxy_rx.recv() {
                    match notification {
                        tuan_rpc::proxy::ProxyRpc::Request(id, req) => {
                            println!("ProxyRpc::Request - id: {:?}, req: {:?}", id, req);
                        }
                        tuan_rpc::proxy::ProxyRpc::Notification(notif) => {
                            println!("ProxyRpc::Notification - notif: {:?}", notif);
                        }
                        tuan_rpc::proxy::ProxyRpc::Shutdown => {
                            println!("ProxyRpc::Shutdown");
                        }
                    }
                }
            }
        });

        Self { proxy }
    }
}

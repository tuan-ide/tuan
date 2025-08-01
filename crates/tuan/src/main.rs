// Copyright 2024 the Xilem Authors
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::sync::{Arc, Mutex};

use log::trace;
use tuan_rpc::Client;
use winit::error::EventLoopError;
use xilem::view::flex_row;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem};

use crate::globals::XI_PLUGIN_DIR;
use crate::globals::{EMBEDDED_PLUGINS, XI_CONFIG_DIR};
use crate::line_cache::LineCache;

mod globals;
mod line_cache;

struct AppState {
    line_cache: Arc<Mutex<LineCache>>,
}

impl AppState {
    fn new() -> Self {
        Self {
            line_cache: Arc::new(Mutex::new(LineCache::new())),
        }
    }
}

fn app_logic(data: &mut AppState) -> impl WidgetView<AppState> + use<> {
    let (client, receiver) = Client::new();

    let line_cache = data.line_cache.clone();

    std::thread::spawn(move || {
        loop {
            match receiver.recv() {
                Ok(operation) => match operation {
                    tuan_rpc::RpcOperations::Update(update) => {
                        trace!("Received update: {:?}", update);
                        line_cache.lock().unwrap().handle_xi_update(update);
                    }
                    _ => {
                        println!("Received operation: {:?}", operation);
                    }
                },
                Err(_) => {
                    println!("RPC receiver channel closed");
                    break;
                }
            }
        }
    });

    let start = std::time::Instant::now();
    let xi_plugin_dir = extract_embedded_plugins().unwrap();
    let duration = start.elapsed();
    println!("extract_embedded_plugins took {:?}", duration);
    let xi_config_dir = XI_CONFIG_DIR.clone();

    client.client_started(
        Some(&xi_config_dir.to_string()),
        Some(&xi_plugin_dir.to_string()),
    );

    // let path = String::from(
    //     "/Users/arthurfontaine/Developer/code/github.com/arthur-fontaine/tuan/docs/adr/0001-use-rust-core-language.md",
    // );
    // client.new_view(Some(&path), |_| {});

    flex_row(())
}

fn extract_embedded_plugins() -> Option<String> {
    let temp_dir = XI_PLUGIN_DIR.clone();
    let temp_dir = std::path::Path::new(&temp_dir);

    // Measure it on my MacBook Pro 14" 2021 M1 Pro:
    //  - without skipping, takes about 4.93ms to extract the plugins
    //  - with skipping if the directory exists, takes about 0.5ms (515.209µs)
    // TODO: extract directory after application start to avoid blocking the UI thread
    // we need to still extract the plugins to support updates
    if temp_dir.exists() {
        println!(
            "Embedded plugins already extracted to: {}",
            temp_dir.display()
        );
        return Some(temp_dir.to_string_lossy().to_string());
    }

    if let Err(err) = fs::create_dir_all(&temp_dir) {
        eprintln!("Failed to create directory {}: {}", temp_dir.display(), err);
        return None;
    }

    EMBEDDED_PLUGINS.extract(&temp_dir).ok()?;

    println!("Extracted embedded plugins to: {}", temp_dir.display());

    Some(temp_dir.to_string_lossy().to_string())
}

fn main() -> Result<(), EventLoopError> {
    let app = Xilem::new_simple(
        AppState::new(),
        app_logic,
        WindowOptions::new("Centered Flex"),
    );
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}

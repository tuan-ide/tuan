use std::sync::Arc;

use winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem, core::lens};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::editor_view::{EditorConfig, EditorState, editor_view};

mod document;
mod editor_view;
mod graph_view;
mod file;
mod globals;
mod proxy;
mod terminal;
mod workspace;
mod theme;
mod keybindings;
mod languages;

pub struct AppState {
    editor_state: EditorState,
}

impl AppState {
    fn new() -> Self {
        Self {
            editor_state: EditorState::new(
                "/Users/arthurfontaine/Developer/code/local/la-galerie-de-max/la-galerie-de-max copie".into(),
                Arc::new(EditorConfig::default()),
            ),
        }
    }
}

fn app_logic(data: &mut AppState) -> impl WidgetView<AppState> + use<> {
    lens(editor_view, |s: &mut AppState| &mut s.editor_state)
}

fn main() -> Result<(), EventLoopError> {
    // Initialize tracing with a filter to reduce debug noise
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| {
                    tracing_subscriber::EnvFilter::new("warn,tuan=debug")
                })
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Xilem::new_simple(AppState::new(), app_logic, WindowOptions::new("Tuan"));
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}

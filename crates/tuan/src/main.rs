use std::sync::Arc;

use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use winit::error::EventLoopError;
use xilem::{EventLoop, WidgetView, WindowOptions, Xilem, core::lens};

use crate::{
    editor_view::{EditorConfig, EditorState, editor_view},
    graph_view::{GraphFeeder as _, GraphState, graph_view},
    languages::typescript::TypescriptProject,
};

mod document;
mod editor_view;
mod file;
mod globals;
mod graph_view;
mod keybindings;
mod languages;
mod proxy;
mod terminal;
mod theme;
mod workspace;

pub struct AppState {
    editor_state: EditorState,
    graph_state: GraphState,
}

impl AppState {
    fn new() -> Self {
        let workspace_path =
            "/Users/arthur-fontaine/Developer/code/github.com/arthur-fontaine/agrume";

        let editor_state =
            EditorState::new(workspace_path.into(), Arc::new(EditorConfig::default()));

        let mut graph_state = GraphState::new(editor_state.config.clone());
        let typescript_project = TypescriptProject::new(workspace_path.into());
        typescript_project.feed_graph(&mut graph_state);

        let start = std::time::Instant::now();
        graph_state.relax_until_stable(1000, 0.25);
        let duration = start.elapsed();
        println!("Graph positioning took: {:?}", duration);

        Self {
            editor_state,
            graph_state,
        }
    }
}

fn app_logic(data: &mut AppState) -> impl WidgetView<AppState> + use<> {
    lens(graph_view, |s: &mut AppState| &mut s.graph_state)
}

fn main() -> Result<(), EventLoopError> {
    // Initialize tracing with a filter to reduce debug noise
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("warn,tuan=debug")),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = Xilem::new_simple(AppState::new(), app_logic, WindowOptions::new("Tuan"));
    app.run_in(EventLoop::with_user_event())?;
    Ok(())
}

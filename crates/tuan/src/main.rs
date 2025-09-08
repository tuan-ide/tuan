use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use winit::error::EventLoopError;
use xilem::{EventLoop, WindowOptions, Xilem};

use crate::app::{app_logic, AppState};

mod app;
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

use std::{sync::Arc, time::Duration};

use xilem::{
    WidgetView,
    core::{fork, lens, one_of::Either},
    tokio,
    view::task,
};

use crate::{
    editor_view::{EditorConfig, EditorState, editor_view},
    graph_view::{GraphFeeder as _, GraphState, graph_view},
    languages::typescript::TypescriptProject,
};

pub struct AppState {
    pub(super) editor_state: EditorState,
    pub(super) graph_state: GraphState,
}

impl AppState {
    pub(crate) fn new() -> Self {
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

pub(crate) fn app_logic(data: &mut AppState) -> impl WidgetView<AppState> + use<> {
    println!("app_logic");
    println!("focused: {:?}", data.editor_state.focused_document_path);
    fork(
        if data.editor_state.focused_document_path.is_some() {
            Either::A(editor_view(data))
        } else {
            Either::B(graph_view(data))
        },
        task(
            async move |proxy| {
                let mut interval = tokio::time::interval(Duration::from_millis(8)); // 8ms ~= 120fps
                loop {
                    interval.tick().await;
                    let Ok(()) = proxy.message(()) else {
                        break;
                    };
                }
            },
            |data: &mut AppState, ()| {},
        ),
    )
}

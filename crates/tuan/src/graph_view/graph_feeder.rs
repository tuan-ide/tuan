use crate::graph_view::graph_state::GraphState;

pub(crate) trait GraphFeeder {
    fn feed_graph(&self, graph: &mut GraphState);
}

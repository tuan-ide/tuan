use crate::graph_view::graph::Graph;

pub(crate) trait GraphFeeder {
    fn feed_graph(&self, graph: &mut Graph);
}

// &fdg_sim::petgraph::prelude::StableGraph<fdg_sim::Node<crate::file::File>, (), fdg_sim::petgraph::Undirected>

use fdg_sim::{
    Node,
    petgraph::{prelude::StableGraph, visit::EdgeRef},
};
use xilem::Vec2;

use super::camera::Camera;
use crate::file::File;

pub(super) struct EdgeDescriptor {
    pub(super) source_position: Vec2,
    pub(super) target_position: Vec2,
    pub(super) width: f64,
}

pub(super) struct NodeDescriptor {
    pub(super) file: File,
    pub(super) position: Vec2,
    pub(super) radius: f64,
}

pub(super) struct GraphDescriptor {
    pub(super) edges: Vec<EdgeDescriptor>,
    pub(super) nodes: Vec<NodeDescriptor>,
}

pub(super) fn create_graph_descriptor(
    graph: &StableGraph<Node<File>, (), fdg_sim::petgraph::Undirected>,
    camera: &Camera,
) -> GraphDescriptor {
    let mut edges = Vec::new();
    let mut nodes = Vec::new();

    for source_idx in graph.node_indices() {
        let source = &graph[source_idx];

        nodes.push(NodeDescriptor {
            file: source.data.clone(),
            position: camera.world_to_screen(source.location),
            radius: camera.value_to_screen(10.0),
        });

        for edge in graph.edges(source_idx) {
            let target_idx = edge.target();
            let target = &graph[target_idx];

            edges.push(EdgeDescriptor {
                source_position: camera.world_to_screen(source.location),
                target_position: camera.world_to_screen(target.location),
                width: camera.value_to_screen(2.0),
            });
        }
    }

    GraphDescriptor { edges, nodes }
}

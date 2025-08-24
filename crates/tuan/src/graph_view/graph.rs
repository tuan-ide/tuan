use std::collections::HashMap;

use crate::file::File;
use fdg_sim::{
    ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, petgraph::graph::NodeIndex,
};

pub struct Graph {
    graph: Simulation<File, ()>,
    path_to_node_index: HashMap<File, NodeIndex>,
}

impl Graph {
    pub fn new() -> Self {
        let graph =
            Simulation::from_graph(ForceGraph::default(), SimulationParameters::default());

        Self {
            graph,
            path_to_node_index: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, file: File) {
        let graph = self.graph.get_graph_mut();
        let node_index = graph.add_force_node(file.name.clone(), file.clone());
        self.path_to_node_index.insert(file, node_index);
    }

    pub fn add_relation(&mut self, a: File, b: File) {
        let a_index = self.path_to_node_index.get(&a);
        let b_index = self.path_to_node_index.get(&b);

        if let (Some(a_index), Some(b_index)) = (a_index, b_index) {
            let graph = self.graph.get_graph_mut();
            graph.add_edge(*a_index, *b_index, ());
        }
    }
}

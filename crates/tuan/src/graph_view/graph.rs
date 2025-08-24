use std::collections::HashMap;
use crate::file::File;
use fdg_sim::{
    glam::Vec3, petgraph::graph::NodeIndex, ForceGraph, ForceGraphHelper, Simulation, SimulationParameters
};
use quad_rand::RandomRange;

pub struct Graph {
    graph: Simulation<File, ()>,
    path_to_node_index: HashMap<File, NodeIndex>,
}

impl Graph {
    pub fn new() -> Self {
        let graph = Simulation::from_graph(ForceGraph::default(), SimulationParameters::default());

        Self {
            graph,
            path_to_node_index: HashMap::new(),
        }
    }

    pub fn add_file(&mut self, file: File) {
        let graph = self.graph.get_graph_mut();

        let r = || RandomRange::gen_range(-50.0, 50.0);
        let loc = Vec3::new(r(), r(), 0.0);

        let node_index =
            graph.add_force_node_with_coords(file.path.to_string_lossy(), file.clone(), loc);

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

    // TODO: remove this function, it's only for debugging purposes
    pub fn log(&mut self) {
        for frame in 0..50 {
            // update the nodes positions based on force algorithm
            self.graph.update(0.035);

            // render (print) your nodes new locations.
            println!("---- frame {frame} ----");
            for node in self.graph.get_graph().node_weights() {
                println!("\"{}\" - {:?}", node.name, node.location);
            }
            println!("-----------------------")
        }
    }
}

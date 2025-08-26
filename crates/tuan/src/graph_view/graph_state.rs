use super::camera::Camera;
use crate::{editor_view::EditorConfig, file::File};
use fdg_sim::{
    ForceGraph, ForceGraphHelper, Node, Simulation, SimulationParameters, force,
    glam::Vec3,
    petgraph::{
        graph::NodeIndex,
        visit::{EdgeRef as _, IntoEdgeReferences as _},
    },
};
use quad_rand::RandomRange;
use std::{collections::HashMap, sync::Arc};
use xilem::Vec2;

#[derive(Clone)]
pub struct GraphState {
    pub(super) graph: Simulation<File, ()>,
    pub(super) editor_config: Arc<EditorConfig>,
    pub(super) camera: Option<Camera>,
    path_to_node_index: HashMap<File, NodeIndex>,
}

impl GraphState {
    pub fn new(editor_config: Arc<EditorConfig>) -> Self {
        let mut simulation_parameters = SimulationParameters::from_force(
            // scale, cooloff, gravity, centering
            force::handy(45.0, 0.975, true, true),
        );
        simulation_parameters.dimensions = fdg_sim::Dimensions::Two;
        simulation_parameters.node_start_size = 200.0;

        let graph = Simulation::from_graph(ForceGraph::default(), simulation_parameters);

        Self {
            graph,
            editor_config,
            camera: None,
            path_to_node_index: HashMap::new(),
        }
    }

    pub(super) fn init_camera(&mut self, size: (f64, f64)) {
        if self.camera.is_some() {
            return;
        }
        let camera = Camera {
            center: Vec2::new(size.0 / 2.0, size.1 / 2.0),
            zoom: 1.0,
            viewport: size,
        };
        self.camera = Some(camera);
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

    pub fn run_positionning(&mut self, n: i16) {
        for _ in 0..n {
            self.graph.update(0.035);
        }
    }

    pub fn relax_until_stable(&mut self, max_iters: usize, eps_px: f32) {
        let mut prev = Vec::<Vec3>::new();
        let mut first = true;

        for _ in 0..max_iters {
            // snapshot positions
            let g = self.graph.get_graph();
            if first {
                prev = g.node_weights().map(|n| n.location).collect();
                first = false;
            }

            self.graph.update(0.035);

            // calc max displacement
            let g2 = self.graph.get_graph();
            let mut max_d = 0.0f32;
            for (i, n) in g2.node_weights().enumerate() {
                let d = (n.location - prev[i]).length();
                if d > max_d {
                    max_d = d;
                }
            }
            if max_d <= eps_px {
                break;
            }
            // rotate buffer
            prev.iter_mut()
                .zip(g2.node_weights())
                .for_each(|(p, n)| *p = n.location);
        }
    }
}

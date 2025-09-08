use super::camera::Camera;
use crate::{editor_view::EditorConfig, file::File, graph_view::graph_descriptor::GraphDescriptor};
use fdg_sim::{
    ForceGraph, ForceGraphHelper, Simulation, SimulationParameters, force, glam::Vec3,
    petgraph::graph::NodeIndex,
};
use quad_rand::RandomRange;
use std::{cell::RefCell, collections::HashMap, rc::Rc, sync::Arc};
use xilem::Vec2;

#[derive(Clone)]
pub struct GraphState {
    pub(super) graph: Simulation<File, ()>,
    pub(super) editor_config: Arc<EditorConfig>,
    pub(super) camera: Rc<RefCell<Option<Camera>>>,
    path_to_node_index: HashMap<File, NodeIndex>,
    pub(super) graph_descriptor: Option<super::graph_descriptor::GraphDescriptor>,
}

impl GraphState {
    pub fn new(editor_config: Arc<EditorConfig>) -> Self {
        let mut simulation_parameters =
            SimulationParameters::from_force(force::handy(45.0, 0.975, true, true));
        simulation_parameters.dimensions = fdg_sim::Dimensions::Two;
        simulation_parameters.node_start_size = 200.0;

        let graph = Simulation::from_graph(ForceGraph::default(), simulation_parameters);

        Self {
            graph,
            editor_config,
            camera: Rc::new(RefCell::new(None)),
            path_to_node_index: HashMap::new(),
            graph_descriptor: None,
        }
    }

    pub(super) fn init_camera(&mut self, size: (f64, f64)) {
        let mut camera = self.camera.borrow_mut();
        if camera.is_none() {
            let cam = Camera::new(Vec2::new(size.0 / 2.0, size.1 / 2.0), 1.0, size);
            *camera = Some(cam);
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

    pub fn relax_until_stable(&mut self, max_iters: usize, eps_px: f32) {
        let mut prev = Vec::<Vec3>::new();
        let mut first = true;

        for _ in 0..max_iters {
            let g = self.graph.get_graph();
            if first {
                prev = g.node_weights().map(|n| n.location).collect();
                first = false;
            }

            self.graph.update(0.035);

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
            prev.iter_mut()
                .zip(g2.node_weights())
                .for_each(|(p, n)| *p = n.location);
        }
    }

    pub fn update_graph_descriptor(&mut self) {
        let camera = self.camera.borrow();
        let camera = camera.as_ref().unwrap();

        let graph = self.graph.get_graph();

        let graph_descriptor = GraphDescriptor::new(graph, camera);
        self.graph_descriptor = Some(graph_descriptor);
    }
}

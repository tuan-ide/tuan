use fdg_sim::glam::Vec3;
use xilem::Vec2;

#[derive(Clone)]
pub(super) struct Camera {
    pub center: Vec2,
    zoom: f64,
    pub viewport: (f64, f64),
}

impl Camera {
    pub fn new(center: Vec2, zoom: f64, viewport: (f64, f64)) -> Self {
        Self { center, zoom, viewport }
    }

    pub(super) fn world_to_screen(&self, p: Vec3) -> Vec2 {
        let p = Vec2::new(p.x as f64, p.y as f64);
        let vp = Vec2::new(self.viewport.0, self.viewport.1);
        let screen_center = vp * 0.5;
        (p - self.center) * self.zoom + screen_center
    }

    pub(super) fn value_to_screen(&self, v: f64) -> f64 {
        v * self.zoom
    }

    pub fn zoom(&mut self, factor: f64) {
        self.zoom *= 1.0 + factor;
        self.zoom = self.zoom.clamp(0.1, 3.5);
    }
}

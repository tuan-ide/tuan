use fdg_sim::glam::Vec3;
use xilem::Vec2;

#[derive(Clone)]
pub(super) struct Camera {
    pub center: Vec2,
    pub zoom: f64,
    pub viewport: (f64, f64),
}

impl Camera {
    pub(super) fn world_to_screen(&self, p: Vec3) -> Vec2 {
        let p = Vec2::new(p.x as f64, p.y as f64);
        let vp = Vec2::new(self.viewport.0, self.viewport.1);
        let screen_center = vp * 0.5;
        (p - self.center) * self.zoom + screen_center
    }

    pub(super) fn value_to_screen(&self, v: f64) -> f64 {
        v * self.zoom
    }
}

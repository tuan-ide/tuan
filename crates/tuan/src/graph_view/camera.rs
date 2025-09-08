use fdg_sim::glam::Vec3;
use xilem::Vec2;

#[derive(Clone)]
pub(super) struct Camera {
    center: Vec2,
    zoom: f64,
    viewport: (f64, f64),
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

    pub fn zoom(&mut self, factor: f64, origin: Option<Vec2>) {
        let old_zoom = self.zoom;
        let new_zoom = (old_zoom * (1.0 + factor)).clamp(0.1, 3.5);
        if (new_zoom - old_zoom).abs() < f64::EPSILON {
            return;
        }

        let screen_center = Vec2::new(self.viewport.0 * 0.5, self.viewport.1 * 0.5);
        let s0 = origin.unwrap_or(screen_center);
        let delta_screen = s0 - screen_center;

        self.center = self.center + delta_screen * (1.0 / old_zoom - 1.0 / new_zoom);
        self.zoom = new_zoom;
    }

    pub fn translate(&mut self, xy: Vec2) {
        self.center.x -= xy.x / self.zoom;
        self.center.y -= xy.y / self.zoom;
    }
}

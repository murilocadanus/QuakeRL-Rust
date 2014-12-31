extern crate graphics;

use piston::graphics::*;
use render::{Render, Draw};

pub struct AABB {
    pub center: [f64, ..2],
    pub size: [u32, ..2],
}

impl AABB {
    pub fn new(x: f64, y: f64, w: u32, h: u32) -> AABB {
        AABB {
            center: [x, y],
            size: [w, h],
        }
    }

    pub fn trans(&self, offset: [u32, ..2]) -> AABB {
        let &center = &self.center;
        let &offset = &offset;
        AABB {
            center: [center[0] - offset[0] as f64, center[1] - offset[1] as f64],
            size: self.size,
        }
    }

    pub fn is_collided_with(&self, other: &AABB) -> bool {
        self.right() >= other.left() &&
        self.left() <= other.right() &&
        self.top() <= other.bottom() &&
        self.bottom() >= other.top()
    }

    fn left(&self) -> f64 {
        self.center[0] - self.size[0] as f64 / 2.0
    }

    fn right(&self) -> f64 {
        self.center[0] + self.size[0] as f64 / 2.0
    }

    fn top(&self) -> f64 {
        self.center[1] - self.size[1] as f64 / 2.0
    }

    fn bottom(&self) -> f64 {
        self.center[1] + self.size[1] as f64 / 2.0
    }
}

impl Draw for AABB {
    fn draw(&self, render: &mut Render) {
        if cfg!(feature="debug_collider") {
            let (w, h) = (self.size[0], self.size[1]);
            let hw = (w / 2) as f64;
            let hh = (h / 2) as f64;

            // Get the context
            let collider_ctx = &render.ctx.trans(self.center[0] - hw, self.center[1] - hh);

            // Add border to collider
            Rectangle::new([0.0, 1.0, 0.0, 1.0]).draw(
                [0.0, 0.0, w as f64, h as f64], collider_ctx, &mut render.gl
            );
        }
    }
}

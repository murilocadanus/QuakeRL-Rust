extern crate graphics;

use piston::graphics::*;
use render::{Render, Draw};
use std::num::{Float, FloatMath};

pub struct AABB {
    pub center: [f64, ..2],
    pub half_size: [f64, ..2],
}

impl AABB {
    pub fn new(c: [f64, ..2], r: [f64, ..2]) -> AABB {
        //! An AABB is a kind of Bounding Box, as so it has a center and a "radius" size per axis.
        //!
        //! +-------------+
        //! |             |
        //! |             |
        //! |      c------|
        //! |      |      | rh
        //! |      |      |
        //! +-------------+
        //!           rw
        //!
        //!
        //! To construct a AABB you should pass a center point "c" and a radius size r[x, y].
        //! As a result, the AABB drawing will be corrected to (-rw, -rh) but the real world is offsetted.
        //
        //! Example:
        //!
        //! ```
        //! let aabb = AABB::new([0.0, 0.0], [20.0, 20.0]);
        //!
        //! ```
        //!
        //! This will create a AABB of size 40 by 40.
        //!
        AABB {
            center: c,
            half_size: r,
        }
    }

    pub fn set_pos(&mut self, pos: [f64, ..2]) {
        self.center = pos;
    }

    pub fn intersect_aabb(&self, other: &AABB) -> bool {
        // this calculates the area overlaped. we must compare performance here, but this one
        // fixes the imprecision on collision compared with the boolean one.
        let x = (self.right().min(other.right())   - self.left().max(other.left())).max(0.0);
        let y = (self.bottom().min(other.bottom()) - self.top().max(other.top())  ).max(0.0);
        x * y > 0.0
/*
        self.right() >= other.left() &&
        self.left() <= other.right() &&
        self.top() <= other.bottom() &&
        self.bottom() >= other.top()
*/
    }

    // we add .4 and do a floor so we can round to the near int on both sides
    // (left and right or up and down) correctly
    fn left(&self) -> f64 {
        (self.center[0] - self.half_size[0] + 0.4).floor()
    }

    fn right(&self) -> f64 {
        (self.center[0] + self.half_size[0] + 0.4).floor()
    }

    fn top(&self) -> f64 {
        (self.center[1] - self.half_size[1] + 0.4).floor()
    }

    fn bottom(&self) -> f64 {
        (self.center[1] + self.half_size[1] + 0.4).floor()
    }
}

impl Draw for AABB {
    fn draw(&self, render: &mut Render) {
        if cfg!(feature="debug_collider") {
            let x = self.center[0];
            let y = self.center[1];
            let w = self.half_size[0];
            let h = self.half_size[1];

            // Get the context
            let collider_ctx = &render.ctx.trans(x, y);

            // Add border to collider
            Rectangle::new([0.0, 1.0, 0.0, 1.0]).draw(
                [-w, -h, w * 2.0, h * 2.0], collider_ctx, &mut render.gl
            );
        }
    }
}

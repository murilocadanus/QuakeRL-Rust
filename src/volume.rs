extern crate graphics;
use render::{Render, Draw};
use std::num::Float;
use graphics::RelativeTransform;

pub struct AABB {
    pub center: [f64; 2],
    half_size: [f64; 2],
    lower_corner: [f64; 2],
    upper_corner: [f64; 2],
}

impl AABB {
    pub fn new(c: [f64; 2], r: [f64; 2]) -> AABB {
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
            lower_corner: [c[0] - r[0], c[1] - r[1]],
            upper_corner: [c[0] + r[0], c[1] + r[1]],
        }
    }

    pub fn get_pos(&self) -> [f64; 2] {
        self.center
    }

    pub fn set_pos(&mut self, pos: [f64; 2]) {
        self.center = pos;

        let (lb, ub) = self.get_corners();
        self.lower_corner = lb;
        self.upper_corner = ub;
    }

    pub fn get_corners(&self) -> ([f64; 2], [f64; 2]) {
        (
            [self.center[0] - self.half_size[0], self.center[1] - self.half_size[1]],
            [self.center[0] + self.half_size[0], self.center[1] + self.half_size[1]],
        )
    }

    #[allow(dead_code)]
    pub fn get_dimension(&self) -> [f64; 2] {
        [self.half_size[0] * 2.0, self.half_size[1] * 2.0]
    }

    #[allow(dead_code)]
    pub fn get_halfs(&self) -> [f64; 2] {
        self.half_size
    }

    #[allow(dead_code)]
    pub fn contains_aabb(&self, other: &AABB) -> bool {
        //! Find if this AABB fully contains another AABB.
        //! This is an exact corner comparation, so it may have some float imprecision.
        other.lower_corner[0] >= self.lower_corner[0] &&
        other.upper_corner[0] <= self.upper_corner[0] &&
        other.lower_corner[1] >= self.lower_corner[1] &&
        other.upper_corner[1] <= self.upper_corner[1]
    }

    pub fn intersect_aabb(&self, other: &AABB) -> bool {
        //! Finds if there is an intersection between two AABB.
        //! This add a little rounding factor so we can accept a very tiny overlap area.
        // this fixes the imprecision on collision compared with the contains_aabb.
        let x = (self.right().min(other.right())   - self.left().max(other.left())).max(0.0);
        let y = (self.bottom().min(other.bottom()) - self.top().max(other.top())  ).max(0.0);
        x * y > 0.0
    }

    // we add .4 and do a floor so we can round to the near int on both sides
    // (left and right or up and down) correctly
    fn left(&self) -> f64 {
        (self.lower_corner[0] + 0.4).floor()
    }

    fn right(&self) -> f64 {
        (self.upper_corner[0] + 0.4).floor()
    }

    fn top(&self) -> f64 {
        (self.lower_corner[1] + 0.4).floor()
    }

    fn bottom(&self) -> f64 {
        (self.upper_corner[1] + 0.4).floor()
    }
}

impl Draw for AABB {
    fn draw(&self, at: &[f64; 2], render: &mut Render) {
        use graphics::Line;

        if cfg!(feature="debug_volume") {
            let x = at[0];
            let y = at[1];
            let w = self.half_size[0];
            let h = self.half_size[1];

            // Get the context
            let collider_ctx = &render.ctx.trans(x, y);

            // Add border to collider
            let width = 1.0;
            let color = [0.0, 1.0, 0.0, 1.0];
            Line::new(color, width).draw(
                [-w, h, w, h], collider_ctx, &mut render.gl
            );

            Line::new(color, width).draw(
                [-w, -h, w, -h], collider_ctx, &mut render.gl
            );

            Line::new(color, width).draw(
                [w, -h, w, h], collider_ctx, &mut render.gl
            );

            Line::new(color, width).draw(
                [-w, -h, -w, h], collider_ctx, &mut render.gl
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersect_aabb()
    {
        //! Test these intersections
        //!
        //! +-----+     +-----+
        //! | 1   |     | 2   |
        //! |  +-----------+  |
        //! |  |  |     |  |  |
        //! +--|--+  3  +--|--+
        //!    +-----------+
        //!
        let box1 = AABB::new([10.0, 10.0], [5.0, 5.0]);
        assert_eq!(box1.get_dimension(), [10.0, 10.0]);

        let box2 = AABB::new([30.0, 10.0], [5.0, 5.0]);
        assert_eq!(box1.intersect_aabb(&box2), false);
        assert_eq!(box2.intersect_aabb(&box1), false);

        let mut box3 = AABB::new([0.0, 0.0], [10.0, 5.0]);
        assert_eq!(box3.get_dimension(), [20.0, 10.0]);

        box3.set_pos([20.0, 15.0]);
        assert_eq!(box1.intersect_aabb(&box3), true);
        assert_eq!(box2.intersect_aabb(&box3), true);
        assert_eq!(box3.intersect_aabb(&box1), true);
        assert_eq!(box3.intersect_aabb(&box2), true);
    }

    #[test]
    fn contains_aabb()
    {
        //! Test these intersections
        //!
        //! +-----------------+
        //! | 1               |
        //! |  +-----------+  |
        //! |  | 2         |  |
        //! |  |           |  |
        //! |  +-----------+  |
        //! +-----------------+
        //!
        let box1 = AABB::new([55.0, 55.0], [50.0, 50.0]);
        assert_eq!(box1.get_dimension(), [100.0, 100.0]);
        assert_eq!(box1.get_corners(), ([5.0, 5.0], [105.0, 105.0]));

        let mut box2 = AABB::new([0.0, 0.0], [10.0, 10.0]);
        assert_eq!(box2.get_corners(), ([-10.0, -10.0], [10.0, 10.0]));

        assert_eq!(box2.contains_aabb(&box1), false);
        assert_eq!(box1.contains_aabb(&box2), false);
        assert_eq!(box1.intersect_aabb(&box2), true);
        assert_eq!(box2.intersect_aabb(&box1), true);

        box2.set_pos([45.0, 45.0]);
        assert_eq!(box1.contains_aabb(&box2), true);
        assert_eq!(box2.contains_aabb(&box1), false);
        assert_eq!(box1.intersect_aabb(&box2), true);
        assert_eq!(box2.intersect_aabb(&box1), true);
    }
}

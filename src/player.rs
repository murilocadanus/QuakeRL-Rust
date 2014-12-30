use aabb::{AABB};
use sprite::Sprite;
use render::{Render, Draw};

pub struct Player {
    pub sprite: Sprite,
    pub aabb: AABB
}

impl Player {
    pub fn aabb(&self) -> AABB {
        self.aabb.trans(self.pos)
    }

    pub fn position(&self) -> [f64, ..2] -> [f64, ..2] {
        self.pos
    }

    pub fn set_pos(&mut self, pos: [f64, ..2]) {
        sprite.x = aabb.center[0] = pos[0];
        sprite.y = aabb.center[1] = pos[1];
    }

    pub fn from_path(path: &Path) -> Player {
        Player {
            sprite: Sprite::from_path(path),
            aabb: AABB::new([0f64, 0f64], 40f64, 40f64) // UGLY: Change to not use new fn
        }
    }
}

impl Draw for Player {
    fn draw(&self, render: &mut Render) {
        render.draw(&self.sprite)
    }
}

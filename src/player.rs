use collider::{AABB};
use sprite::Sprite;
use render::{Render, Draw};

pub struct Player {
    pub sprite: Sprite,
    pub aabb: AABB
}

impl Player {
    pub fn aabb(&self) -> AABB {
        let w = self.aabb.size[0];
        let h = self.aabb.size[1];
        self.aabb.trans([w/2, h/2])
    }

    pub fn set_pos(&mut self, pos: [f64, ..2]) {
        self.sprite.x = pos[0];
        self.sprite.y = pos[1];
        self.aabb = self.aabb();
    }

    pub fn from_path(path: &Path) -> Player {
        let sprite = Sprite::from_path(path);
        let (w, h) = sprite.get_size();

        Player {
            sprite: sprite,
            aabb: AABB::new(80f64, 80f64, w, h) // UGLY: Change to not use fixed start position
        }
    }
}

impl Draw for Player {
    fn draw(&self, render: &mut Render) {
        render.draw(&self.sprite)
    }
}

extern crate vecmath;

use self::vecmath::*;
use volume::{AABB};
use sprite::Sprite;
use render::{Render, Draw};

pub struct Player {
    sprite: Sprite,
    aabb: AABB,
}

impl Player {

    #[allow(dead_code)]
    pub fn get_pos(&self) -> [f64, ..2] {
        self.sprite.pos
    }

    pub fn set_pos(&mut self, pos: [f64, ..2]) {
        self.sprite.pos = pos;
        self.aabb.set_pos([self.sprite.pos[0] + 20.0, self.sprite.pos[1] + 20.0]);
    }

    pub fn add_pos(&mut self, pos: [f64, ..2]) {
        self.sprite.pos = vec2_add(self.sprite.pos, pos);
        self.aabb.set_pos([self.sprite.pos[0] + 20.0, self.sprite.pos[1] + 20.0]);
    }

    pub fn add_rotation(&mut self, rot: f64) {
        self.sprite.rotation += rot;
    }

    pub fn intersect(&self, other: &AABB) -> bool {
        self.aabb.intersect_aabb(other)
    }

    pub fn from_path(path: &Path) -> Player {
        let sprite = Sprite::from_path(path);
        let (w, h) = sprite.get_size();

        Player {
            sprite: sprite,
            aabb: AABB::new([0.0, 0.0], [w as f64 / 2.0, h as f64 / 2.0]),
        }
    }
}

impl Draw for Player {
    fn draw(&self, render: &mut Render) {
        render.draw(&self.sprite);
        render.draw(&self.aabb);
    }
}

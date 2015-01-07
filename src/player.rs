extern crate vecmath;
extern crate graphics;

use self::vecmath::*;
use self::graphics::ImageSize;
use opengl_graphics::Texture;
use volume::{AABB};
use render::{Render, Draw, draw_texture};

pub struct Player {
    pub image: Texture,
    aabb: AABB,
}

impl Player {

    #[allow(dead_code)]
    pub fn get_pos(&self) -> [f64, ..2] {
        self.aabb.get_pos()
    }

    pub fn set_pos(&mut self, pos: [f64, ..2]) {
        self.aabb.set_pos(pos);
    }

    pub fn add_pos(&mut self, pos: [f64, ..2]) {
        let pos = vec2_add(self.get_pos(), pos);
        self.aabb.set_pos(pos);
    }

    pub fn intersect(&self, other: &AABB) -> bool {
        self.aabb.intersect_aabb(other)
    }

    pub fn from_path(path: &Path) -> Player {
        let texture = Texture::from_path(path).unwrap();
        let (w, h) = texture.get_size();

        Player {
            image: texture,
            aabb: AABB::new([0.0, 0.0], [w as f64 / 2.0, h as f64 / 2.0]),
        }
    }
}

impl Draw for Player {
    fn draw(&self, at: &[f64, ..2], render: &mut Render) {
        draw_texture(&self.image, at, render);
        render.draw(&self.aabb, at);
    }
}

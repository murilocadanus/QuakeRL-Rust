use opengl_graphics::Texture;
use aabb::{AABB};

// abstract this to an actor
pub struct Player {
    pub pos: [f64, .. 2],
    pub image: Texture,
    pub aabb: AABB
}

impl Player {
    pub fn aabb(&self) -> AABB {
        self.aabb.trans(self.pos)
    }

    pub fn position(&self) -> [f64, ..2] {
        self.pos
    }

    pub fn set_pos(&mut self, pos: [f64, ..2]) {
        self.pos = [pos[0], pos[1]];
    }

}

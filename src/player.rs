use sprite::Sprite;
use render::{Render, Draw};

pub struct Player {
    pub sprite: Sprite,
}

impl Player {
    pub fn from_path(path: &Path) -> Player {
        Player {
            sprite: Sprite::from_path(path),
        }
    }
}

impl Draw for Player {
    fn draw(&self, render: &mut Render) {
        render.draw(&self.sprite)
    }
}

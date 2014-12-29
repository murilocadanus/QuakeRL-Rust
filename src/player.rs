use sprite::Sprite;

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

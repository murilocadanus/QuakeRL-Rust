use opengl_graphics::Texture;

// abstract this to an actor
pub struct Player {
    pub x: f64,
    pub y: f64,
    pub image: Texture
}

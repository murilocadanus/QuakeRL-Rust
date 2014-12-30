extern crate graphics;

use opengl_graphics::Texture;
use piston::graphics::*;

use render::{Render, Draw};

// abstract this to an actor
pub struct Sprite {
    pub x: f64,
    pub y: f64,
    pub rotation: f64,
    pub image: Texture,
}

impl Sprite {
    pub fn from_path(path: &Path) -> Sprite {
        let image = Texture::from_path(path).unwrap();
        Sprite {
            x: 0.0,
            y: 0.0,
            rotation: 0.0,
            image: image,
        }
    }
}

impl Draw for Sprite {
    fn draw(&self, render: &mut Render) {
        let (w, h) = self.image.get_size();
        let hw = w as f64 / 2.0;
        let hh = h as f64 / 2.0;

        // Draw the player
        let sprite_context = &render.ctx
            .trans(self.x - hw, self.y - hh)
            .rot_rad(self.rotation)
            .trans(-hw, -hh)
        ;

        if cfg!(feature="debug_sprite") {
            // add border to sprite so we can debug it as we do not have a nice bg yet
            Rectangle::new([1.0, 0.0, 1.0, 1.0]).draw([-1.0, -1.0, w as f64 + 2.0, h as f64 + 2.0], sprite_context, &mut render.gl);
        }

        graphics::image(&self.image, sprite_context, &mut render.gl);

        if cfg!(feature="debug_sprite") {
            let sprite_context = &render.ctx.trans(self.x - hw, self.y - hh);
            Rectangle::new([1.0, 0.0, 1.0, 1.0]).draw([-2.0, -2.0, 5.0, 5.0], sprite_context, &mut render.gl);
        }
    }
}

extern crate graphics;

use shader_version::opengl::OpenGL;
use opengl_graphics::Gl;
use piston::graphics::*;
use sprite::Sprite;

pub struct Render {
    pub gl: Gl,
    pub ctx: Context,
}

pub trait Draw {
    fn draw(&self, render: &mut Render);
}

impl Render {
    pub fn new(w: f64, h: f64) -> Render {
        Render {
            gl: Gl::new(OpenGL::_3_2),
            ctx: Context::abs(w, h),
        }
    }

    pub fn draw<T: Draw>(&mut self, obj: T) {
        obj.draw(self);
    }
}

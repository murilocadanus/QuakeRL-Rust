extern crate graphics;

use shader_version::opengl::OpenGL;
use opengl_graphics::Gl;
use piston::graphics::*;

pub struct RenderState {
    pub enable_alpha: bool,
    pub clear: Option<internal::Color>,
}

pub struct Render {
    pub gl: Gl,
    pub ctx: Context, // this context is per object or global?
    pub states: Vec<RenderState>,
}

pub trait Draw {
    fn draw(&self, render: &mut Render);
}

impl Render {
    pub fn new(w: f64, h: f64) -> Render {
        let default = RenderState {
            enable_alpha: false,
            clear: None,
        };

        let mut v = Vec::with_capacity(5);
        v.push(default);

        Render {
            gl: Gl::new(OpenGL::_3_2),
            ctx: Context::abs(w, h),
            states: v,
        }
    }

    pub fn draw<T: Draw>(&mut self, obj: &T) {
        obj.draw(self);
    }

    pub fn state_push(&mut self, state: RenderState) {
        self.state_apply(&state);
        self.states.push(state);
    }

    pub fn state_pop(&mut self) -> RenderState {
        let ret = self.states.pop();

        let state = self.states.pop().unwrap();
        self.state_apply(&state);
        self.states.push(state);

        ret.unwrap()
    }

    fn state_apply(&mut self, state: &RenderState) {
        match state.enable_alpha {
            true  => self.gl.enable_alpha_blend(),
            false => self.gl.disable_alpha_blend(),
        }

        match state.clear {
            Some(color) => graphics::clear(color, &mut self.gl),
            None => (),
        }
    }
}

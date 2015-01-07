extern crate graphics;

use shader_version::opengl::OpenGL;
use opengl_graphics::Gl;
use piston::graphics::*;
use opengl_graphics::Texture;

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
    fn draw(&self, at: &[f64, ..2], render: &mut Render);
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

    pub fn draw<T: Draw>(&mut self, obj: &T, at: &[f64, ..2]) {
        obj.draw(at, self);
    }

    pub fn state_push(&mut self, state: RenderState) {
        self.state_apply(&state);
        self.states.push(state);
    }

    pub fn state_pop(&mut self) -> RenderState {
        assert!(self.states.len() > 1, "Unbalanced push<>pop.");
        let ret = self.states.pop().unwrap();
        let state = self.states.pop().unwrap();

        self.state_apply(&state);
        self.states.push(state);

        ret
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

pub fn draw_texture(tex: &Texture, at: &[f64, ..2], render: &mut Render) {
    let (w, h) = tex.get_size();
    let hw = w as f64 / 2.0;
    let hh = h as f64 / 2.0;

    // Draw the player
    let sprite_context = &render.ctx
        .trans(at[0], at[1])
        //.rot_rad(0.0)
        .trans(-hw, -hh)
    ;

    if cfg!(feature="debug_sprite") {
        // add border to sprite so we can debug it as we do not have a nice bg yet
        Rectangle::new([1.0, 0.0, 1.0, 1.0]).draw([-1.0, -1.0, w as f64 + 2.0, h as f64 + 2.0], sprite_context, &mut render.gl);
    }

    graphics::image(tex, sprite_context, &mut render.gl);

    if cfg!(feature="debug_sprite") {
        let sprite_context = &render.ctx.trans(at[0], at[1]);
        Rectangle::new([1.0, 0.0, 1.0, 1.0]).draw([-2.0, -2.0, 5.0, 5.0], sprite_context, &mut render.gl);
    }
}

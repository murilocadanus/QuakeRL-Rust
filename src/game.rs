extern crate graphics;
extern crate input;

use self::graphics::*;
use self::input::{Button, Key};

use opengl_graphics::Gl;
use piston::RenderArgs;
use piston::UpdateArgs;
use event::Window;
use player::Player;

pub struct Game {
    pub gl: Gl,
    pub player: Player,
    pub rotation: f64,
}

impl Game {
    pub fn render<W: Window>(&mut self, _: &mut W, args: &RenderArgs) {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);

        // Clear the screen
        graphics::clear([1.0, ..4], &mut self.gl);

        // Draw player actor
        self.draw_actor(context);
    }

    fn draw_actor(&mut self, context: &Context) {
            // find a way to get w/h from texture private width/height
        let w = 40f64;
        let h = 40f64;
        let hw = w / 2.0f64;
        let hh = h / 2.0f64;

        // Draw the player
        let player_context = &context
            .trans(self.player.x - hw, self.player.y - hh)
            .rot_rad(self.rotation)
            .trans(-hw, -hh)
        ;

        if cfg!(feature="debug_sprite") {
            // add border to sprite so we can debug it as we do not have a nice bg yet
            Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw([-1.0, -1.0, w + 2.0, h + 2.0], player_context, &mut self.gl);
        }

        graphics::image(&self.player.image, player_context, &mut self.gl);

        if cfg!(feature="debug_sprite") {
            let player_context = &context.trans(self.player.x - hw, self.player.y - hh);
            Rectangle::new([1.0, 0.0, 1.0, 1.0]).draw([-2.0, -2.0, 5.0, 5.0], player_context, &mut self.gl);
        }
    }

    pub fn update<W: Window>(&mut self, _: &mut W, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        //self.rotation += 2.0 * args.dt;
    }

    pub fn press(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up)       => { self.player.y -= 10.0 },
            Button::Keyboard(Key::Down)     => { self.player.y += 10.0 },
            Button::Keyboard(Key::Left)     => { self.player.x -= 10.0 },
            Button::Keyboard(Key::Right)    => { self.player.x += 10.0 },
            _ => {}
        }
    }
}

extern crate graphics;
extern crate input;

use self::graphics::*;
use self::input::{Button, Key};

use opengl_graphics::Gl;
use piston::RenderArgs;
use piston::UpdateArgs;
use event::Window;
use player::Player;

pub struct Game
{
    pub gl: Gl,
    pub player: Player,
    pub rotation: f64,
}

impl Game
{
    pub fn render<W: Window>(&mut self, _: &mut W, args: &RenderArgs)
    {
        // Set up a context to draw into.
        let context = &Context::abs(args.width as f64, args.height as f64);

        // Clear the screen
        graphics::clear([1.0, ..4], &mut self.gl);

        // Draw the player
        let player_context = &context.trans(self.player.x, self.player.y)
            .rot_rad(self.rotation)
            .trans(-25.0, -25.0);

        graphics::image(&self.player.image, player_context, &mut self.gl);
        //Rectangle::new([1.0, 0.0, 0.0, 1.0]).draw([0.0, 0.0, 50.0, 50.0], player_context, &mut self.gl);
    }

    pub fn update<W: Window>(&mut self, _: &mut W, args: &UpdateArgs)
    {
        // Rotate 2 radians per second.
        // self.rotation += 2.0 * args.dt;
    }

    pub fn press(&mut self, button: Button)
    {
        match button
        {
            Button::Keyboard(Key::Up)       => { self.player.y -= 10.0 },
            Button::Keyboard(Key::Down)     => { self.player.y += 10.0 },
            Button::Keyboard(Key::Left)     => { self.player.x -= 10.0 },
            Button::Keyboard(Key::Right)    => { self.player.x += 10.0 },
            _ => {}
        }
    }
}

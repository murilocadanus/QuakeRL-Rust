extern crate graphics;
extern crate input;

use self::input::{Button, Key};

use piston::RenderArgs;
use piston::UpdateArgs;
use player::Player;
use render::{Render, RenderState};

pub struct Game {
    pub render: Render,
    pub player: Player,
}

impl Game {
    pub fn render(&mut self, _: &RenderArgs) {
        let state = RenderState {
            enable_alpha: true,
            clear: Some([0.0, ..4]),
        };

        self.render.state_push(state);
        self.render.draw(&self.player);
        self.render.state_pop(); // how to auto pop?
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.player.sprite.rotation += 2.0 * args.dt;
    }

    pub fn press(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up)       => { self.player.sprite.y -= 10.0 },
            Button::Keyboard(Key::Down)     => { self.player.sprite.y += 10.0 },
            Button::Keyboard(Key::Left)     => { self.player.sprite.x -= 10.0 },
            Button::Keyboard(Key::Right)    => { self.player.sprite.x += 10.0 },
            _ => {}
        }
    }
}

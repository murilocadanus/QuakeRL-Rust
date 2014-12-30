extern crate graphics;
extern crate input;

use self::input::{Button, Key};

use piston::RenderArgs;
use piston::UpdateArgs;
use player::Player;
use aabb::AABB;
use render::{Render, RenderState};

pub struct Game {
    pub render: Render,
    pub player: Player,
    pub top_wall_aabb: AABB
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
            Button::Keyboard(Key::Up)       => {
                if(self.player.aabb.is_collided_with(&self.top_wall_aabb)) {
                    self.player.pos[1] = self.player.pos[1];
                    self.player.aabb.center[1] = self.player.aabb.center[1];
                } else {
                    self.player.pos[1] -= 10.0;
                    self.player.aabb.center[1] -= 10.0;
                }
            },
            Button::Keyboard(Key::Down)     => {
                self.player.pos[1] += 10.0;
                self.player.aabb.center[1] += 10.0;
            },
            Button::Keyboard(Key::Left)     => { self.player.pos[0] -= 10.0 },
            Button::Keyboard(Key::Right)    => { self.player.pos[0] += 10.0 },
            _ => {}
        }
    }
}

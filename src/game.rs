extern crate graphics;
extern crate input;
extern crate vecmath;

use piston::RenderArgs;
use piston::UpdateArgs;
use player::Player;
use self::vecmath::*;
use collider::AABB;

use render::{Render, RenderState};
use input::{Input, Signal};

pub struct Game {
    pub render: Render,
    pub input: Input,
    pub player: Player,
    pub timestamp: f64,
    pub top_wall: AABB,
    pub bottom_wall: AABB,
    pub left_wall: AABB,
    pub right_wall: AABB
}

impl Game {
    pub fn render(&mut self, _: &RenderArgs) {
        let state = RenderState {
            enable_alpha: true,
            clear: Some([0.0, ..4]),
        };

        self.render.state_push(state);
        self.render.draw(&self.top_wall);
        self.render.draw(&self.bottom_wall);
        self.render.draw(&self.left_wall);
        self.render.draw(&self.right_wall);
        self.render.draw(&self.player.aabb);
        self.render.draw(&self.player);
        self.render.state_pop();
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.player.sprite.rotation += 0.0 * args.dt;

        let x = self.input.get_signal(Signal::AxisX);
        let y = self.input.get_signal(Signal::AxisY);

        let dir = if x == 0.0 && y == 0.0 {
            [0.0, 0.0]
        } else {
            vec2_normalized([x, y])
        };

        self.player.sprite.pos = vec2_add(self.player.sprite.pos, dir);
    }

/*
Do the collision check on update each frame base on the player movement.
*/
/*
    pub fn press(&mut self, button: Button) {
        match button {
            Button::Keyboard(Key::Up) => {
                if self.player.aabb.is_collided_with(&self.top_wall) {
                    self.player.sprite.y = self.player.sprite.y;
                    self.player.aabb.center[1] = self.player.aabb.center[1];
                } else {
                    self.player.sprite.y -= 40.0;
                    self.player.aabb.center[1] -= 40.0;
                }
            },
            Button::Keyboard(Key::Down) => {
                if self.player.aabb.is_collided_with(&self.bottom_wall) {
                    self.player.sprite.y = self.player.sprite.y;
                    self.player.aabb.center[1] = self.player.aabb.center[1];
                } else {
                    self.player.sprite.y += 40.0;
                    self.player.aabb.center[1] += 40.0;
                }
            },
            Button::Keyboard(Key::Left) => {
                if self.player.aabb.is_collided_with(&self.left_wall) {
                    self.player.sprite.x = self.player.sprite.x;
                    self.player.aabb.center[0] = self.player.aabb.center[0];
                } else {
                    self.player.sprite.x -= 40.0;
                    self.player.aabb.center[0] -= 40.0;
                }
            },
            Button::Keyboard(Key::Right) => {
                if self.player.aabb.is_collided_with(&self.right_wall) {
                    self.player.sprite.x = self.player.sprite.x;
                    self.player.aabb.center[0] = self.player.aabb.center[0];
                } else {
                    self.player.sprite.x += 40.0;
                    self.player.aabb.center[0] += 40.0;
                }
            },
            _ => {}
        }
    }
    */
}

extern crate graphics;
extern crate input;
extern crate vecmath;

use piston::RenderArgs;
use piston::UpdateArgs;
use player::Player;
use self::vecmath::*;

use render::{Render, RenderState};
use input::{Input, Signal};

pub struct Game {
    pub render: Render,
    pub input: Input,
    pub player: Player,
    pub timestamp: f64,
}

impl Game {
    pub fn render(&mut self, _: &RenderArgs) {
        let state = RenderState {
            enable_alpha: true,
            clear: Some([0.0, ..4]),
        };

        self.render.state_push(state);
        self.render.draw(&self.player);
        self.render.state_pop();
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        // Rotate 2 radians per second.
        self.player.sprite.rotation += 2.0 * args.dt;

        let x = self.input.get_signal(Signal::AxisX);
        let y = self.input.get_signal(Signal::AxisY);

        let dir = if x == 0.0 && y == 0.0 {
            [0.0, 0.0]
        } else {
            vec2_normalized([x, y])
        };

        self.player.sprite.pos = vec2_add(self.player.sprite.pos, dir);
    }
}

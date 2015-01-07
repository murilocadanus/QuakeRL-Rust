#![feature(default_type_params)]
#![feature(globs)]
#![cfg_attr(test, allow(warnings))]

extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;
extern crate quack;

mod settings;
mod app;
mod render;
mod game;
mod player;
mod volume;
mod tilemap;
mod input;

fn main() {
    // Set the config for the game
    let config = settings::Settings {
        title: "QuakeRL".to_string(),
        window_width: 800,
        window_height: 600,
    };

    // Create and run the game
    let mut app = app::App::new(config);
    app.run();
}

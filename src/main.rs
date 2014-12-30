#![feature(globs)]
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;

mod settings;
mod app;
mod game;
mod player;
mod aabb;

fn main() {
    // Set the config for the game
    let config = settings::Settings {
        title: "QuakeRL".to_string(),
        window_width: 800,
        window_height: 600
    };

    // Create and run the game
    let mut app = app::App::new(config);
    app.run();
}

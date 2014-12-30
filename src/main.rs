#![feature(globs)]
extern crate piston;
extern crate sdl2_window;
extern crate opengl_graphics;
extern crate shader_version;
extern crate event;
extern crate current;

mod settings;
mod app;
mod render;
mod game;
mod player;
mod sprite;
mod tilemap;

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

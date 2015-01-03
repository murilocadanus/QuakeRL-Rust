use shader_version::opengl::OpenGL;
use settings::Settings;
use sdl2_window::Sdl2Window;

// Represents the App
pub struct App {
    config: Settings
}

impl App {
    // Returns a app struct
    pub fn new(config: Settings) -> App {
        // Return a new App
        App {config: config}
    }

    // Run the app
    pub fn run(&mut self) {
        // Set the namespaces
        use std::cell::RefCell;
        use event::{ Events, RenderEvent, UpdateEvent, PressEvent };
        use game::Game;
        use player::Player;
        use collider::AABB;
        use render::Render;
        use tilemap::TileMap;

        // Create the window
        let window = RefCell::new(self.window());
        let render = Render::new(self.config.window_width as f64, self.config.window_height as f64);

        // Create the player
        let mut player = Player::from_path(&Path::new("./assets/ranger_avatar.png"));
        player.set_pos([80f64, 80f64]);

        // Create the map
        let mut tilemap = TileMap::from_tileset_path(&Path::new("./assets/tileset.png"));
        tilemap.tileset.x = (self.config.window_width / 2) as f64;
        tilemap.tileset.y = (self.config.window_height / 2) as f64;
        tilemap.build_procedural_map(20u, 15u);

        // Create a new game and run it.
        let mut game = Game {
            render: render,
            player: player,
            tilemap: tilemap,
            top_wall: AABB::new(
                self.config.window_width as f64 / 2.0,
                20.0,
                self.config.window_width,
                20 * 2 as u32
            ),
            bottom_wall: AABB::new(
                self.config.window_width as f64 / 2.0,
                self.config.window_height as f64 - 20.0,
                self.config.window_width,
                20 * 2 as u32
            ),
            left_wall: AABB::new(
                20.0,
                self.config.window_height as f64 / 2.0,
                20 * 2 as u32,
                self.config.window_height
            ),
            right_wall: AABB::new(
                self.config.window_width as f64 - 20.0,
                self.config.window_height as f64 / 2.0,
                20 * 2 as u32,
                self.config.window_height
            )
        };

        // Iterate the main game loop
        for e in Events::new(&window) {
            e.render(|args| game.render(args));
            e.update(|args| game.update(args));
            e.press (|args| game.press(args));
        }
    }

    // Returns a window.
    fn window(&self) -> Sdl2Window {
        // Set the namespaces
        use event::WindowSettings;

        // Values for Window Creation
        let window_settings = WindowSettings {
            title: self.config.title.to_string(),
            size: [self.config.window_width, self.config.window_height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        };

        // Create SDL Window
        Sdl2Window::new(OpenGL::_3_2, window_settings)
    }
}

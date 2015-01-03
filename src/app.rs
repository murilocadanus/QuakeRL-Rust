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
        use event::Events;
        use game::Game;
        use player::Player;
        use volume::AABB;
        use render::Render;
        use tilemap::TileMap;
        use input::Input;

        let w = self.config.window_width as f64;
        let h = self.config.window_height as f64;

        // Create the window
        let window = RefCell::new(self.window());
        let render = Render::new(w, h);
        let input = Input::new();

        // Create the player
        let mut player = Player::from_path(&Path::new("./assets/ranger_avatar.png"));
        player.set_pos([40.0, 40.0]);

        // Create the map
        let mut tilemap = TileMap::from_tileset_path(&Path::new("./assets/tileset.png"));
        tilemap.tileset.x = (self.config.window_width / 2) as f64;
        tilemap.tileset.y = (self.config.window_height / 2) as f64;
        tilemap.build_procedural_map(20u, 15u);

        // Create a new game and run it.
        let mut game = Game {
            render: render,
            input: input,
            player: player,
            tilemap: tilemap,
            timestamp: 0.0,
            top_wall:    AABB::new([w / 2.0, 20.0],     [w / 2.0, 20.0]),
            bottom_wall: AABB::new([w / 2.0, h - 20.0], [w / 2.0, 20.0]),
            left_wall:   AABB::new([20.0, h / 2.0],     [20.0, h / 2.0]),
            right_wall:  AABB::new([w - 20.0, h / 2.0], [20.0, h / 2.0]),
        };

        // Iterate the main game loop
        let mut dt : f64 = 0.0;
        for e in Events::new(&window) {
            use piston::input::Input;
            use piston::event::Event;
            use event::{RenderEvent, UpdateEvent, PressEvent, ReleaseEvent };
            let e: Event<Input> = e;
            e.render (|args|   game.render(args));
            e.update (|args| { game.update(args); dt = args.dt; });
            e.press  (|args|   game.input.press(args, dt));
            e.release(|args|   game.input.release(args, dt));
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

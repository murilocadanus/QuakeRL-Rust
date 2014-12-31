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
        use event::{ Events, RenderEvent, UpdateEvent, PressEvent, ReleaseEvent };
        use game::Game;
        use player::Player;
        use render::Render;
        use input::Input;

        // Create the window
        let window = RefCell::new(self.window());
        let render = Render::new(self.config.window_width as f64, self.config.window_height as f64);
        let input = Input::new();

        let mut player = Player::from_path(&Path::new("./assets/ranger_avatar.png"));
        player.sprite.pos = [(self.config.window_width / 2) as f64, (self.config.window_height / 2) as f64];

        // Create a new game and run it.
        let mut game = Game {
            render: render,
            input: input,
            player: player,
            timestamp: 0.0,
        };

        // Iterate the main game loop
        let mut dt : f64 = 0.0;
        for e in Events::new(&window) {
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

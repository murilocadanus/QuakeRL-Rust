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
        use render::Render;

        // Create the window
        let window = RefCell::new(self.window());
        let render = Render::new(self.config.window_width as f64, self.config.window_height as f64);

        let mut player = Player::from_path(&Path::new("./assets/ranger_avatar.png"));
        player.sprite.x = (self.config.window_width / 2) as f64;
        player.sprite.y = (self.config.window_height / 2) as f64;

        // Create a new game and run it.
        let mut game = Game {
            render: render,
            player: player,
        };

        // Iterate the main game loop
        for e in Events::new(&window) {
            e.render(|r| game.render(window.borrow_mut().deref_mut(), r));
            e.update(|u| game.update(window.borrow_mut().deref_mut(), u));
            e.press(|k| game.press(k));
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

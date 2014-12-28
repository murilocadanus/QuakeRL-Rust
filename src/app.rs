use shader_version::opengl::OpenGL::_3_2;
use settings::Settings;
use sdl2_window::Sdl2Window;

// Represents the App
pub struct App
{
    config: Settings
}

impl App
{
    // Returns a app struct
    pub fn new(config: Settings) -> App
    {
        // Return a new App
        App {config: config}
    }

    // Run the app
    pub fn run(&mut self)
    {
        // Set the namespaces
        use std::cell::RefCell;
        use event::{ Events, RenderEvent, UpdateEvent, PressEvent };
        use opengl_graphics::{ Gl, Texture };
        use game::Game;
        use player::Player;

        // Create the window
        let window = RefCell::new(self.window());

        // Load player image
        let image = Path::new("./assets/ranger_avatar.png");
        let image = Texture::from_path(&image).unwrap();

        // Create the player
        let player = Player { x: (self.config.window_width/2) as f64, y: (self.config.window_height/2) as f64, image: image};

        // Create a new game and run it.
        let mut game = Game { gl: Gl::new(_3_2), player: player, rotation: 0.0 };

        // Iterate the main game loop
        for e in Events::new(&window)
        {
            e.render(|r| game.render(window.borrow_mut().deref_mut(), r));
            e.update(|u| game.update(window.borrow_mut().deref_mut(), u));
            e.press(|k| game.press(k));
        }
    }

    // Returns a window.
    fn window(&self) -> Sdl2Window
    {
        // Set the namespaces
        use event::WindowSettings;

        // Values for Window Creation
        let window_settings = WindowSettings
        {
            title: self.config.title.to_string(),
            size: [self.config.window_width, self.config.window_height],
            fullscreen: false,
            exit_on_esc: true,
            samples: 0,
        };

        // Create SDL Window
        Sdl2Window::new(_3_2, window_settings)
    }
}

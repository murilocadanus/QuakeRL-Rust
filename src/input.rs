use piston::input::{Button, Key};

const MAX_INPUT : uint = 4; // find how to use size of Signal

pub struct Input {
    buffer: InputBuffer,
}

pub enum Signal {
    AxisX,
    AxisY,
}

struct InputBuffer {
    pressed: [(f64, f64, f64), ..MAX_INPUT],
//    previous_pressed: [(f32, f64, f64), ..MAX_INPUT],
}

/*
With this we should be able to implement an Action system so we can do bindings somewhat like:

Action::bind_axis(Signal::AxisY, Signal::AxisX, Action::Move); // that will have a vec2d(x, y)
Action::bind(Signal::Escape, Action::Quit);
Action::bind_timed((Signal::X, 2.0), Action::HoldXByTwoSeconds);
Action::bind_sequence((Signal::X, 2.0), (Signal::Y, 0.0), (Signal::X, 1.0), Action::PressedX2sThenYThenX1s);
*/

impl Input {
    pub fn new() -> Input {
        let buffer = InputBuffer {
            pressed: [(0.0, 0.0, 0.0), ..MAX_INPUT],
//            previous_pressed: [(0.0, 0.0, 0.0), ..MAX_INPUT],
        };

        Input {
            buffer: buffer
        }
    }

    pub fn get_signal(&mut self, signal: Signal) -> f64 {
        let (val, _, _) = self.buffer.pressed[signal as uint];
        val
    }

    #[allow(dead_code)]
    pub fn get_signal_data(&mut self, signal: Signal) -> (f64, f64, f64) {
        self.buffer.pressed[signal as uint]
    }

    fn do_press(&mut self, signal: Signal, amount: f64, dt: f64) {
        let idx = signal as uint;
        self.buffer.pressed[idx] = (amount, dt, 0.0);
    }

    fn do_release(&mut self, signal: Signal, dt: f64) {
        let idx = signal as uint;
        let (_, press_dt, _) = self.buffer.pressed[idx];
        self.buffer.pressed[idx] = (0.0, press_dt, dt);
    }

    // to_: http://aturon.github.io/style/naming/conversions.html
    fn to_signal(button: Button) -> Option<(Signal, f64)> {
        match button {
            Button::Keyboard(Key::Up)       => { Some((Signal::AxisY, -1.0)) },
            Button::Keyboard(Key::Down)     => { Some((Signal::AxisY,  1.0)) },
            Button::Keyboard(Key::Left)     => { Some((Signal::AxisX, -1.0)) },
            Button::Keyboard(Key::Right)    => { Some((Signal::AxisX,  1.0)) },
            _ => { None }
        }
    }

    pub fn press(&mut self, button: Button, dt: f64) {
        match Input::to_signal(button) {
            Some((signal, amount)) => self.do_press(signal, amount, dt),
            None => (),
        }
    }

    pub fn release(&mut self, button: Button, dt: f64) {
        match Input::to_signal(button) {
            Some((signal, _)) => self.do_release(signal, dt),
            None => (),
        }
    }
}

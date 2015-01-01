use piston::input::{Button, Key};

/// TEMPORARY: A number maximum for input signals. This should be the size of enum Signal.
const MAX_INPUT : uint = 2; // find how to use size of Signal

/// An Input to Signal conversor.
/// Input is a kind of a conversor of input buttons to input signals.
/// All input are converted to signals in the range of [-1.0, 1.0].
pub struct Input {
    /// An internal buffer for current input state.
    buffer: InputBuffer,
}

/// A signal is an ecoding of one or more keys.
/// Related inputs are codified together, ie. KeyLeft and KeyRight are considered signals
/// on axis x.
pub enum Signal {
    AxisX,
    AxisY,
}

/// Internal buffer that hold all needed states from current inputs.
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
        //! Get a specified signal current value.
        //!
        //! Example:
        //!
        //! ```
        //! let mut input = Input::new();
        //!
        //! input.press(Button::Keyboard(Key::Up), 0.0);
        //! let y = input.get_signal(Signal::AxisY);
        //! assert_eq!(y, -1.0);
        //! ```
        let (val, _, _) = self.buffer.pressed[signal as uint];
        val
    }

    #[allow(dead_code)]
    pub fn get_signal_data(&mut self, signal: Signal) -> (f64, f64, f64) {
        //! Get a specified signal stored data in the form of a 3-tuple of f64:
        //! (signal value, milliseconds begin/press time, milliseconds end/release time)
        self.buffer.pressed[signal as uint]
    }

    // FIXME: find where is the bug.
    // there is a old bug (sdl?) when we press LEFT+(UP|DOWN) and then the next (DOWN|UP) is not registered.
    // try this: right, down, then up.
    // compare with: left, down then up.
    fn do_press(&mut self, signal: Signal, amount: f64, dt: f64) {
        let idx = signal as uint;
        let (val, press_dt, _) = self.buffer.pressed[idx];
        //println!("press: {} - {}", val, amount);
        self.buffer.pressed[idx] = (val + amount, dt, 0.0);
    }

    fn do_release(&mut self, signal: Signal, amount: f64, dt: f64) {
        let idx = signal as uint;
        let (val, press_dt, _) = self.buffer.pressed[idx];
        //println!("release: {} - {}", val, amount);
        self.buffer.pressed[idx] = (val - amount, press_dt, dt);
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
            Some((signal, amount)) => self.do_release(signal, amount, dt),
            None => (),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use piston::input::{Button, Key};

    #[test]
    fn signal_axisx_opposite_press()
    {
        let mut input = Input::new();

        input.press(Button::Keyboard(Key::Left), 0.0);
        let x = input.get_signal(Signal::AxisX);
        assert_eq!(x, -1.0);

        input.press(Button::Keyboard(Key::Right), 1.0);
        let x = input.get_signal(Signal::AxisX);
        assert_eq!(x, 0.0);

        input.release(Button::Keyboard(Key::Left), 2.0);
        let x = input.get_signal(Signal::AxisX);
        assert_eq!(x, 1.0);

        input.release(Button::Keyboard(Key::Right), 3.0);
        let x = input.get_signal(Signal::AxisX);
        assert_eq!(x, 0.0);
    }

    #[test]
    fn signal_axisy_opposite_press()
    {
        let mut input = Input::new();

        input.press(Button::Keyboard(Key::Up), 0.0);
        let y = input.get_signal(Signal::AxisY);
        assert_eq!(y, -1.0);

        input.press(Button::Keyboard(Key::Down), 1.0);
        let y = input.get_signal(Signal::AxisY);
        assert_eq!(y, 0.0);

        input.release(Button::Keyboard(Key::Up), 2.0);
        let y = input.get_signal(Signal::AxisY);
        assert_eq!(y, 1.0);

        input.release(Button::Keyboard(Key::Down), 3.0);
        let y = input.get_signal(Signal::AxisY);
        assert_eq!(y, 0.0);
    }
}

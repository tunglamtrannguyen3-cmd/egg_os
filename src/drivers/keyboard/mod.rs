pub mod port;
pub mod scancodes;

use scancodes::KeyboardState;

pub struct Keyboard {
    state: KeyboardState,
}

impl Keyboard {
    pub const fn new() -> Self {
        Self {
            state: KeyboardState::new(),
        }
    }

    /// Polls the keyboard I/O port and updates internal state.
    /// Returns `Some(char)` if a valid ASCII keypress (letter, number, symbol) was produced.
    pub fn pop_char(&mut self) -> Option<char> {
        let scancode = port::read_io_port_60();
        self.state.process_scancode(scancode)
    }
}


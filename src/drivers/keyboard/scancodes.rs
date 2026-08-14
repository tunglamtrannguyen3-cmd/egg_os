#![no_std]

pub struct KeyboardState {
    pub shift_pressed: bool,
}

impl KeyboardState {
    pub const fn new() -> Self {
        Self { shift_pressed: false }
    }

    /// Processes a raw PS/2 Set 1 scancode, updates internal state (like Shift),
    /// and returns the mapped ASCII character if applicable.
    pub fn process_scancode(&mut self, scancode: u8) -> Option<char> {
        match scancode {
            // Shift Key Pressed (Left Shift: 0x2A, Right Shift: 0x36)
            0x2A | 0x36 => {
                self.shift_pressed = true;
                None
            }
            // Shift Key Released (Left Shift Release: 0xAA, Right Shift Release: 0xB6)
            0xAA | 0xB6 => {
                self.shift_pressed = false;
                None
            }
            // Ignore other key releases (scancodes with the highest bit set)
            code if code & 0x80 != 0 => None,

            // Key press mapping
            code => self.map_key(code),
        }
    }

    fn map_key(&self, scancode: u8) -> Option<char> {
        let (unshifted, shifted) = match scancode {
            // Numbers & Top Row Symbols
            0x02 => ('1', '!'),
            0x03 => ('2', '@'),
            0x04 => ('3', '#'),
            0x05 => ('4', '$'),
            0x06 => ('5', '%'),
            0x07 => ('6', '^'),
            0x08 => ('7', '&'),
            0x09 => ('8', '*'),
            0x0A => ('9', '('),
            0x0B => ('0', ')'),
            0x0C => ('-', '_'),
            0x0D => ('=', '+'),

            // Letters
            0x10 => ('q', 'Q'),
            0x11 => ('w', 'W'),
            0x12 => ('e', 'E'),
            0x13 => ('r', 'R'),
            0x14 => ('t', 'T'),
            0x15 => ('y', 'Y'),
            0x16 => ('u', 'U'),
            0x17 => ('i', 'I'),
            0x18 => ('o', 'O'),
            0x19 => ('p', 'P'),
            0x1E => ('a', 'A'),
            0x1F => ('s', 'S'),
            0x20 => ('d', 'D'),
            0x21 => ('f', 'F'),
            0x22 => ('g', 'G'),
            0x23 => ('h', 'H'),
            0x24 => ('j', 'J'),
            0x25 => ('k', 'K'),
            0x26 => ('l', 'L'),
            0x2C => ('z', 'Z'),
            0x2D => ('x', 'X'),
            0x2E => ('c', 'C'),
            0x2F => ('v', 'V'),
            0x30 => ('b', 'B'),
            0x31 => ('n', 'N'),
            0x32 => ('m', 'M'),

            // Punctuation & Controls
            0x1A => ('[', '{'),
            0x1B => (']', '}'),
            0x27 => (';', ':'),
            0x28 => ('\'', '"'),
            0x29 => ('`', '~'),
            0x2B => ('\\', '|'),
            0x33 => (',', '<'),
            0x34 => ('.', '>'),
            0x35 => ('/', '?'),

            // Whitespace & Special
            0x39 => (' ', ' '),
            0x1C => ('\n', '\n'),
            0x0E => ('\x08', '\x08'), // Backspace

            _ => return None,
        };

        if self.shift_pressed {
            Some(shifted)
        } else {
            Some(unshifted)
        }
    }
}


use super::font::draw_char;
use super::framebuffer::Framebuffer;

pub struct TextConsole {
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub margin_left: usize,
    pub margin_top: usize,
    pub text_color: u32,
    pub bg_color: u32,
    pub shift_pressed: bool,
    pub caps_lock: bool,
}

impl TextConsole {
    pub fn new(margin_left: usize, margin_top: usize, text_color: u32, bg_color: u32) -> Self {
        Self {
            cursor_x: margin_left,
            cursor_y: margin_top,
            margin_left,
            margin_top,
            text_color,
            bg_color,
            shift_pressed: false,
            caps_lock: false,
        }
    }

    /// Process incoming typing events from keyboard scancodes
    pub fn handle_key(&mut self, fb: &mut Framebuffer, ch: char) {
        match ch {
            // 1. BACKSPACE: Move back 8px and wipe character with background fill
            '\x08' => {
                if self.cursor_x >= self.margin_left + 8 {
                    self.cursor_x -= 8;
                    unsafe {
                        fb.draw_rect(self.cursor_x, self.cursor_y, 8, 8, self.bg_color);
                    }
                } else if self.cursor_y >= self.margin_top + 10 {
                    // Wrap back to end of previous line
                    self.cursor_y -= 10;
                    self.cursor_x = fb.mode.width - 16;
                    unsafe {
                        fb.draw_rect(self.cursor_x, self.cursor_y, 8, 8, self.bg_color);
                    }
                }
            }

            // 2. ENTER / RETURN: Advance to next line
            '\n' | '\r' => {
                self.cursor_x = self.margin_left;
                self.cursor_y += 10;
            }

            // 3. TAB: Move 4 character widths (32px) forward
            '\t' => {
                let tab_space = 32;
                let next_x = self.cursor_x + tab_space;
                if next_x < fb.mode.width - 8 {
                    self.cursor_x = next_x;
                }
            }

            // 4. PRINTABLE KEYS: Apply Shift/Caps capitalization & draw
            c => {
                let processed_char = self.apply_capitalization(c);
                
                // Screen edge auto-wrap
                if self.cursor_x + 8 >= fb.mode.width {
                    self.cursor_x = self.margin_left;
                    self.cursor_y += 10;
                }

                draw_char(fb, self.cursor_x, self.cursor_y, processed_char, self.text_color);
                self.cursor_x += 8;
            }
        }
    }

    /// Shift and CapsLock mapping for letters and standard top-row symbols
    fn apply_capitalization(&self, ch: char) -> char {
        let is_uppercase = self.shift_pressed ^ self.caps_lock;

        if self.shift_pressed {
            match ch {
                '1' => '!', '2' => '@', '3' => '#', '4' => '$', '5' => '%',
                '6' => '^', '7' => '&', '8' => '*', '9' => '(', '0' => ')',
                '-' => '_', '=' => '+', '[' => '{', ']' => '}', '\\' => '|',
                ';' => ':', '\'' => '"', ',' => '<', '.' => '>', '/' => '?',
                '`' => '~',
                c if c.is_ascii_alphabetic() => c.to_ascii_uppercase(),
                c => c,
            }
        } else if is_uppercase && ch.is_ascii_alphabetic() {
            ch.to_ascii_uppercase()
        } else {
            ch
        }
    }
}

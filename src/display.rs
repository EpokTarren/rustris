use std::fmt::Display;

// Colours as their associated XTerm colour
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
    None = 0,

    Cyan = 45,
    Yellow = 226,
    Purple = 93,
    Green = 34,
    Red = 124,
    Blue = 21,
    Orange = 202,

    Grey = 250,
    White = 255,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 24;
const BUFFER_SIZE: usize = BUFFER_WIDTH * BUFFER_HEIGHT;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenCell {
    c: char,
    colour: Colour,
}

impl Default for ScreenCell {
    fn default() -> Self {
        Self {
            c: ' ',
            colour: Colour::None,
        }
    }
}

impl ScreenCell {
    pub fn new(c: char, colour: Colour) -> Self {
        Self { c, colour }
    }
}

impl Display for ScreenCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.c)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenBuffer {
    buffer: [ScreenCell; BUFFER_SIZE],
}

impl Default for ScreenBuffer {
    fn default() -> Self {
        Self {
            buffer: [Default::default(); BUFFER_SIZE],
        }
    }
}

impl ScreenBuffer {
    pub fn write(&mut self, x: usize, y: usize, content: ScreenCell) {
        if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
            self.buffer[x + y * BUFFER_WIDTH] = content;
        }
    }

    pub fn write_string(&mut self, x: usize, y: usize, content: &str, colour: Colour) {
        for (i, c) in content.chars().enumerate() {
            self.write(x + i, y, ScreenCell::new(c, colour));
        }
    }

    pub fn join(&mut self, buf: ScreenBuffer) {
        for i in 0..BUFFER_SIZE {
            if buf.buffer[i].colour != Colour::None {
                self.buffer[i] = buf.buffer[i];
            }
        }
    }

    pub fn print(self) {
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);

        for x in 0..BUFFER_WIDTH {
            for y in 0..BUFFER_HEIGHT {
                print!("{}", self.buffer[x + y * BUFFER_WIDTH]);
            }
            print!("\n")
        }
    }
}

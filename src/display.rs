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
        write!(f, "\x1b[38;5;{}m{}\x1b[0m", self.colour as u8, self.c)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ScreenBuffer {
    buffer: [[ScreenCell; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Default for ScreenBuffer {
    fn default() -> Self {
        Self {
            buffer: [[ScreenCell::default(); BUFFER_WIDTH]; BUFFER_HEIGHT],
        }
    }
}

pub fn clear_terminal() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}

fn move_cursor(x: usize, y: usize) {
    print!("\x1b[{y};{x}H", x = x, y = y);
}

impl ScreenBuffer {
    pub fn write(&mut self, x: usize, y: usize, content: ScreenCell) -> &mut Self {
        if x < BUFFER_WIDTH && y < BUFFER_HEIGHT {
            self.buffer[y][x] = content;
        }

        self
    }

    pub fn write_string(&mut self, x: usize, y: usize, content: &str, colour: Colour) -> &mut Self {
        for (i, c) in content.chars().enumerate() {
            self.write(x + i, y, ScreenCell::new(c, colour));
        }

        self
    }

    pub fn print(self) {
        move_cursor(0, 0);

        let s = self
            .buffer
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|cell| format!("{}", cell))
                    .reduce(|acc, v| acc + &v)
                    .unwrap()
            })
            .reduce(|acc, v| format!("{}\n{}", acc, v));

        print!("{}\n", s.unwrap());
    }
}

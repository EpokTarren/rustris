use core::{Board, Colour, Piece};
use std::fmt::Display;

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

impl From<&Board> for ScreenBuffer {
    fn from(board: &Board) -> Self {
        let mut buf = ScreenBuffer::default();

        fn draw_piece(buf: &mut ScreenBuffer, piece: Piece, offset_x: i8, offset_y: i8) {
            let blocks = piece.blocks();

            for y in 0..4 {
                for x in 0..4 {
                    let c = blocks[y][x];

                    if c != Colour::None {
                        let x = x as i8 + offset_x;
                        let y = y as i8 + offset_y;

                        buf.write(x as usize, y as usize, ScreenCell::new('@', c));
                    }
                }
            }
        }

        const HOLD_LEFT: usize = 2;
        const HOLD_TOP: usize = 3;

        buf.write_string(HOLD_LEFT, HOLD_TOP - 1, "Hold", Colour::White);

        for y in HOLD_TOP..=(HOLD_TOP + 5) {
            buf.write(HOLD_LEFT, y, ScreenCell::new('#', Colour::Grey));
            buf.write(HOLD_LEFT + 7, y, ScreenCell::new('#', Colour::Grey));
        }

        if let Some(piece) = board.held() {
            draw_piece(&mut buf, piece, (HOLD_TOP + 1) as i8, (HOLD_LEFT + 2) as i8);
        }

        const HOLD_BOTTOM: &str = "######";

        buf.write_string(HOLD_LEFT + 1, HOLD_TOP, HOLD_BOTTOM, Colour::Grey);
        buf.write_string(HOLD_LEFT + 1, HOLD_TOP + 5, HOLD_BOTTOM, Colour::Grey);

        const PLAY_FIELD_BOTTOM: &str = "##########";
        const PLAY_FIELD_LEFT: usize = HOLD_LEFT + 7 + 3;

        for y in 2..23 {
            buf.write(PLAY_FIELD_LEFT - 1, y, ScreenCell::new('#', Colour::Grey));
            buf.write(
                PLAY_FIELD_LEFT + Board::WIDTH,
                y,
                ScreenCell::new('#', Colour::Grey),
            );
        }

        buf.write_string(PLAY_FIELD_LEFT, 22, PLAY_FIELD_BOTTOM, Colour::Grey);

        let blocks = board.blocks();

        for y in 0..Board::HEIGHT {
            for x in 0..Board::WIDTH {
                let colour = blocks[y][x];
                let y = y.wrapping_sub(23);

                if y < Board::HEIGHT && colour != Colour::None {
                    buf.write(x + PLAY_FIELD_LEFT, y, ScreenCell::new('@', colour));
                }
            }
        }

        let (piece, position) = board.piece();

        draw_piece(
            &mut buf,
            piece,
            position.x() + PLAY_FIELD_LEFT as i8,
            position.y().wrapping_sub(23),
        );

        const NEXT_LEFT: usize = PLAY_FIELD_LEFT + Board::WIDTH + 4;
        const NEXT_TOP: usize = 3;

        buf.write_string(NEXT_LEFT, NEXT_TOP - 1, "Next", Colour::White);

        for y in NEXT_TOP..=(NEXT_TOP + 11) {
            buf.write(NEXT_LEFT, y, ScreenCell::new('#', Colour::Grey));
            buf.write(NEXT_LEFT + 7, y, ScreenCell::new('#', Colour::Grey));
        }

        for i in 0..3 {
            draw_piece(
                &mut buf,
                board.peek(i),
                (NEXT_LEFT + 2) as i8,
                (NEXT_TOP + 1 + i * 3) as i8,
            );
        }

        const NEXT_BOTTOM: &str = "######";

        buf.write_string(NEXT_LEFT + 1, NEXT_TOP, NEXT_BOTTOM, Colour::Grey);
        buf.write_string(NEXT_LEFT + 1, NEXT_TOP + 11, NEXT_BOTTOM, Colour::Grey);

        buf
    }
}

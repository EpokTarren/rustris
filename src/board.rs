use crate::{
    display::{Colour, ScreenBuffer, ScreenCell},
    kicks::{I_KICKS, KICKS},
    piece::{Piece, PieceType},
    point::Point,
};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 45;
const BOARD_SIZE: usize = BOARD_WIDTH * BOARD_HEIGHT;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct Bag {
    i: usize,
}

impl Bag {
    pub fn next(&mut self) -> Piece {
        const KINDS: [PieceType; 7] = [
            PieceType::I,
            PieceType::J,
            PieceType::L,
            PieceType::O,
            PieceType::S,
            PieceType::T,
            PieceType::Z,
        ];

        let t = KINDS[self.i % 7];

        self.i += 1;

        Piece::new(t)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputDirection {
    None,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputRotation {
    None,
    Quarter,
    TwoQuarter,
    ThreeQuarter,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Input {
    pub direction: InputDirection,
    pub hard_drop: bool,
    pub rotation: InputRotation,
    pub soft_drop: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board {
    bag: Bag,
    board: [Colour; BOARD_SIZE],
    piece: Piece,
    position: Point,
}

impl Default for Board {
    fn default() -> Self {
        let mut bag: Bag = Default::default();

        let piece = bag.next();

        Self {
            bag,
            board: [Colour::None; BOARD_SIZE],
            piece,
            position: Self::START_POSITION,
        }
    }
}

impl Board {
    const START_POSITION: Point = Point::new(3, (BOARD_HEIGHT - 21) as i8);

    fn legal_position(&self, piece: Piece, position: Point) -> bool {
        for (i, c) in piece.blocks().into_iter().enumerate() {
            let x = (i as i8) % 4 + position.x();
            let y = ((i as i8) - x) / 4 + position.y();

            if c != Colour::None {
                if x < 0 || x >= BOARD_WIDTH as i8 || y < 0 || y >= BOARD_HEIGHT as i8 {
                    return false;
                }

                if self.board[x as usize + (y as usize) * BOARD_WIDTH] != Colour::None {
                    return false;
                }
            }
        }

        true
    }

    fn move_piece(&mut self, direction: InputDirection) {
        let delta = match direction {
            InputDirection::None => 0,
            InputDirection::Left => -1,
            InputDirection::Right => 1,
        };

        let position = self.position + Point::new(delta, 0);

        if self.legal_position(self.piece, position) {
            self.position = position;
        }
    }

    fn rotate_piece(&mut self, direction: InputRotation) {
        let delta: u8 = match direction {
            InputRotation::None => {
                return;
            }
            InputRotation::TwoQuarter => {
                return;
            }
            InputRotation::Quarter => 0,
            InputRotation::ThreeQuarter => 1,
        };

        let piece = self.piece.rotate(delta);
        let delta = delta as usize;

        let mut process_kick = |table: [[[Point; 5]; 2]; 4]| {
            for kick in table[self.piece.rotation() as usize][delta] {
                let position = self.position + kick;

                if self.legal_position(piece, position) {
                    self.position = position;
                    self.piece = piece;
                    return;
                }
            }
        };

        match piece.kind() {
            PieceType::O => {}
            PieceType::I => process_kick(I_KICKS),
            _ => process_kick(KICKS),
        }
    }

    fn soft_drop(&mut self) {
        let position = self.position + Point::new(0, 1);

        if self.legal_position(self.piece, position) {
            self.position = position;
        }
    }

    fn next_piece(&mut self) {
        for (i, c) in self.piece.blocks().into_iter().enumerate() {
            let x = (i as i8) % 4 + self.position.x();
            let y = ((i as i8) - x) / 4 + self.position.y();

            if c != Colour::None {
                self.board[x as usize + y as usize * BOARD_WIDTH] = c;
            }
        }

        self.piece = self.bag.next();
        self.position = Self::START_POSITION;
    }

    fn hard_drop(&mut self) {
        let mut position = self.position;
        self.soft_drop();

        while position != self.position {
            position = self.position;
            self.soft_drop()
        }

        self.next_piece();
    }

    pub fn tick(&mut self, input: Input) {
        self.move_piece(input.direction);
        self.rotate_piece(input.rotation);

        if input.hard_drop {
            self.hard_drop();
        } else if input.soft_drop {
            self.soft_drop();
        }
    }

    pub fn to_screen_buffer(&self) -> ScreenBuffer {
        let mut buf: ScreenBuffer = Default::default();

        for y in 2..23 {
            buf.write(2, y, ScreenCell::new('#', Colour::Grey));
            buf.write(13, y, ScreenCell::new('#', Colour::Grey));
        }

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let colour = self.board[x + y * BOARD_WIDTH];
                let y = y.wrapping_sub(23);

                if y < BOARD_HEIGHT && colour != Colour::None {
                    buf.write(x + 3, y, ScreenCell::new('@', colour));
                }
            }
        }

        for (i, c) in self.piece.blocks().into_iter().enumerate() {
            let x = (i as i8) % 4 + self.position.x() + 3;
            let y = (((i as i8) - x) / 4 + self.position.y()).wrapping_sub(22);

            if c != Colour::None {
                buf.write(x as usize, y as usize, ScreenCell::new('@', c));
            }
        }

        const BOTTOM: &str = "##########";

        buf.write_string(3, 22, BOTTOM, Colour::Grey);

        buf
    }
}

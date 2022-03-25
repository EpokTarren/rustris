use crate::{
    display::{Colour, ScreenBuffer, ScreenCell},
    input::{Input, InputDirection, InputRotation},
    kicks::{I_KICKS, KICKS},
    piece::{Piece, PieceType},
    point::Point,
};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 45;

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
pub struct Board {
    bag: Bag,
    board: [[Colour; BOARD_WIDTH]; BOARD_HEIGHT],
    piece: Piece,
    contact: u8,
    position: Point,
}

impl Default for Board {
    fn default() -> Self {
        let mut bag: Bag = Default::default();

        let piece = bag.next();

        Self {
            bag,
            board: [[Colour::None; BOARD_WIDTH]; BOARD_HEIGHT],
            piece,
            contact: 0,
            position: Self::START_POSITION,
        }
    }
}

impl Board {
    const START_POSITION: Point = Point::new(3, (BOARD_HEIGHT - 21) as i8);

    fn legal_position(&self, piece: Piece, position: Point) -> bool {
        let blocks = piece.blocks();

        for y in 0..4 {
            for x in 0..4 {
                let c = blocks[y][x];

                if c != Colour::None {
                    let x = x as i8 + position.x();
                    let y = y as i8 + position.y();

                    if x < 0 || x >= BOARD_WIDTH as i8 || y < 0 || y >= BOARD_HEIGHT as i8 {
                        return false;
                    }

                    if self.board[y as usize][x as usize] != Colour::None {
                        return false;
                    }
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

        let piece = self.piece.rotate(match direction {
            InputRotation::None => 0,
            InputRotation::Quarter => 1,
            InputRotation::TwoQuarter => 2,
            InputRotation::ThreeQuarter => 3,
        });

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

    fn test_soft_drop(&self) -> bool {
        let position = self.position + Point::new(0, 1);

        if self.legal_position(self.piece, position) {
            true
        } else {
            false
        }
    }

    fn soft_drop(&mut self) {
        let position = self.position + Point::new(0, 1);

        if self.legal_position(self.piece, position) {
            self.position = position;
        }
    }

    fn next_piece(&mut self) -> usize {
        let blocks = self.piece.blocks();

        for y in 0..4 {
            for x in 0..4 {
                let c = blocks[y][x];

                if c != Colour::None {
                    let x = x as i8 + self.position.x();
                    let y = y as i8 + self.position.y();
                    self.board[y as usize][x as usize] = c;
                }
            }
        }

        self.piece = self.bag.next();
        self.position = Self::START_POSITION;

        0
    }

    fn hard_drop(&mut self) -> usize {
        while self.test_soft_drop() {
            self.soft_drop();
        }

        self.next_piece()
    }

    fn input(&mut self, input: Input) -> usize {
        self.move_piece(input.direction);
        self.rotate_piece(input.rotation);

        if input.hard_drop {
            self.hard_drop()
        } else if input.soft_drop {
            self.soft_drop();
            0
        } else {
            0
        }
    }

    pub fn tick(&mut self, input: Input, tick: u128) -> usize {
        self.input(input);

        if tick % 500 == 0 {
            self.soft_drop();
        }

        if tick % 50 == 0 {
            if !self.test_soft_drop() {
                self.contact += 1;
            } else {
                self.contact = 0;
            }

            if self.contact == 30 {
                self.next_piece()
            } else {
                0
            }
        } else {
            0
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
                let colour = self.board[y][x];
                let y = y.wrapping_sub(23);

                if y < BOARD_HEIGHT && colour != Colour::None {
                    buf.write(x + 3, y, ScreenCell::new('@', colour));
                }
            }
        }

        let blocks = self.piece.blocks();

        for y in 0..4 {
            for x in 0..4 {
                let c = blocks[y][x];

                if c != Colour::None {
                    let x = x as i8 + self.position.x() + 3;
                    let y = y as i8 + self.position.y().wrapping_sub(23);

                    buf.write(x as usize, y as usize, ScreenCell::new('@', c));
                }
            }
        }

        const BOTTOM: &str = "##########";

        buf.write_string(3, 22, BOTTOM, Colour::Grey);

        buf
    }
}

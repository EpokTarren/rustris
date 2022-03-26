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
    fn nth(i: usize) -> Piece {
        const KINDS: [PieceType; 7] = [
            PieceType::I,
            PieceType::J,
            PieceType::L,
            PieceType::O,
            PieceType::S,
            PieceType::T,
            PieceType::Z,
        ];

        let t = KINDS[i % 7];

        Piece::new(t)
    }

    pub fn next(&mut self) -> Piece {
        let piece = Self::nth(self.i);

        self.i += 1;

        piece
    }

    pub fn peek(&self, i: usize) -> Piece {
        Self::nth(self.i + i)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TickResult {
    None,
    One,
    Two,
    Three,
    Four,
    GameOver,
    Spin(PieceType, u8),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Board {
    bag: Bag,
    board: [[Colour; BOARD_WIDTH]; BOARD_HEIGHT],
    held: Option<Piece>,
    piece: Piece,
    contact: u8,
    may_hold: bool,
    position: Point,
}

impl Default for Board {
    fn default() -> Self {
        let mut bag: Bag = Default::default();

        let piece = bag.next();

        Self {
            bag,
            board: [[Colour::None; BOARD_WIDTH]; BOARD_HEIGHT],
            held: None,
            piece,
            contact: 0,
            may_hold: true,
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

    fn next_piece(&mut self) -> TickResult {
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
        self.may_hold = true;
        self.position = Self::START_POSITION;

        if self.legal_position(self.piece, self.position) {
            let mut cleared_indexes: Vec<usize> = Vec::with_capacity(4);

            for y in (0..BOARD_HEIGHT).rev() {
                if self.board[y]
                    .iter()
                    .map(|tile| *tile != Colour::None)
                    .reduce(|acc, tile| acc && tile)
                    .unwrap()
                {
                    cleared_indexes.push(y);
                }
            }

            let cleared = cleared_indexes.len();

            while let Some(i) = cleared_indexes.pop() {
                for y in (1..=i).rev() {
                    self.board[y] = self.board[y - 1];
                }

                self.board[0] = [Colour::None; BOARD_WIDTH];
            }

            match cleared {
                0 => TickResult::None,
                1 => TickResult::One,
                2 => TickResult::Two,
                3 => TickResult::Three,
                4 => TickResult::Four,
                _ => unreachable!("It should be impossible to clear outside the range of 0-4"),
            }
        } else {
            TickResult::GameOver
        }
    }

    fn hard_drop(&mut self) -> TickResult {
        while self.test_soft_drop() {
            self.soft_drop();
        }

        self.next_piece()
    }

    fn hold(&mut self) {
        if self.may_hold {
            if let Some(held) = self.held {
                self.held = Some(Piece::new(self.piece.kind()));
                self.piece = Piece::new(held.kind());
            } else {
                self.held = Some(Piece::new(self.piece.kind()));
                self.piece = self.bag.next();
            }

            self.position = Self::START_POSITION;
            self.may_hold = false
        }
    }

    fn input(&mut self, input: Input) -> TickResult {
        self.move_piece(input.direction);
        self.rotate_piece(input.rotation);

        if input.hold {
            self.hold();
        }

        if input.hard_drop {
            return self.hard_drop();
        }

        if input.soft_drop {
            self.soft_drop();
        }

        TickResult::None
    }

    pub fn tick(&mut self, input: Input, tick: u128) -> TickResult {
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
                return self.next_piece();
            }
        }

        self.input(input)
    }

    pub fn to_screen_buffer(&self) -> ScreenBuffer {
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

        if let Some(piece) = self.held {
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
                PLAY_FIELD_LEFT + BOARD_WIDTH,
                y,
                ScreenCell::new('#', Colour::Grey),
            );
        }

        buf.write_string(PLAY_FIELD_LEFT, 22, PLAY_FIELD_BOTTOM, Colour::Grey);

        for y in 0..BOARD_HEIGHT {
            for x in 0..BOARD_WIDTH {
                let colour = self.board[y][x];
                let y = y.wrapping_sub(23);

                if y < BOARD_HEIGHT && colour != Colour::None {
                    buf.write(x + PLAY_FIELD_LEFT, y, ScreenCell::new('@', colour));
                }
            }
        }

        draw_piece(
            &mut buf,
            self.piece,
            self.position.x() + PLAY_FIELD_LEFT as i8,
            self.position.y().wrapping_sub(23),
        );

        const NEXT_LEFT: usize = PLAY_FIELD_LEFT + BOARD_WIDTH + 4;
        const NEXT_TOP: usize = 3;

        buf.write_string(NEXT_LEFT, NEXT_TOP - 1, "Next", Colour::White);

        for y in NEXT_TOP..=(NEXT_TOP + 11) {
            buf.write(NEXT_LEFT, y, ScreenCell::new('#', Colour::Grey));
            buf.write(NEXT_LEFT + 7, y, ScreenCell::new('#', Colour::Grey));
        }

        for i in 0..3 {
            draw_piece(
                &mut buf,
                self.bag.peek(i),
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

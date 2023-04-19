use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    bag::Bag,
    colour::Colour,
    input::{Input, InputDirection, InputRotation},
    kicks::{I_KICKS, KICKS},
    piece::{Piece, PieceType},
    point::Point,
};

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 45;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub struct TickResult {
    kind: TickType,
    piece: PieceType,
    lines: u8,
}

impl TickResult {
    pub(crate) fn new(kind: TickType, piece: PieceType, lines: u8) -> Self {
        Self { kind, piece, lines }
    }
}

#[wasm_bindgen]
impl TickResult {
    pub fn kind(&self) -> TickType {
        self.kind
    }

    pub fn piece(&self) -> PieceType {
        self.piece
    }

    pub fn lines(&self) -> u8 {
        self.lines
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub enum TickType {
    None,
    Clear,
    Spin,
    GameOver,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub struct Board {
    bag: Bag,
    board: [[Colour; BOARD_WIDTH]; BOARD_HEIGHT],
    held: Option<Piece>,
    piece: Piece,
    contact: u8,
    may_hold: bool,
    position: Point,
    last_input_rot: bool,
}

impl Board {
    pub const WIDTH: usize = BOARD_WIDTH;
    pub const HEIGHT: usize = BOARD_HEIGHT;
    const START_POSITION: Point = Point::constant(3, (BOARD_HEIGHT - 21) as i8);

    pub fn blocks(&self) -> &[[Colour; BOARD_WIDTH]; BOARD_HEIGHT] {
        &self.board
    }

    pub fn mirror(&self) -> Self {
        let mut board = self.clone();

        for (i, line) in board.board.iter_mut().enumerate() {
            for (j, block) in line.iter_mut().rev().enumerate() {
                *block = self.board[i][j];
            }
        }

        board
    }

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
            InputDirection::None => {
                return;
            }
            InputDirection::Left => -1,
            InputDirection::Right => 1,
            InputDirection::SnapLeft => -10,
            InputDirection::SnapRight => 10,
        };

        let d = i8::signum(delta);

        for _ in 0..delta.abs() {
            let position = self.position + Point::new(d, 0);

            if self.legal_position(self.piece, position) {
                self.position = position;
                self.last_input_rot = false;
            }
        }
    }

    fn rotate_180(&mut self) {
        use crate::kicks::KICKS_180;

        let piece = self.piece.rotate(2);

        for kick in KICKS_180[self.piece.rotation() as usize] {
            let position = self.position + kick;

            if self.legal_position(piece, position) {
                self.position = position;
                self.piece = piece;
                self.last_input_rot = true;
                return;
            }
        }
    }

    fn rotate_piece(&mut self, direction: InputRotation) {
        let (delta, rotation) = match direction {
            InputRotation::None => return,
            InputRotation::TwoQuarter => return self.rotate_180(),
            InputRotation::Quarter => (0, 1),
            InputRotation::ThreeQuarter => (1, 3),
        };

        let piece = self.piece.rotate(rotation);
        let mut process_kick = |table: [[[Point; 5]; 2]; 4]| {
            for kick in table[self.piece.rotation() as usize][delta] {
                let position = self.position + kick;

                if self.legal_position(piece, position) {
                    self.position = position;
                    self.piece = piece;
                    self.last_input_rot = true;
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
            self.last_input_rot = false;
            self.position = position;
        }
    }

    fn next_piece(&mut self) -> TickResult {
        let spin = if self.last_input_rot {
            match self.piece.kind() {
                PieceType::I | PieceType::O => false,

                PieceType::J | PieceType::L | PieceType::S | PieceType::Z => {
                    let mut blocked_sides = 0;
                    let blocks = self.piece.blocks();

                    for y in 0..4 {
                        for x in 0..4 {
                            let c = blocks[y][x];

                            if c != Colour::None {
                                let x = x as i8 + self.position.x();
                                let y = y as i8 + self.position.y();
                                if x == 0
                                    || x >= (BOARD_WIDTH - 1) as i8
                                    || self.board[y as usize][(x - 1) as usize] != Colour::None
                                    || self.board[y as usize][(x + 1) as usize] != Colour::None
                                {
                                    blocked_sides += 1;
                                }
                            }
                        }
                    }

                    blocked_sides == 4
                }

                PieceType::T => {
                    const T_OFFSETS: [Point; 4] = [
                        Point::constant(1, -1),
                        Point::constant(1, 1),
                        Point::constant(-1, 1),
                        Point::constant(-1, -1),
                    ];

                    let mut blocked_corners = 0;
                    let center = self.position + Point::new(1, 2);

                    for offset in T_OFFSETS {
                        let corner = center + offset;
                        let (x, y) = (corner.x() as usize, corner.y() as usize);

                        if x < BOARD_WIDTH && y < BOARD_HEIGHT {
                            blocked_corners += (self.board[y][x] != Colour::None) as u8;
                        } else {
                            blocked_corners += 1;
                        }
                    }

                    blocked_corners > 2
                }
            }
        } else {
            false
        };

        let piece = self.piece;
        let blocks = piece.blocks();

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

        self.piece = Piece::new(self.bag.next());
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

            let cleared = cleared_indexes.len() as u8;

            while let Some(i) = cleared_indexes.pop() {
                for y in (1..=i).rev() {
                    self.board[y] = self.board[y - 1];
                }

                self.board[0] = [Colour::None; BOARD_WIDTH];
            }

            if spin {
                return match cleared {
                    0 | 1 | 2 | 3 => TickResult {
                        kind: TickType::Spin,
                        piece: piece.kind(),
                        lines: cleared,
                    },
                    _ => unreachable!("It should be impossible to clear outside the range of 0-4"),
                };
            }

            match cleared {
                0 => TickResult {
                    kind: TickType::None,
                    piece: piece.kind(),
                    lines: 0,
                },

                1 | 2 | 3 | 4 => TickResult {
                    kind: TickType::Clear,
                    piece: piece.kind(),
                    lines: cleared,
                },
                _ => unreachable!("It should be impossible to clear outside the range of 0-4"),
            }
        } else {
            TickResult {
                kind: TickType::GameOver,
                piece: piece.kind(),
                lines: 0,
            }
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
                self.piece = Piece::new(self.bag.next());
            }

            self.position = Self::START_POSITION;
            self.may_hold = false
        }
    }

    fn input(&mut self, input: Input) -> TickResult {
        if input.quit {
            return TickResult {
                kind: TickType::GameOver,
                piece: self.piece.kind(),
                lines: 0,
            };
        }

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

        TickResult {
            kind: TickType::None,
            piece: self.piece.kind(),
            lines: 0,
        }
    }

    #[inline(always)]
    fn tick_inner(&mut self, input: Input, tick: u128) -> TickResult {
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

    #[cfg(not(target_arch = "wasm32"))]
    pub fn tick(&mut self, input: Input, tick: u128) -> TickResult {
        self.tick_inner(input, tick)
    }

    pub fn from_strs(rows: &[&str], mut bag: Bag) -> Self {
        let piece = Piece::new(bag.next());

        let mut board = [[Colour::None; BOARD_WIDTH]; BOARD_HEIGHT];

        if rows.len() >= BOARD_HEIGHT {
            panic!("Provided board is too tall");
        }

        let offset = BOARD_HEIGHT - rows.len();

        for (y, row) in rows.iter().enumerate() {
            if row.len() != BOARD_WIDTH {
                panic!(
                    "Provided row is of wrong length, expected {} got {}",
                    BOARD_WIDTH,
                    row.len()
                );
            }

            for (x, c) in row.chars().enumerate() {
                if c != ' ' {
                    board[y + offset][x] = Colour::Grey;
                }
            }
        }

        Self {
            bag,
            held: None,
            piece,
            board,
            contact: 0,
            may_hold: true,
            position: Self::START_POSITION,
            last_input_rot: false,
        }
    }

    pub fn from_position(
        board: [[Colour; BOARD_WIDTH]; BOARD_HEIGHT],
        bag: Bag,
        piece: Piece,
        held: Option<Piece>,
    ) -> Self {
        Self {
            bag,
            held,
            piece,
            board,
            contact: 0,
            may_hold: true,
            position: Self::START_POSITION,
            last_input_rot: false,
        }
    }
}

#[wasm_bindgen]
impl Board {
    pub fn new(mut bag: Bag) -> Self {
        let piece = Piece::new(bag.next());

        Self {
            bag,
            held: None,
            piece,
            board: [[Colour::None; BOARD_WIDTH]; BOARD_HEIGHT],
            contact: 0,
            may_hold: true,
            position: Self::START_POSITION,
            last_input_rot: false,
        }
    }

    pub fn from_string(board: String, bag: Bag) -> Self {
        Self::from_strs(board.lines().collect::<Vec<&str>>().as_slice(), bag)
    }

    pub fn bag(&self) -> Bag {
        self.bag.clone()
    }

    pub fn block(&self, x: usize, y: usize) -> Colour {
        self.board[y][x]
    }

    pub fn piece(&self) -> Piece {
        self.piece
    }

    pub fn position(&self) -> Point {
        self.position
    }

    pub fn held(&self) -> Option<Piece> {
        self.held
    }

    pub fn peek(&self, i: usize) -> Piece {
        Piece::new(self.bag.peek(i))
    }

    #[cfg(target_arch = "wasm32")]
    pub fn tick(&mut self, input: Input, tick: u64) -> TickResult {
        self.tick_inner(input, tick as u128)
    }

    #[cfg(target_arch = "wasm32")]
    pub fn height() -> usize {
        Self::HEIGHT
    }

    #[cfg(target_arch = "wasm32")]
    pub fn width() -> usize {
        Self::WIDTH
    }
}

// These are debug functions intended to make setting up a board easier
#[allow(dead_code)]
impl Board {
    pub(crate) fn from_strs_with_piece(rows: &[&str], bag: Bag, piece: Piece) -> Self {
        let mut board = Self::from_strs(rows, bag);
        board.piece = piece;

        board
    }
}

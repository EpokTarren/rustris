use std::fmt::Display;

use wasm_bindgen::prelude::wasm_bindgen;

use crate::{point::Point, Bag, Board, Colour, Input, Piece, Score, TickResult, TickType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
#[repr(u8)]
pub enum GameMode {
    Marathon = 0,
    Lines = 1,
    Time = 2,
}

impl GameMode {
    pub(crate) fn new(t: u8) -> Result<Self, ()> {
        match t {
            0 => Ok(Self::Marathon),
            1 => Ok(Self::Lines),
            2 => Ok(Self::Time),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub struct GameType {
    mode: GameMode,
    count: u64,
}

#[wasm_bindgen]
impl GameType {
    pub fn new(mode: GameMode, count: u64) -> Self {
        Self { mode, count }
    }

    pub fn new_marathon() -> Self {
        Self {
            mode: GameMode::Marathon,
            count: 0,
        }
    }

    pub fn new_lines(lines: u64) -> Self {
        Self {
            mode: GameMode::Marathon,
            count: lines,
        }
    }

    pub fn new_timed(seconds: u64) -> Self {
        Self {
            mode: GameMode::Marathon,
            count: seconds,
        }
    }

    pub fn lines(&self) -> u64 {
        self.count
    }

    pub fn time(&self) -> u64 {
        self.count
    }

    pub fn mode(&self) -> GameMode {
        self.mode
    }
}

impl Display for GameType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.mode {
            GameMode::Marathon => write!(f, "Marathon"),
            GameMode::Lines => write!(f, "{}L", self.count),
            GameMode::Time => write!(f, "{}S", self.count),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub struct Game {
    board: Board,
    score: Score,
    kind: GameType,
}

impl Game {
    pub fn blocks(&self) -> &[[Colour; Board::WIDTH]; Board::HEIGHT] {
        self.board.blocks()
    }

    pub fn board(&self) -> &Board {
        &self.board
    }

    fn process_tick(&mut self, res: TickResult, tick: u128) -> TickResult {
        if res.kind() == TickType::GameOver {
            return res;
        }

        self.score.update(res);

        match self.kind.mode() {
            GameMode::Time => {
                if tick / 1000 >= self.kind.count as u128 {
                    TickResult::new(TickType::GameOver, res.piece(), res.lines())
                } else {
                    res
                }
            }
            GameMode::Lines => {
                if self.score.lines() >= self.kind.count {
                    TickResult::new(TickType::GameOver, res.piece(), res.lines())
                } else {
                    res
                }
            }
            GameMode::Marathon => res,
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    pub fn tick(&mut self, input: Input, tick: u128) -> TickResult {
        let res = self.board.tick(input, tick);
        self.process_tick(res, tick)
    }
}

#[wasm_bindgen]
impl Game {
    pub fn new(bag: Bag, kind: GameType) -> Self {
        Self {
            board: Board::new(bag),
            score: Score::new(),
            kind,
        }
    }

    pub fn block(&self, x: usize, y: usize) -> Colour {
        self.board.block(x, y)
    }

    pub fn piece(&self) -> Piece {
        self.board.piece()
    }

    pub fn position(&self) -> Point {
        self.board.position()
    }

    pub fn held(&self) -> Option<Piece> {
        self.board.held()
    }

    pub fn peek(&self, i: usize) -> Piece {
        self.board.peek(i)
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn kind(&self) -> GameType {
        self.kind
    }

    #[cfg(target_arch = "wasm32")]
    pub fn tick(&mut self, input: Input, tick: u64) -> TickResult {
        let res = self.board.tick(input, tick);
        self.process_tick(res, tick as u128)
    }
}

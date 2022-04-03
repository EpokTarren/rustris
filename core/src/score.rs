use wasm_bindgen::prelude::wasm_bindgen;

use crate::{
    board::{TickResult, TickType},
    piece::PieceType,
};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub struct Score {
    pub(crate) score: u64,
    pub(crate) lines: u64,
}

#[wasm_bindgen]
impl Score {
    pub fn new() -> Self {
        Self { score: 0, lines: 0 }
    }

    pub fn score(&self) -> u64 {
        self.score
    }

    pub fn lines(&self) -> u64 {
        self.lines
    }

    pub fn update(&mut self, tick: TickResult) {
        let (lines, score) = match tick.kind() {
            TickType::None => (0, 0),
            TickType::Clear => match tick.lines() {
                1 => (1, 100),
                2 => (2, 300),
                3 => (3, 500),
                4 => (4, 800),
                _ => unreachable!("Only clears of 1-4 lines are possible"),
            },

            TickType::Spin => match tick.piece() {
                PieceType::T => match tick.lines() {
                    0 => (0, 100),
                    1 => (1, 800),
                    2 => (2, 1200),
                    3 => (3, 1600),
                    _ => unreachable!("Only spins of 0-3 lines are possible"),
                },

                _ => match tick.lines() {
                    0 => (0, 0),
                    1 => (1, 100),
                    2 => (2, 300),
                    3 => (3, 500),
                    _ => unreachable!("Only spins of 0-3 lines are possible"),
                },
            },

            TickType::GameOver => unreachable!("Game should terminate before this"),
        };

        self.score += score;
        self.lines += lines;
    }
}

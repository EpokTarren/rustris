use crate::{board::TickResult, piece::PieceType};

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Score {
    score: u128,
    lines: u128,
}

impl Score {
    pub const fn new() -> Self {
        Self { score: 0, lines: 0 }
    }

    pub const fn score(&self) -> u128 {
        self.score
    }

    pub const fn lines(&self) -> u128 {
        self.lines
    }

    pub fn update(&mut self, tick: TickResult) {
        let (lines, score) = match tick {
            TickResult::None => (0, 0),
            TickResult::One => (1, 100),
            TickResult::Two => (2, 300),
            TickResult::Three => (3, 500),
            TickResult::Four => (4, 800),

            TickResult::Spin(kind, lines) => match kind {
                PieceType::T => match lines {
                    0 => (0, 100),
                    1 => (1, 800),
                    2 => (2, 1200),
                    3 => (3, 1600),
                    _ => unreachable!("Only spins of 0-3 lines are possible"),
                },

                _ => match lines {
                    0 => (0, 0),
                    1 => (1, 100),
                    2 => (2, 300),
                    3 => (3, 500),
                    _ => unreachable!("Only spins of 0-3 lines are possible"),
                },
            },

            TickResult::GameOver => unreachable!("Game should terminate before this"),
        };

        self.score += score;
        self.lines += lines;
    }
}

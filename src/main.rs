use std::time::Instant;

use crate::{
    board::{Board, TickResult},
    config::Config,
    display::Colour,
    input::Input,
};

mod board;
mod config;
mod display;
mod get_key;
mod input;
mod kicks;
mod piece;
mod point;

fn main() {
    let folder = Config::folder();
    let conf_file = folder.clone() + "config";
    let conf = Config::from_file(&conf_file);

    let start = Instant::now();

    let mut board = Board::default();
    let mut input = Input::default();
    let mut last_update: u128 = 0;
    let mut score: u128 = 0;
    let mut lines: u128 = 0;

    display::clear_terminal();

    loop {
        if let Some(c) = get_key::get_key() {
            let c = c.to_ascii_lowercase();

            if c == conf.quit {
                break;
            }

            input.update(c, conf);
        }

        let now = start.elapsed().as_millis();

        if now != last_update {
            let tick = board.tick(input, now);
            let (lines_cleared, score_increase) = match tick {
                TickResult::None => (0, 0),
                TickResult::One => (1, 100),
                TickResult::Two => (2, 300),
                TickResult::Three => (3, 500),
                TickResult::Four => (4, 800),

                TickResult::Spin(kind, lines) => match kind {
                    piece::PieceType::T => match lines {
                        1 => (1, 800),
                        2 => (2, 1200),
                        3 => (3, 1600),
                        _ => unreachable!("Only spins of 1-3 lines are possible"),
                    },

                    _ => match lines {
                        1 => (1, 100),
                        2 => (2, 300),
                        3 => (3, 500),
                        _ => unreachable!("Only spins of 1-3 lines are possible"),
                    },
                },

                TickResult::GameOver => break,
            };

            lines += lines_cleared;
            score += score_increase;

            input = Input::default();

            last_update = now;

            if now % 50 == 0 {
                board
                    .to_screen_buffer()
                    .write_string(26, 16, &format!("Score: {}", score), Colour::White)
                    .write_string(26, 18, &format!("Lines: {}", lines), Colour::White)
                    .print();
            }
        }
    }

    println!("Thanks for playing")
}

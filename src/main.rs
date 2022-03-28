use std::time::{Duration, Instant};

use rand::{RngCore, SeedableRng};

use crate::{
    bag::Bag,
    board::{Board, TickResult},
    config::Config,
    display::Colour,
    score::Score,
};

mod bag;
mod board;
mod config;
mod display;
mod get_input;
mod input;
mod kicks;
mod piece;
mod point;
mod score;

fn time_format(duration: Duration) -> String {
    let ms = duration.as_millis() % 1000;
    let s = duration.as_secs() % 60;
    let m = duration.as_secs() / 60;

    format!("Time: {:0w$}:{:0w$}.{:0w_ms$}", m, s, ms, w = 2, w_ms = 3)
}

fn main() {
    let folder = Config::folder();
    let conf_file = folder.clone() + "config";
    let conf = Config::from_file(&conf_file);

    let start = Instant::now();
    let seed = {
        let mut seed: [u8; 32] = [0; 32];
        let mut rng = rand::rngs::SmallRng::from_entropy();

        rng.fill_bytes(&mut seed);

        seed
    };
    let bag = Bag::new(seed);

    let mut board = Board::new(bag);
    let mut last_update: u128 = 0;
    let mut score = Score::new();

    display::clear_terminal();

    'game_loop: loop {
        let duration = start.elapsed();
        let now = duration.as_millis();

        if now != last_update {
            let input = if let Ok(input) = get_input::get_input(conf) {
                input
            } else {
                break 'game_loop;
            };

            let tick = board.tick(input, now);

            if tick == TickResult::GameOver {
                break 'game_loop;
            } else {
                score.update(tick);
            }

            last_update = now;

            if now % conf.frame_time as u128 == 0 {
                board
                    .to_screen_buffer()
                    .write_string(26, 16, &format!("Score: {}", score.score()), Colour::White)
                    .write_string(26, 18, &format!("Lines: {}", score.lines()), Colour::White)
                    .write_string(26, 20, &time_format(duration), Colour::White)
                    .print();
            }
        }
    }

    println!("--------------------");
    println!(" Game Results");
    println!("--------------------");
    println!(" Score: {}", score.score());
    println!(" Lines: {}", score.lines());
    println!(" {}", time_format(start.elapsed()));
    println!("--------------------");
    println!(" Thanks for playing");
}

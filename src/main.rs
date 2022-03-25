use std::time::Instant;

use crate::{board::Board, config::Config, input::Input};

mod board;
mod config;
mod display;
mod get_key;
mod input;
mod kicks;
mod piece;
mod point;

fn main() {
    let mut board: Board = Default::default();

    let conf = Config::from_file(Config::PATH);

    let start = Instant::now();

    let mut input = Input::default();
    let mut last_update: u128 = 0;

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
            board.tick(input, now);

            input = Input::default();

            last_update = now;

            if now % 100 == 0 {
                let screen = board.to_screen_buffer();
                screen.print();
            }
        }
    }

    println!("Thanks for playing")
}

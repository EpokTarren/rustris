use std::time::Instant;

use crate::{
    board::{Board, Input, InputDirection, InputRotation},
    config::Config,
    display::ScreenBuffer,
};

mod board;
mod config;
mod display;
mod get_key;
mod kicks;
mod piece;
mod point;

fn main() {
    let mut board: Board = Default::default();

    let conf = Config::from_file(Config::PATH);

    let start = Instant::now();

    loop {
        if let Some(c) = get_key::get_key() {
            let mut input = Input {
                direction: InputDirection::None,
                hard_drop: false,
                rotation: InputRotation::None,
                soft_drop: false,
            };

            let c = c.to_ascii_lowercase();

            if c == 'q' {
                break;
            } else if c == conf.left {
                input.direction = InputDirection::Left;
            } else if c == conf.right {
                input.direction = InputDirection::Right;
            } else if c == conf.hold {
                todo!();
            } else if c == conf.rotate_90 {
                input.rotation = InputRotation::Quarter;
            } else if c == conf.rotate_180 {
                input.rotation = InputRotation::TwoQuarter;
            } else if c == conf.rotate_270 {
                input.rotation = InputRotation::ThreeQuarter;
            } else if c == conf.left {
                input.direction = InputDirection::Left;
            } else if c == conf.soft_drop {
                input.soft_drop = true;
            } else if c == conf.hard_drop {
                input.hard_drop = true;
            }

            board.input(input);
        }

        let duration = start.elapsed();

        if duration.as_millis() % 100 == 0 {
            let mut buf: ScreenBuffer = Default::default();

            board.tick();

            buf.join(board.to_screen_buffer());

            buf.print();
        }
    }

    println!("Thanks for playing")
}

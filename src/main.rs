use crate::{
    board::{Board, Input, InputDirection, InputRotation},
    display::ScreenBuffer,
};

mod board;
mod display;
mod get_key;
mod kicks;
mod piece;
mod point;

fn main() {
    let mut buf: ScreenBuffer = Default::default();

    let mut board: Board = Default::default();

    let input = Input {
        direction: InputDirection::Left,
        hard_drop: true,
        rotation: InputRotation::Quarter,
        soft_drop: true,
    };

    board.tick(input);

    buf.join(board.to_screen_buffer());

    buf.print();
}

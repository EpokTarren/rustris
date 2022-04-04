use crate::{Board, Colour, Input, InputDirection, InputRotation};

pub fn print_board(board: &Board) {
    const PLAY_FIELD_BOTTOM: &str = "############";
    const PLAY_FIELD_LEFT: usize = 1;

    let blocks = board.blocks();

    for y in 23..Board::HEIGHT {
        print!("#");
        for x in 0..Board::WIDTH {
            let colour = blocks[y][x];
            if colour != Colour::None {
                print!("\x1b[38;5;{}m@\x1b[0m", colour as u8);
            } else {
                print!(" ");
            }
        }
        println!("#");
    }

    println!("{}", PLAY_FIELD_BOTTOM);
}

const NONE: Input = Input {
    hold: false,
    quit: false,
    hard_drop: false,
    soft_drop: false,
    rotation: InputRotation::None,
    direction: InputDirection::None,
};

const LEFT: Input = {
    Input {
        direction: InputDirection::Left,
        ..NONE
    }
};

const RIGHT: Input = {
    Input {
        direction: InputDirection::Right,
        ..NONE
    }
};

const LEFT_SNAP: Input = {
    Input {
        direction: InputDirection::SnapLeft,
        ..NONE
    }
};

const RIGHT_SNAP: Input = {
    Input {
        direction: InputDirection::SnapRight,
        ..NONE
    }
};

const SOFT: Input = {
    Input {
        soft_drop: true,
        ..NONE
    }
};

const HARD: Input = {
    Input {
        hard_drop: true,
        ..NONE
    }
};

const COUNTER_CLOCKWISE: Input = {
    Input {
        rotation: InputRotation::ThreeQuarter,
        ..NONE
    }
};

const CLOCKWISE: Input = {
    Input {
        rotation: InputRotation::Quarter,
        ..NONE
    }
};

/*
 * l - left
 * r - right
 * L - left instant
 * R - right instant
 * c - clockwise rotation
 * C - counter clockwise rotation
 * s - soft drop
 * S - soft drop to bottom
 * H - hard drop
 */
pub fn spin_test(mut board: Board, expected: Board, inputs: &str) {
    let mut tick = 0;
    for input in inputs.chars() {
        let input = match input {
            'l' => LEFT,
            'r' => RIGHT,
            'L' => LEFT_SNAP,
            'R' => RIGHT_SNAP,
            'c' => CLOCKWISE,
            'C' => COUNTER_CLOCKWISE,
            's' => SOFT,
            'S' => {
                for i in tick..(tick + 30) {
                    board.tick(SOFT, i);
                }
                tick += 30;
                continue;
            }
            'H' => HARD,
            _ => panic!("Unknown input `{}`", input),
        };

        board.tick(input, tick);
        tick += 1;
    }

    let res = board.blocks() == expected.blocks();
    if !res {
        println!("Found");
        print_board(&board);
        println!("Expected");
        print_board(&expected);
    }

    assert!(res);
}

pub fn reverse_board(board: Board) -> Board {
    fn reverse(
        mut board: [[Colour; Board::WIDTH]; Board::HEIGHT],
    ) -> [[Colour; Board::WIDTH]; Board::HEIGHT] {
        for (y, row) in board.clone().iter().enumerate() {
            for (x, square) in row.iter().rev().enumerate() {
                board[y][x] = *square;
            }
        }

        board
    }

    Board::from_position(
        reverse(*board.blocks()),
        board.bag(),
        board.piece(),
        board.held(),
    )
}

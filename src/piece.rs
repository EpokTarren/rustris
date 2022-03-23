use crate::display::Colour;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
enum PieceType {
    I,
    J,
    L,
    O,
    S,
    T,
    Z,
}

impl PieceType {
    pub const fn colour(&self) -> Colour {
        match self {
            Self::I => Colour::Cyan,
            Self::J => Colour::Blue,
            Self::L => Colour::Orange,
            Self::O => Colour::Yellow,
            Self::S => Colour::Green,
            Self::T => Colour::Purple,
            Self::Z => Colour::Red,
        }
    }
}

const I: [[Colour; 16]; 4] = {
    let mut blocks = [Colour::None; 16];

    blocks[4] = Colour::Cyan;
    blocks[5] = Colour::Cyan;
    blocks[6] = Colour::Cyan;
    blocks[7] = Colour::Cyan;

    const fn rotate(blocks: [Colour; 16]) -> [Colour; 16] {
        /*
         * 0  1  2  3
         * 4  5  6  7
         * 8  9  10 11
         * 12 13 14 15
         *
         * Becomes
         *
         * 12 8  4  0
         * 13 9  5  1
         * 14 10 6  2
         * 15 11 7  3
         */
        [
            blocks[12], blocks[8], blocks[4], blocks[0], blocks[13], blocks[9], blocks[5],
            blocks[1], blocks[14], blocks[10], blocks[6], blocks[2], blocks[15], blocks[11],
            blocks[7], blocks[3],
        ]
    }

    [
        blocks,
        rotate(blocks),
        rotate(rotate(blocks)),
        rotate(rotate(rotate(blocks))),
    ]
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    kind: PieceType,
    rotation: u8,
}

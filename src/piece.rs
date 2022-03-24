use crate::display::Colour;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PieceType {
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

const fn rotate(shape: [Colour; 9], rotations: usize) -> [Colour; 9] {
    /*
     * 0 1 2
     * 3 4 5
     * 6 7 8
     *
     * Becomes
     *
     * 6 3 0
     * 7 4 1
     * 8 5 2
     */

    if rotations > 0 {
        let shape = [
            shape[6], shape[3], shape[0], shape[7], shape[4], shape[1], shape[8], shape[5],
            shape[2],
        ];

        rotate(shape, rotations - 1)
    } else {
        shape
    }
}

const I: [[Colour; 16]; 4] = {
    let mut blocks = [Colour::None; 16];

    blocks[4] = Colour::Cyan;
    blocks[5] = Colour::Cyan;
    blocks[6] = Colour::Cyan;
    blocks[7] = Colour::Cyan;

    const fn rotate_4x4(blocks: [Colour; 16], recursions: usize) -> [Colour; 16] {
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

        if recursions > 0 {
            let blocks = [
                blocks[12], blocks[8], blocks[4], blocks[0], blocks[13], blocks[9], blocks[5],
                blocks[1], blocks[14], blocks[10], blocks[6], blocks[2], blocks[15], blocks[11],
                blocks[7], blocks[3],
            ];

            rotate_4x4(blocks, recursions - 1)
        } else {
            blocks
        }
    }

    [
        blocks,
        rotate_4x4(blocks, 1),
        rotate_4x4(blocks, 2),
        rotate_4x4(blocks, 3),
    ]
};

const fn rotations(shape: [Colour; 9]) -> [[Colour; 16]; 4] {
    const fn pad(shape: [Colour; 9]) -> [Colour; 16] {
        [
            Colour::None,
            Colour::None,
            Colour::None,
            Colour::None,
            shape[0],
            shape[1],
            shape[2],
            Colour::None,
            shape[3],
            shape[4],
            shape[5],
            Colour::None,
            shape[6],
            shape[7],
            shape[8],
            Colour::None,
        ]
    }

    [
        pad(rotate(shape, 0)),
        pad(rotate(shape, 1)),
        pad(rotate(shape, 2)),
        pad(rotate(shape, 3)),
    ]
}

const J: [[Colour; 16]; 4] = rotations({
    let mut blocks = [Colour::None; 9];

    blocks[0] = Colour::Blue;
    blocks[3] = Colour::Blue;
    blocks[4] = Colour::Blue;
    blocks[5] = Colour::Blue;

    blocks
});

const L: [[Colour; 16]; 4] = rotations({
    let mut blocks = [Colour::None; 9];

    blocks[2] = Colour::Orange;
    blocks[3] = Colour::Orange;
    blocks[4] = Colour::Orange;
    blocks[5] = Colour::Orange;

    blocks
});

const O: [Colour; 16] = {
    let mut blocks = [Colour::None; 16];

    blocks[5] = Colour::Orange;
    blocks[6] = Colour::Orange;
    blocks[9] = Colour::Orange;
    blocks[10] = Colour::Orange;

    blocks
};

const S: [[Colour; 16]; 4] = rotations({
    let mut blocks = [Colour::None; 9];

    blocks[1] = Colour::Orange;
    blocks[2] = Colour::Orange;
    blocks[3] = Colour::Orange;
    blocks[4] = Colour::Orange;

    blocks
});

const T: [[Colour; 16]; 4] = rotations({
    let mut blocks = [Colour::None; 9];

    blocks[1] = Colour::Orange;
    blocks[3] = Colour::Orange;
    blocks[4] = Colour::Orange;
    blocks[5] = Colour::Orange;

    blocks
});

const Z: [[Colour; 16]; 4] = rotations({
    let mut blocks = [Colour::None; 9];

    blocks[0] = Colour::Orange;
    blocks[1] = Colour::Orange;
    blocks[4] = Colour::Orange;
    blocks[5] = Colour::Orange;

    blocks
});

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    kind: PieceType,
    rotation: u8,
}

impl Piece {
    pub const fn new(kind: PieceType) -> Self {
        Self { kind, rotation: 0 }
    }

    pub const fn blocks(&self) -> [Colour; 16] {
        match self.kind {
            PieceType::I => I[(self.rotation % 4) as usize],
            PieceType::J => J[(self.rotation % 4) as usize],
            PieceType::L => L[(self.rotation % 4) as usize],
            PieceType::O => O,
            PieceType::S => S[(self.rotation % 4) as usize],
            PieceType::T => T[(self.rotation % 4) as usize],
            PieceType::Z => Z[(self.rotation % 4) as usize],
        }
    }

    pub const fn kind(&self) -> PieceType {
        self.kind
    }

    pub const fn colour(&self) -> Colour {
        self.kind.colour()
    }

    pub const fn rotation(&self) -> u8 {
        self.rotation
    }

    pub const fn rotate(&self, turns: u8) -> Self {
        Self {
            kind: self.kind(),
            rotation: self.rotation.wrapping_add(turns) % 4,
        }
    }
}

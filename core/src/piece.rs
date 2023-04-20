use crate::colour::Colour;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub struct PieceBody([[Colour; 4]; 4]);

impl std::ops::Index<usize> for PieceBody {
    type Output = [Colour; 4];
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[wasm_bindgen]
impl PieceBody {
    pub fn block(&self, x: usize, y: usize) -> Colour {
        self.0[y][x]
    }
}

const fn rotate(shape: PieceBody, rotations: usize) -> PieceBody {
    /*
     * - - - -
     * 0 1 2 -
     * 3 4 5 -
     * 6 7 8 -
     *
     * Becomes
     *
     * - - - -
     * 6 3 0 -
     * 7 4 1 -
     * 8 5 2 -
     */

    if rotations > 0 {
        let shape = shape.0;
        let shape = [
            [shape[0][0], shape[0][1], shape[0][2], shape[0][3]],
            [shape[3][0], shape[2][0], shape[1][0], shape[1][3]],
            [shape[3][1], shape[2][1], shape[1][1], shape[2][3]],
            [shape[3][2], shape[2][2], shape[1][2], shape[3][3]],
        ];

        rotate(PieceBody(shape), rotations - 1)
    } else {
        shape
    }
}

use crate::colour::Colour::{Blue, Cyan, Green, None, Orange, Purple, Red, Yellow};

const I: [PieceBody; 4] = {
    let mut blocks = [None; 16];

    blocks[4] = Cyan;
    blocks[5] = Cyan;
    blocks[6] = Cyan;
    blocks[7] = Cyan;

    const fn rotate_4x4(blocks: [Colour; 16], rotations: usize) -> PieceBody {
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

        if rotations > 0 {
            let blocks = [
                blocks[12], blocks[8], blocks[4], blocks[0], blocks[13], blocks[9], blocks[5],
                blocks[1], blocks[14], blocks[10], blocks[6], blocks[2], blocks[15], blocks[11],
                blocks[7], blocks[3],
            ];

            rotate_4x4(blocks, rotations - 1)
        } else {
            PieceBody([
                [blocks[0], blocks[1], blocks[2], blocks[3]],
                [blocks[4], blocks[5], blocks[6], blocks[7]],
                [blocks[8], blocks[9], blocks[10], blocks[11]],
                [blocks[12], blocks[13], blocks[14], blocks[15]],
            ])
        }
    }

    [
        rotate_4x4(blocks, 0),
        rotate_4x4(blocks, 1),
        rotate_4x4(blocks, 2),
        rotate_4x4(blocks, 3),
    ]
};

const fn rotations(shape: [[Colour; 4]; 4]) -> [PieceBody; 4] {
    let shape = PieceBody(shape);
    [
        rotate(shape, 0),
        rotate(shape, 1),
        rotate(shape, 2),
        rotate(shape, 3),
    ]
}

const J: [PieceBody; 4] = rotations([
    [None, None, None, None],
    [Blue, None, None, None],
    [Blue, Blue, Blue, None],
    [None, None, None, None],
]);

const L: [PieceBody; 4] = rotations([
    [None, None, None, None],
    [None, None, Orange, None],
    [Orange, Orange, Orange, None],
    [None, None, None, None],
]);

const O: [PieceBody; 4] = [PieceBody([
    [None, None, None, None],
    [None, Yellow, Yellow, None],
    [None, Yellow, Yellow, None],
    [None, None, None, None],
]); 4];

const S: [PieceBody; 4] = rotations([
    [None, None, None, None],
    [None, Green, Green, None],
    [Green, Green, None, None],
    [None, None, None, None],
]);

const T: [PieceBody; 4] = rotations([
    [None, None, None, None],
    [None, Purple, None, None],
    [Purple, Purple, Purple, None],
    [None, None, None, None],
]);

const Z: [PieceBody; 4] = rotations([
    [None, None, None, None],
    [Red, Red, None, None],
    [None, Red, Red, None],
    [None, None, None, None],
]);

#[wasm_bindgen]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Piece {
    kind: PieceType,
    rotation: u8,
}

macro_rules! constant {
    ($($name: ident),*) => {
        impl Piece {
            $(pub const $name: Self = Self {
                kind: PieceType::$name,
                rotation: 0,
            };)*
        }
    };
}

constant!(I, J, L, O, S, T, Z);

#[wasm_bindgen]
impl Piece {
    pub fn new(kind: PieceType) -> Self {
        Self { kind, rotation: 0 }
    }

    pub fn blocks(&self) -> PieceBody {
        (match self.kind {
            PieceType::I => I,
            PieceType::J => J,
            PieceType::L => L,
            PieceType::O => O,
            PieceType::S => S,
            PieceType::T => T,
            PieceType::Z => Z,
        })[(self.rotation % 4) as usize]
    }

    pub fn kind(&self) -> PieceType {
        self.kind
    }

    pub fn rotation(&self) -> u8 {
        self.rotation
    }

    pub fn rotate(&self, turns: u8) -> Self {
        Self {
            kind: self.kind(),
            rotation: self.rotation.wrapping_add(turns) % 4,
        }
    }
}

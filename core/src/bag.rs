use crate::piece::PieceType;
use rand::prelude::SliceRandom;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bag {
    i: usize,
    rng: SmallRng,
    seed: u64,
    pieces: [PieceType; 14],
}

const KINDS: [PieceType; 7] = [
    PieceType::I,
    PieceType::J,
    PieceType::L,
    PieceType::O,
    PieceType::S,
    PieceType::T,
    PieceType::Z,
];

#[wasm_bindgen]
impl Bag {
    pub fn new(seed: u64) -> Self {
        let mut rng = SmallRng::seed_from_u64(seed);

        let mut bag1 = KINDS.clone();
        bag1.shuffle(&mut rng);
        let mut bag2 = KINDS.clone();
        bag2.shuffle(&mut rng);

        Self {
            i: 0,
            rng,
            seed,
            pieces: [
                bag1[0], bag1[1], bag1[2], bag1[3], bag1[4], bag1[5], bag1[6], bag2[0], bag2[1],
                bag2[2], bag2[3], bag2[4], bag2[5], bag2[6],
            ],
        }
    }

    pub fn next(&mut self) -> PieceType {
        let kind = self.pieces[self.i % 7];

        self.i += 1;

        if self.i == 7 {
            self.i = 0;

            for i in 0..7 {
                self.pieces[i] = self.pieces[i + 7];
            }

            let mut next_bag = KINDS.clone();
            next_bag.shuffle(&mut self.rng);

            for i in 0..7 {
                self.pieces[i + 7] = next_bag[i];
            }
        }

        kind
    }

    pub fn peek(&self, i: usize) -> PieceType {
        self.pieces[(self.i + i) % 7]
    }
}

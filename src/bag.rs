use crate::piece::PieceType;
use rand::prelude::SliceRandom;
use rand::rngs::SmallRng;
use rand::SeedableRng;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Bag {
    i: usize,
    rng: SmallRng,
    seed: [u8; 32],
    pieces: [PieceType; 14],
}

impl Bag {
    const KINDS: [PieceType; 7] = [
        PieceType::I,
        PieceType::J,
        PieceType::L,
        PieceType::O,
        PieceType::S,
        PieceType::T,
        PieceType::Z,
    ];

    pub fn new(seed: [u8; 32]) -> Self {
        let mut rng = SmallRng::from_seed(seed);

        let mut bag1 = Self::KINDS.clone();
        bag1.shuffle(&mut rng);
        let mut bag2 = Self::KINDS.clone();
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

            let mut next_bag = Self::KINDS.clone();
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

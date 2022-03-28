use std::collections::VecDeque;

use crate::input::{Input, InputDirection, InputRotation};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Recorder {
    seed: [u8; 32],
    frames: Vec<RecorderFrame>,
    last_frame: u128,
}

impl Recorder {
    pub fn new(seed: [u8; 32], now: u128) -> Self {
        Self {
            seed,
            frames: vec![RecorderFrame::new(0, Input::default())],
            last_frame: now,
        }
    }

    pub fn record(&mut self, input: Input, now: u128) {
        const MAX_TIME: u128 = std::u16::MAX as u128;

        let elapsed = now - self.last_frame;

        if elapsed == MAX_TIME {
            self.frames.push(RecorderFrame::new(std::u16::MAX, input));
            self.last_frame = now;
        } else if input != Input::default() {
            self.frames.push(RecorderFrame::new(elapsed as u16, input));
            self.last_frame = now;
        }
    }

    pub fn raw(self) -> Vec<u8> {
        let mut buffer = Vec::with_capacity(32 + 3 * self.frames.len());

        buffer.append(&mut Vec::from(self.seed));

        for frame in self.frames {
            buffer.push((frame.time >> 8) as u8);
            buffer.push(frame.time as u8);
            buffer.push(frame.input);
        }

        buffer
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct RecorderFrame {
    pub time: u16,
    pub input: u8,
}

impl RecorderFrame {
    const QUARTER: u8 = 1 << 3;
    const TWO_QUARTER: u8 = 2 << 3;
    const THREE_QUARTER: u8 = 3 << 3;

    const LEFT: u8 = 1;
    const RIGHT: u8 = 2;

    const fn new(delta_time: u16, input: Input) -> Self {
        let input = (input.hold as u8) << 7
            | (input.hard_drop as u8) << 6
            | (input.soft_drop as u8) << 5
            | match input.rotation {
                InputRotation::None => 0,
                InputRotation::Quarter => Self::QUARTER,
                InputRotation::TwoQuarter => Self::TWO_QUARTER,
                InputRotation::ThreeQuarter => Self::THREE_QUARTER,
            }
            | match input.direction {
                InputDirection::None => 0,
                InputDirection::Left => Self::LEFT,
                InputDirection::Right => Self::RIGHT,
            };

        Self {
            time: delta_time,
            input,
        }
    }

    const fn input(&self) -> Input {
        Input {
            hold: self.input & 1 << 7 != 0,
            hard_drop: self.input & 1 << 6 != 0,
            soft_drop: self.input & 1 << 5 != 0,
            rotation: match self.input & (0b11 << 3) {
                Self::QUARTER => InputRotation::Quarter,
                Self::TWO_QUARTER => InputRotation::TwoQuarter,
                Self::THREE_QUARTER => InputRotation::ThreeQuarter,
                _ => InputRotation::None,
            },
            direction: match self.input & 0b11 {
                Self::LEFT => InputDirection::Left,
                Self::RIGHT => InputDirection::Right,
                _ => InputDirection::None,
            },
        }
    }
}

impl From<[u8; 3]> for RecorderFrame {
    fn from(raw: [u8; 3]) -> Self {
        Self {
            time: (raw[0] as u16) << 8 | raw[1] as u16,
            input: raw[2],
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replay {
    seed: [u8; 32],
    frames: VecDeque<Frame>,
}

impl Replay {
    pub fn new(buffer: Vec<u8>) -> Self {
        if buffer.len() < 35 {
            panic!("Unable to read replay file.");
        }

        let mut seed = [0u8; 32];

        for i in 0..32 {
            seed[i] = buffer[i];
        }

        let mut time = 0u128;

        let mut frames = VecDeque::with_capacity((buffer.len() - 32) / 3);

        for i in (32..buffer.len()).step_by(3) {
            let frame = RecorderFrame::from([buffer[i], buffer[i + 1], buffer[i + 2]]);
            time += frame.time as u128;

            frames.push_back(Frame::new(time, frame));
        }

        Self { seed, frames }
    }

    pub fn seed(&self) -> [u8; 32] {
        self.seed
    }

    pub fn next(&mut self) -> Option<Frame> {
        self.frames.pop_front()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Frame {
    pub time: u128,
    pub input: Input,
}

impl Frame {
    fn new(time: u128, frame: RecorderFrame) -> Self {
        Self {
            time,
            input: frame.input(),
        }
    }
}

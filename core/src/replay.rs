use std::{
    collections::VecDeque,
    io::{BufRead, BufReader, Read},
};

use crate::{
    input::{Input, InputDirection, InputRotation},
    Score,
};

const VERSION: u8 = 1;

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

    pub fn raw(self, username: &str, score: Score, duration: u64, end_time: i64) -> Vec<u8> {
        let mut buffer = Vec::from(username.as_bytes());
        buffer.push('\n' as u8);
        buffer.push(VERSION);

        let mut append = |num: u64| {
            for b in num.to_be_bytes() {
                buffer.push(b);
            }
        };

        append(score.score());
        append(score.lines());
        append(duration);

        for b in end_time.to_be_bytes() {
            buffer.push(b);
        }

        buffer.reserve(32 + 3 * self.frames.len());

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
        let input = if !input.quit {
            (input.hold as u8) << 7
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
                }
        } else {
            std::u8::MAX
        };

        Self {
            time: delta_time,
            input,
        }
    }

    const fn input(&self) -> Input {
        Input {
            quit: self.input == std::u8::MAX,
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

#[derive(Clone, Debug)]
pub enum ReplayError {
    UsernameNotFound,
    BufferTooShort,
}

impl std::fmt::Display for ReplayError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ReplayError::UsernameNotFound => "No username in replay",
                ReplayError::BufferTooShort => "Buffer too short, replay file likely corrupted",
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Replay {
    username: String,
    version: u8,
    score: Score,
    duration: u64,
    time_stamp: i64,
    seed: [u8; 32],
    frames: VecDeque<Frame>,
}

impl Replay {
    pub fn new(buffer: Vec<u8>) -> Result<Self, ReplayError> {
        let mut buf = BufReader::new(buffer.as_slice());

        let mut username = String::default();
        match buf.read_line(&mut username) {
            Ok(_) => {}
            Err(_) => return Err(ReplayError::UsernameNotFound),
        };
        let username = username.trim().to_string();

        let mut version = [0u8];
        match buf.read_exact(&mut version) {
            Ok(_) => {}
            Err(_) => return Err(ReplayError::BufferTooShort),
        };
        let version = version[0];

        let mut num = [0u8; 8];

        match buf.read_exact(&mut num) {
            Ok(_) => {}
            Err(_) => return Err(ReplayError::BufferTooShort),
        };
        let score = u64::from_be_bytes(num);

        match buf.read_exact(&mut num) {
            Ok(_) => {}
            Err(_) => return Err(ReplayError::BufferTooShort),
        };
        let lines = u64::from_be_bytes(num);

        let score = Score { score, lines };

        match buf.read_exact(&mut num) {
            Ok(_) => {}
            Err(_) => return Err(ReplayError::BufferTooShort),
        };
        let duration = u64::from_be_bytes(num);

        match buf.read_exact(&mut num) {
            Ok(_) => {}
            Err(_) => return Err(ReplayError::BufferTooShort),
        };
        let time_stamp = i64::from_be_bytes(num);

        let mut seed = [0u8; 32];
        match buf.read_exact(&mut seed) {
            Ok(_) => {}
            Err(_) => return Err(ReplayError::BufferTooShort),
        };

        let mut time = 0u128;

        let mut frames = VecDeque::with_capacity((buffer.len() - 32) / 3);

        let mut frame_data = [0u8; 3];
        while let Ok(_) = buf.read_exact(&mut frame_data) {
            let frame = RecorderFrame::from(frame_data);
            time += frame.time as u128;

            frames.push_back(Frame::new(time, frame));
        }

        Ok(Self {
            seed,
            score,
            frames,
            version,
            username,
            duration,
            time_stamp,
        })
    }

    pub fn seed(&self) -> [u8; 32] {
        self.seed
    }

    pub fn next(&mut self) -> Option<Frame> {
        self.frames.pop_front()
    }

    pub fn score(&self) -> Score {
        self.score
    }

    pub fn username(&self) -> &String {
        &self.username
    }

    pub fn duration(&self) -> u64 {
        self.duration
    }

    pub fn time_stamp(&self) -> i64 {
        self.time_stamp
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

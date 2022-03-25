use crate::config::Config;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputDirection {
    None,
    Left,
    Right,
}

impl Default for InputDirection {
    fn default() -> Self {
        InputDirection::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InputRotation {
    None,
    Quarter,
    TwoQuarter,
    ThreeQuarter,
}

impl Default for InputRotation {
    fn default() -> Self {
        InputRotation::None
    }
}

#[derive(Clone, Copy, Default, Debug, PartialEq, Eq)]
pub struct Input {
    pub hold: bool,
    pub hard_drop: bool,
    pub soft_drop: bool,
    pub rotation: InputRotation,
    pub direction: InputDirection,
}

impl Input {
    pub fn update(&mut self, c: char, conf: Config) {
        if c == conf.left {
            self.direction = InputDirection::Left;
        } else if c == conf.right {
            self.direction = InputDirection::Right;
        } else if c == conf.hold {
            self.hold = true;
        } else if c == conf.rotate_90 {
            self.rotation = InputRotation::Quarter;
        } else if c == conf.rotate_180 {
            self.rotation = InputRotation::TwoQuarter;
        } else if c == conf.rotate_270 {
            self.rotation = InputRotation::ThreeQuarter;
        } else if c == conf.left {
            self.direction = InputDirection::Left;
        } else if c == conf.soft_drop {
            self.soft_drop = true;
        } else if c == conf.hard_drop {
            self.hard_drop = true;
        }
    }
}

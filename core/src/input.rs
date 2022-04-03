use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
pub enum InputDirection {
    None,
    Left,
    Right,
    SnapLeft,
    SnapRight,
}

impl Default for InputDirection {
    fn default() -> Self {
        InputDirection::None
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen]
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
#[wasm_bindgen]
pub struct Input {
    pub hold: bool,
    pub quit: bool,
    pub hard_drop: bool,
    pub soft_drop: bool,
    pub rotation: InputRotation,
    pub direction: InputDirection,
}

#[wasm_bindgen]
impl Input {
    pub fn new(
        hold: bool,
        quit: bool,
        hard_drop: bool,
        soft_drop: bool,
        rotation: InputRotation,
        direction: InputDirection,
    ) -> Self {
        Self {
            hold,
            quit,
            hard_drop,
            soft_drop,
            rotation,
            direction,
        }
    }
}

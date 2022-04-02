// Colours as their associated XTerm colour
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[wasm_bindgen::prelude::wasm_bindgen]
#[repr(u8)]
pub enum Colour {
    None = 0,

    Cyan = 45,
    Yellow = 226,
    Purple = 93,
    Green = 34,
    Red = 124,
    Blue = 21,
    Orange = 202,

    Grey = 250,
    White = 255,
}

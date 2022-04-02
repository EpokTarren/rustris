use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[wasm_bindgen]
pub struct Point {
    x: i8,
    y: i8,
}

impl Point {
    pub const fn constant(x: i8, y: i8) -> Self {
        Self { x, y }
    }
}

#[wasm_bindgen]
impl Point {
    #[wasm_bindgen]
    pub fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }

    #[wasm_bindgen]
    pub fn x(&self) -> i8 {
        self.x
    }

    #[wasm_bindgen]
    pub fn y(&self) -> i8 {
        self.y
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x() + rhs.x(),
            y: self.y() + rhs.y(),
        }
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Point;

    fn sub(self, rhs: Point) -> Self::Output {
        Self::Output {
            x: self.x() - rhs.x(),
            y: self.y() - rhs.y(),
        }
    }
}

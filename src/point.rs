#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Point {
    x: i8,
    y: i8,
}

impl Point {
    pub const fn new(x: i8, y: i8) -> Self {
        Self { x, y }
    }
    pub const fn x(&self) -> i8 {
        self.x
    }

    pub const fn y(&self) -> i8 {
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

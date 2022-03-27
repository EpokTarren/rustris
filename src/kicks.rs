use crate::point::Point;

pub const KICKS: [[[Point; 5]; 2]; 4] = [
    [
        // 0
        [
            // R
            Point::new(0, 0),
            Point::new(-1, 0),
            Point::new(-1, -1),
            Point::new(0, 2),
            Point::new(-1, 2),
        ],
        [
            // L
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(1, -1),
            Point::new(0, 2),
            Point::new(1, 2),
        ],
    ],
    [
        // R
        [
            // 2
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(0, -2),
            Point::new(1, -2),
        ],
        [
            // 0
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(1, 1),
            Point::new(0, -2),
            Point::new(1, -2),
        ],
    ],
    [
        // 2
        [
            // L
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(1, -1),
            Point::new(0, 2),
            Point::new(1, 2),
        ],
        [
            // R
            Point::new(0, 0),
            Point::new(-1, 0),
            Point::new(-1, -1),
            Point::new(0, 2),
            Point::new(-1, 2),
        ],
    ],
    [
        // L
        [
            // 0
            Point::new(0, 0),
            Point::new(-1, 0),
            Point::new(-1, 1),
            Point::new(0, -2),
            Point::new(-1, -2),
        ],
        [
            // 2
            Point::new(0, 0),
            Point::new(-1, 0),
            Point::new(-1, 1),
            Point::new(0, -2),
            Point::new(-1, -2),
        ],
    ],
];

pub const I_KICKS: [[[Point; 5]; 2]; 4] = [
    [
        // 0
        [
            // R
            Point::new(0, 0),
            Point::new(-2, 0),
            Point::new(1, 0),
            Point::new(-2, 1),
            Point::new(1, -2),
        ],
        [
            // L
            Point::new(0, 0),
            Point::new(-1, 0),
            Point::new(2, 0),
            Point::new(-1, -2),
            Point::new(2, 1),
        ],
    ],
    [
        // R
        [
            // 2
            Point::new(0, 0),
            Point::new(-1, 0),
            Point::new(2, 0),
            Point::new(-1, -2),
            Point::new(2, 1),
        ],
        [
            // 0
            Point::new(0, 0),
            Point::new(2, 0),
            Point::new(-1, 0),
            Point::new(2, -1),
            Point::new(-1, 2),
        ],
    ],
    [
        // 2
        [
            // L
            Point::new(0, 0),
            Point::new(2, 0),
            Point::new(-1, 0),
            Point::new(2, -1),
            Point::new(-1, 2),
        ],
        [
            // R
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(-2, 0),
            Point::new(1, 2),
            Point::new(-2, -1),
        ],
    ],
    [
        // L
        [
            // 0
            Point::new(0, 0),
            Point::new(1, 0),
            Point::new(-2, 0),
            Point::new(1, 2),
            Point::new(-2, -1),
        ],
        [
            // 2
            Point::new(0, 0),
            Point::new(-2, 0),
            Point::new(1, 0),
            Point::new(-2, 1),
            Point::new(1, -2),
        ],
    ],
];

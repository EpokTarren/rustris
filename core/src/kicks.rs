use crate::point::Point;

pub const KICKS: [[[Point; 5]; 2]; 4] = [
    [
        // 0
        [
            // R
            Point::constant(0, 0),
            Point::constant(-1, 0),
            Point::constant(-1, -1),
            Point::constant(0, 2),
            Point::constant(-1, 2),
        ],
        [
            // L
            Point::constant(0, 0),
            Point::constant(1, 0),
            Point::constant(1, -1),
            Point::constant(0, 2),
            Point::constant(1, 2),
        ],
    ],
    [
        // R
        [
            // 2
            Point::constant(0, 0),
            Point::constant(1, 0),
            Point::constant(1, 1),
            Point::constant(0, -2),
            Point::constant(1, -2),
        ],
        [
            // 0
            Point::constant(0, 0),
            Point::constant(1, 0),
            Point::constant(1, 1),
            Point::constant(0, -2),
            Point::constant(1, -2),
        ],
    ],
    [
        // 2
        [
            // L
            Point::constant(0, 0),
            Point::constant(1, 0),
            Point::constant(1, -1),
            Point::constant(0, 2),
            Point::constant(1, 2),
        ],
        [
            // R
            Point::constant(0, 0),
            Point::constant(-1, 0),
            Point::constant(-1, -1),
            Point::constant(0, 2),
            Point::constant(-1, 2),
        ],
    ],
    [
        // L
        [
            // 0
            Point::constant(0, 0),
            Point::constant(-1, 0),
            Point::constant(-1, 1),
            Point::constant(0, -2),
            Point::constant(-1, -2),
        ],
        [
            // 2
            Point::constant(0, 0),
            Point::constant(-1, 0),
            Point::constant(-1, 1),
            Point::constant(0, -2),
            Point::constant(-1, -2),
        ],
    ],
];

pub const I_KICKS: [[[Point; 5]; 2]; 4] = [
    [
        // 0
        [
            // R
            Point::constant(0, 0),
            Point::constant(-2, 0),
            Point::constant(1, 0),
            Point::constant(-2, 1),
            Point::constant(1, -2),
        ],
        [
            // L
            Point::constant(0, 0),
            Point::constant(-1, 0),
            Point::constant(2, 0),
            Point::constant(-1, -2),
            Point::constant(2, 1),
        ],
    ],
    [
        // R
        [
            // 2
            Point::constant(0, 0),
            Point::constant(-1, 0),
            Point::constant(2, 0),
            Point::constant(-1, -2),
            Point::constant(2, 1),
        ],
        [
            // 0
            Point::constant(0, 0),
            Point::constant(2, 0),
            Point::constant(-1, 0),
            Point::constant(2, -1),
            Point::constant(-1, 2),
        ],
    ],
    [
        // 2
        [
            // L
            Point::constant(0, 0),
            Point::constant(2, 0),
            Point::constant(-1, 0),
            Point::constant(2, -1),
            Point::constant(-1, 2),
        ],
        [
            // R
            Point::constant(0, 0),
            Point::constant(1, 0),
            Point::constant(-2, 0),
            Point::constant(1, 2),
            Point::constant(-2, -1),
        ],
    ],
    [
        // L
        [
            // 0
            Point::constant(0, 0),
            Point::constant(1, 0),
            Point::constant(-2, 0),
            Point::constant(1, 2),
            Point::constant(-2, -1),
        ],
        [
            // 2
            Point::constant(0, 0),
            Point::constant(-2, 0),
            Point::constant(1, 0),
            Point::constant(-2, 1),
            Point::constant(1, -2),
        ],
    ],
];

pub const KICKS_180: [[Point; 6]; 4] = [
    [
        Point::constant(0, 0),
        Point::constant(0, -1),
        Point::constant(1, -1),
        Point::constant(1, -1),
        Point::constant(1, 0),
        Point::constant(1, 0),
    ],
    [
        Point::constant(0, 0),
        Point::constant(1, 0),
        Point::constant(1, -2),
        Point::constant(1, -1),
        Point::constant(0, -2),
        Point::constant(0, -1),
    ],
    [
        Point::constant(0, 0),
        Point::constant(0, 1),
        Point::constant(1, 1),
        Point::constant(1, 1),
        Point::constant(1, 0),
        Point::constant(1, 0),
    ],
    [
        Point::constant(0, 0),
        Point::constant(1, 0),
        Point::constant(1, -2),
        Point::constant(1, -1),
        Point::constant(0, -2),
        Point::constant(0, -1),
    ],
];

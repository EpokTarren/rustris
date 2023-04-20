#[allow(unused_imports)]
use crate::{
    piece::PieceType,
    point::Point,
    tests::util::{spin_test, tester},
    Bag, Board, Piece,
};

#[test]
fn no_kick() {
    let board0 =
        Board::from_strs_with_piece(&["#### #####"], Bag::new(0), Piece::new(PieceType::T));

    let board = board0.clone();
    let expected = Board::from_strs(&["   ###    "], Bag::new(0));
    spin_test(board, expected, "ShH");

    let board = board0.clone();
    let expected = Board::from_strs(
        &[
            //
            "    #     ",
            "   ###    ",
            "#### #####",
        ],
        Bag::new(0),
    );
    spin_test(board, expected, "hShH");

    let board1 = Board::from_strs_with_piece(&[], Bag::new(0), Piece::new(PieceType::T));

    let board = board1.clone();
    let expected = Board::from_strs(
        &[
            //
            "    #     ",
            "   ##     ",
            "    #     ",
        ],
        Bag::new(0),
    );
    spin_test(board, expected.clone(), "cShH");
    spin_test(board1.mirror(), expected.mirror(), "rCShH");
}

tester!(
    off_the_floor, Piece::T, "ShH",
    ;
    "   ###    ",
    "    #     ",
);

tester!(
    unhook_right_north_south, Piece::T, "SLhH",
    "##        ",
    "#         ",
    "######### ",
    ;
    "#####     ",
    "#  #      ",
    "######### ",
);

tester!(
    unhook_left_north_south, Piece::T, "SRhH",
    "        ##",
    "         #",
    " #########",
    ;
    "     #####",
    "      #  #",
    " #########",
);

tester!(
    right_zipper_north_south, Piece::T, "hH", (0, Board::HEIGHT as i8 - 4),
    "# ###     ",
    "    #     ",
    "## ##     ",
    ;
    "# ###     ",
    " ####     ",
    "#####     ",
);

tester!(
    left_zipper_north_south, Piece::T, "hH", (0, Board::HEIGHT as i8 - 4),
    "# ###     ",
    "    #     ",
    "## ##     ",
    ;
    "# ###     ",
    " ####     ",
    "#####     ",
);

tester!(
    off_the_ceiling, Piece::T, "hRSLhH",
    "#######   ",
    "          ",
    "          "
    ;
    "#######   ",
    " #        ",
    "###       "
);

tester!(
    unhook_left_south_north, Piece::T, "hLSRhH",
    "   #######",
    "         #",
    "        ##",
    ;
    "   #######",
    "      #  #",
    "     #####",
);

tester!(
    unhook_right_south_north, Piece::T, "hRSLhH",
    "#######   ",
    "#         ",
    "##        ",
    ;
    "#######   ",
    "#  #      ",
    "#####     ",
);

tester!(
    left_zipper_south_north, Piece::T.rotate(2), "hH", (1, Board::HEIGHT as i8 - 4),
    "# ###     ",
    "    #     ",
    "## ##     ",
    ;
    "#####     ",
    "### #     ",
    "## ##     ",
);

tester!(
    right_zipper_south_north, Piece::T.rotate(2), "hH", (1, Board::HEIGHT as i8 - 4),
    "### #     ",
    "#         ",
    "## ##     ",
    ;
    "#####     ",
    "# ###     ",
    "## ##     ",
);

tester!(
    off_the_wall_left, Piece::T, "cLrShH",
    "#         ",
    "#         ",
    "#         ",
    "####      ",
    ;
    "# T       ",
    "#TT       ",
    "# T       ",
    "####      ",
);

tester!(
    off_the_wall_right, Piece::T, "CLSRhH",
    "   #      ",
    "   #      ",
    "   #      ",
    "####      ",
    ;
    " T #      ",
    " TT#      ",
    " T #      ",
    "####      ",
);

tester!(
    tall_jump_left_wall, Piece::T, "cLrShrH",
    "#         ",
    "#         ",
    "#         ",
    "#  #      ",
    "# ##      ",
    "####      ",
    ;
    "#  T      ",
    "# TT      ",
    "#  T      ",
    "#  #      ",
    "# ##      ",
    "####      ",
);

tester!(
    tall_jump_right_wall, Piece::T, "CLrShlH",
    "   #      ",
    "   #      ",
    "   #      ",
    "#  #      ",
    "## #      ",
    "####      ",
    ;
    "T  #      ",
    "TT #      ",
    "T  #      ",
    "#  #      ",
    "## #      ",
    "####      ",
);

tester!(
    constricted_jump_left_wall, Piece::T, "rSLclhH",
    "####      ",
    "#         ",
    "#         ",
    "#  #######",
    "# ##     #",
    "####     #",
    ;
    "####      ",
    "# T       ",
    "#TT       ",
    "# T#######",
    "# ##     #",
    "####     #",
);

tester!(
    constricted_jump_right_wall, Piece::T, "SRCrhH",
    "      ####",
    "         #",
    "         #",
    "#######  #",
    "#     ## #",
    "#     ####",
    ;
    "      ####",
    "       T #",
    "       TT#",
    "#######T #",
    "#     ## #",
    "#     ####",
);

tester!(
    tall_zipper_east_west, Piece::T.rotate(1), "hH", (0, Board::HEIGHT as i8 - 5),
    "  #       ",
    "  #       ",
    "# #       ",
    "#         ",
    "# #       ",
    "###       ",
    ;
    " T#       ",
    "TT#       ",
    "#T#       ",
    "#         ",
    "# #       ",
    "###       ",
);

tester!(
    tall_zipper_west_east, Piece::T.rotate(3), "hH", (0, Board::HEIGHT as i8 - 5),
    "#         ",
    "#         ",
    "# #       ",
    "  #       ",
    "# #       ",
    "###       ",
    ;
    "#T        ",
    "#TT       ",
    "#T#       ",
    "  #       ",
    "# #       ",
    "###       ",
);

tester!(
    constricted_zipper_east_west, Piece::T.rotate(1), "hH", (0, Board::HEIGHT as i8 - 5),
    "###       ",
    "  #       ",
    "  #       ",
    "#         ",
    "# #       ",
    "###       ",
    ;
    "###       ",
    " T#       ",
    "TT#       ",
    "#T        ",
    "# #       ",
    "###       ",
);

tester!(
    constricted_zipper_west_east, Piece::T.rotate(3), "hH", (0, Board::HEIGHT as i8 - 5),
    "###       ",
    "#         ",
    "#         ",
    "  #       ",
    "# #       ",
    "###       ",
    ;
    "###       ",
    "#T        ",
    "#TT       ",
    " T#       ",
    "# #       ",
    "###       ",
);

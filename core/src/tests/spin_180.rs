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
    off_the_floor, PieceType::T, "ShH",
    ;
    "   ###    ",
    "    #     ",
);

tester!(
    unhook_right_north_south, PieceType::T, "SLhH",
    "##        ",
    "#         ",
    "######### ",
    ;
    "#####     ",
    "#  #      ",
    "######### ",
);

tester!(
    unhook_left_north_south, PieceType::T, "SRhH",
    "        ##",
    "         #",
    " #########",
    ;
    "     #####",
    "      #  #",
    " #########",
);

tester!(
    right_zipper_north_south, PieceType::T, "hH", (0, Board::HEIGHT as i8 - 4),
    "# ###     ",
    "    #     ",
    "## ##     ",
    ;
    "# ###     ",
    " ####     ",
    "#####     ",

);

tester!(
    left_zipper_north_south, PieceType::T, "hH", (0, Board::HEIGHT as i8 - 4),
    "# ###     ",
    "    #     ",
    "## ##     ",
    ;
    "# ###     ",
    " ####     ",
    "#####     ",

);

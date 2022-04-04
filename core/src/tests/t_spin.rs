#[allow(unused_imports)]
use crate::{piece::PieceType, tests::util::spin_test, Bag, Board, Piece};

const TKI_LEFT: [&'static str; 4] = [
    //
    "       #  ",
    "#  ## ####",
    "#   ######",
    "## #######",
];

#[test]
fn tki_left_counter_clockwise() {
    let board = Board::from_strs_with_piece(&TKI_LEFT, Bag::new(0), Piece::new(PieceType::T));
    let expected = Board::from_strs(&TKI_LEFT[0..2], Bag::new(0));
    spin_test(board, expected, "llCSCH")
}

#[test]
fn tki_left_clockwise() {
    let board = Board::from_strs_with_piece(&TKI_LEFT, Bag::new(0), Piece::new(PieceType::T));
    let expected = Board::from_strs(&TKI_LEFT[0..2], Bag::new(0));
    spin_test(board, expected, "lllcScH")
}

const TKI_RIGHT: [&'static str; 4] = [
    //
    "  #       ",
    "#### ##  #",
    "######   #",
    "####### ##",
];

#[test]
fn tki_right_counter_clockwise() {
    let board = Board::from_strs_with_piece(&TKI_RIGHT, Bag::new(0), Piece::new(PieceType::T));
    let expected = Board::from_strs(&TKI_RIGHT[0..2], Bag::new(0));
    spin_test(board, expected, "rrrrCSCH")
}

#[test]
fn tki_right_clockwise() {
    let board = Board::from_strs_with_piece(&TKI_RIGHT, Bag::new(0), Piece::new(PieceType::T));
    let expected = Board::from_strs(&TKI_RIGHT[0..2], Bag::new(0));
    spin_test(board, expected, "rrrcScH")
}

const HACHI_LEFT: [&'static str; 7] = [
    "  ##    ##",
    "   ## ####",
    "## #######",
    "#  #######",
    "#   ######",
    "## #######",
    "## #######",
];

const HACHI_LEFT_TST: [&'static str; 5] = [
    HACHI_LEFT[0],
    HACHI_LEFT[1],
    HACHI_LEFT[2],
    HACHI_LEFT[3],
    HACHI_LEFT[6],
];

#[test]
fn hachi_left() {
    let board = Board::from_strs_with_piece(&HACHI_LEFT, Bag::new(0), Piece::new(PieceType::T));
    let tst = Board::from_strs_with_piece(&HACHI_LEFT_TST, Bag::new(0), Piece::new(PieceType::T));
    let expected = Board::from_strs(&HACHI_LEFT_TST[0..2], Bag::new(0));

    spin_test(board, tst.clone(), "cLSCSCSCH");

    spin_test(tst, expected, "cLSCSCH");
}

const HACHI_RIGHT: [&'static str; 7] = [
    "##    ##  ",
    "#### ##   ",
    "####### ##",
    "#######  #",
    "######   #",
    "####### ##",
    "####### ##",
];

const HACHI_RIGHT_TST: [&'static str; 5] = [
    HACHI_RIGHT[0],
    HACHI_RIGHT[1],
    HACHI_RIGHT[2],
    HACHI_RIGHT[3],
    HACHI_RIGHT[6],
];

#[test]
fn hachi_right() {
    let board = Board::from_strs_with_piece(&HACHI_RIGHT, Bag::new(0), Piece::new(PieceType::T));
    let tst = Board::from_strs_with_piece(&HACHI_RIGHT_TST, Bag::new(0), Piece::new(PieceType::T));
    let expected = Board::from_strs(&HACHI_RIGHT_TST[0..2], Bag::new(0));

    spin_test(board, tst.clone(), "CRScScScH");

    spin_test(tst, expected, "CRScScH");
}

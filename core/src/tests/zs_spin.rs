#[allow(unused_imports)]
use super::util::{reverse_board, spin_test};
#[allow(unused_imports)]
use crate::{piece::PieceType, Bag, Board, Piece};

const ZSD: [&'static str; 2] = [
    //
    "#####  ###",
    "######  ##",
];

#[test]
fn zsd() {
    let board = Board::from_strs_with_piece(&ZSD, Bag::new(0), Piece::new(PieceType::Z));
    let expected = Board::from_strs(&[], Bag::new(0));

    spin_test(board, expected, "rrrCSCH");
}

#[test]
fn ssd() {
    let board = reverse_board(Board::from_strs_with_piece(
        &ZSD,
        Bag::new(0),
        Piece::new(PieceType::S),
    ));
    let expected = Board::from_strs(&[], Bag::new(0));

    spin_test(board, expected, "llcScH");
}

const B2B_ZST: [&[&'static str]; 5] = [
    &[
        "       ###",
        "####    ##",
        "###### ###",
        "#####  ###",
        "##### ####",
        "####### ##",
        "######  ##",
        "###### ###",
        "##### ####",
        "####  ####",
        "#### #####",
        "#####   ##",
        "####### ##",
        "######  ##",
        "###### ###",
    ],
    &[
        "       ###",
        "####    ##",
        "####### ##",
        "######  ##",
        "###### ###",
        "##### ####",
        "####  ####",
        "#### #####",
        "#####   ##",
        "####### ##",
        "######  ##",
        "###### ###",
    ],
    &[
        "       ###",
        "####    ##",
        "##### ####",
        "####  ####",
        "#### #####",
        "#####   ##",
        "####### ##",
        "######  ##",
        "###### ###",
    ],
    &[
        "       ###",
        "####    ##",
        "#####   ##",
        "####### ##",
        "######  ##",
        "###### ###",
    ],
    &[
        //
        "       ###",
        "####    ##",
        "#####   ##",
    ],
];

#[test]
fn b2b_zst() {
    let indented_hang =
        Board::from_strs_with_piece(&B2B_ZST[0], Bag::new(0), Piece::new(PieceType::Z));
    let normal_hang =
        Board::from_strs_with_piece(&B2B_ZST[1], Bag::new(0), Piece::new(PieceType::Z));
    let roofless_kick =
        Board::from_strs_with_piece(&B2B_ZST[2], Bag::new(0), Piece::new(PieceType::Z));
    let double_hang =
        Board::from_strs_with_piece(&B2B_ZST[3], Bag::new(0), Piece::new(PieceType::Z));
    let last = Board::from_strs(&B2B_ZST[4], Bag::new(0));

    spin_test(indented_hang, normal_hang.clone(), "SRsRCH");
    spin_test(normal_hang, roofless_kick.clone(), "SRsRCH");
    spin_test(roofless_kick, double_hang.clone(), "SRsLcH");
    spin_test(double_hang, last, "SRsRCH");
}

#[test]
fn b2b_sst() {
    let indented_hang = reverse_board(Board::from_strs_with_piece(
        &B2B_ZST[0],
        Bag::new(0),
        Piece::new(PieceType::S),
    ));

    let normal_hang = reverse_board(Board::from_strs_with_piece(
        &B2B_ZST[1],
        Bag::new(0),
        Piece::new(PieceType::S),
    ));

    let roofless_kick = reverse_board(Board::from_strs_with_piece(
        &B2B_ZST[2],
        Bag::new(0),
        Piece::new(PieceType::S),
    ));

    let double_hang = reverse_board(Board::from_strs_with_piece(
        &B2B_ZST[3],
        Bag::new(0),
        Piece::new(PieceType::S),
    ));

    let last = reverse_board(Board::from_strs(&B2B_ZST[4], Bag::new(0)));

    spin_test(indented_hang, normal_hang.clone(), "SLsLcH");
    spin_test(normal_hang, roofless_kick.clone(), "SLsLcH");
    spin_test(roofless_kick, double_hang.clone(), "SLsRCH");
    spin_test(double_hang, last, "SLsLcH");
}

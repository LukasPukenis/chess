use chess::{Board, Kind};

use crate::chess::{render_board};

mod chess;

fn main() {
    let board = Board::default();
    render_board(&board, Kind::Black).unwrap();
    println!();
    render_board(&board, Kind::White).unwrap();
}

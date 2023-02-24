use core::fmt;

use anyhow::anyhow;
use std::{
    ops::{Deref, DerefMut},
};

// TODO: instead of option make it an enum of Piece
#[derive(Clone, Debug, PartialEq)]
pub enum Piece {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Kind {
    White,
    Black,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Pair {
    pub kind: Kind,
    pub piece: Piece,
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.kind {
            Kind::White => {
                match self.piece {
                    Piece::Rook => f.write_str("♖")?,
                    Piece::Knight => f.write_str("♘")?,
                    Piece::Bishop => f.write_str("♗")?,
                    Piece::Queen => f.write_str("♕")?,
                    Piece::King => f.write_str("♔")?,
                    Piece::Pawn => f.write_str("♙")?,
                };
            }
            Kind::Black => {
                match self.piece {
                    Piece::Rook => f.write_str("♜")?,
                    Piece::Knight => f.write_str("♞")?,
                    Piece::Bishop => f.write_str("♝")?,
                    Piece::Queen => f.write_str("♛")?,
                    Piece::King => f.write_str("♚")?,
                    Piece::Pawn => f.write_str("♟")?,
                };
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub struct Board([[Option<Pair>; 8]; 8]);

impl Board {
    pub fn from_data(data: [[Option<Pair>; 8]; 8]) -> Board {
        Board { 0: data }
    }
}
// TODO: Option is not great in board state
impl Deref for Board {
    type Target = [[Option<Pair>; 8]; 8];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

type Position = (usize, usize);

impl Board {
    pub fn new() -> Board {
        Board {
            0: [
                // TODO concise way to do this?
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
                [None, None, None, None, None, None, None, None],
            ],
        }
    }

    // get all valid moves for a piece
    pub fn all_moves(&self, pos: Position) -> Vec<Position> {
        let pair = self[pos.0][pos.1].clone().expect("Piece must be present");

        enum piece_match {
            None,
            Same,
            Different,
        }

        let checker = |pos: Position| -> piece_match {
            match &self[pos.0][pos.1] {
                Some(p) => {
                    if pair.kind == p.kind {
                        piece_match::Same
                    } else {
                        piece_match::Different
                    }
                }
                None => piece_match::None,
            }
        };

        match pair.piece {
            Piece::Rook => {
                let mut moves = Vec::new();

                // rook can move in four directions
                // up
                for i in (0..pos.0).rev() {
                    let p = (i, pos.1);
                    match checker(p) {
                        piece_match::None => moves.push(p),
                        piece_match::Different => {
                            moves.push(p);
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // down
                for i in pos.0 + 1..8 {
                    let p = (i, pos.1);
                    match checker(p) {
                        piece_match::None => moves.push(p),
                        piece_match::Different => {
                            moves.push(p);
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // left
                for i in (0..pos.1).rev() {
                    let p = (pos.0, i);
                    match checker(p) {
                        piece_match::None => moves.push(p),
                        piece_match::Different => {
                            moves.push(p);
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // right
                for i in pos.1 + 1..8 {
                    let p = (pos.0, i);
                    match checker(p) {
                        piece_match::None => moves.push(p),
                        piece_match::Different => {
                            moves.push(p);
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                moves
            }
            Piece::Knight => {
                // knight moves in L shape, 2 steps in one direction and 1 to the side
                let mut moves = Vec::new();

                // up
                if pos.0 > 1 {
                    // left
                    if pos.1 > 0 {
                        let p = (pos.0 - 2, pos.1 - 1);
                        match checker(p) {
                            piece_match::None => moves.push(p),
                            piece_match::Different => moves.push(p),
                            piece_match::Same => (),
                        }
                    }

                    // right
                    if pos.1 < 7 {
                        let p = (pos.0 - 2, pos.1 + 1);
                        match checker(p) {
                            piece_match::None => moves.push(p),
                            piece_match::Different => moves.push(p),
                            piece_match::Same => (),
                        }
                    }
                }

                // down
                if pos.0 < 6 {
                    // left
                    if pos.1 > 0 {
                        let p = (pos.0 + 2, pos.1 - 1);
                        match checker(p) {
                            piece_match::None => moves.push(p),
                            piece_match::Different => moves.push(p),
                            piece_match::Same => (),
                        }
                    }

                    // right
                    if pos.1 < 7 {
                        let p = (pos.0 + 2, pos.1 + 1);
                        match checker(p) {
                            piece_match::None => moves.push(p),
                            piece_match::Different => moves.push(p),
                            piece_match::Same => (),
                        }
                    }
                }

                // left
                if pos.1 > 1 {
                    // up
                    if pos.0 > 0 {
                        let p = (pos.0 - 1, pos.1 - 2);
                        match checker(p) {
                            piece_match::None => moves.push(p),
                            piece_match::Different => moves.push(p),
                            piece_match::Same => (),
                        }
                    }

                    // down
                    if pos.0 < 7 {
                        let p = (pos.0 + 1, pos.1 - 2);
                        match checker(p) {
                            piece_match::None => moves.push(p),
                            piece_match::Different => moves.push(p),
                            piece_match::Same => (),
                        }
                    }
                }

                // right
                if pos.1 < 6 {
                    // up
                    if pos.0 > 0 {
                        let p = (pos.0 - 1, pos.1 + 2);
                        match checker(p) {
                            piece_match::None => moves.push(p),
                            piece_match::Different => moves.push(p),
                            piece_match::Same => (),
                        }
                    }

                    // down
                    if pos.0 < 7 {
                        let p = (pos.0 + 1, pos.1 + 2);
                        match checker(p) {
                            piece_match::None => moves.push(p),
                            piece_match::Different => moves.push(p),
                            piece_match::Same => (),
                        }
                    }
                }

                return moves;
            }
            Piece::Bishop => {
                let mut moves = Vec::new();

                // bishop can move in four directions
                // up left
                for i in (0..pos.0).rev() {
                    let p1: i32 = pos.1.try_into().unwrap();
                    let p0: i32 = pos.0.try_into().unwrap();
                    let j = p1 - (p0 - i as i32);
                    if j < 0 {
                        break;
                    }

                    match checker((i, j as usize)) {
                        piece_match::None => moves.push((i, j as usize)),
                        piece_match::Different => {
                            moves.push((i, j as usize));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // up right
                for i in (0..pos.0).rev() {
                    let j = pos.1 + (pos.0 - i);
                    if j > 7 {
                        break;
                    }
                    match checker((i, j)) {
                        piece_match::None => moves.push((i, j)),
                        piece_match::Different => {
                            moves.push((i, j));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // down left
                for i in (pos.0 + 1)..8 {
                    let p1: i32 = pos.1.try_into().unwrap();
                    let p0: i32 = pos.0.try_into().unwrap();

                    let j = p1 - (i as i32 - p0);
                    if j < 0 {
                        break;
                    }

                    match checker((i, j as usize)) {
                        piece_match::None => moves.push((i, j as usize)),
                        piece_match::Different => {
                            moves.push((i, j as usize));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // down right
                for i in pos.0 + 1..8 {
                    let j = pos.1 + (i - pos.0);
                    if j > 7 {
                        break;
                    }

                    match checker((i, j)) {
                        piece_match::None => moves.push((i, j)),
                        piece_match::Different => {
                            moves.push((i, j));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                moves
            }
            Piece::Queen => {
                let mut moves = Vec::new();

                // queen can move in eight directions
                // up
                for i in (0..pos.0).rev() {
                    match checker((i, pos.1)) {
                        piece_match::None => moves.push((i, pos.1)),
                        piece_match::Different => {
                            moves.push((i, pos.1));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // down
                for i in pos.0 + 1..8 {
                    match checker((i, pos.1)) {
                        piece_match::None => moves.push((i, pos.1)),
                        piece_match::Different => {
                            moves.push((i, pos.1));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // left
                for i in (0..pos.1).rev() {
                    match checker((pos.0, i)) {
                        piece_match::None => moves.push((pos.0, i)),
                        piece_match::Different => {
                            moves.push((pos.0, i));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // right
                for i in pos.1 + 1..8 {
                    match checker((pos.0, i)) {
                        piece_match::None => moves.push((pos.0, i)),
                        piece_match::Different => {
                            moves.push((pos.0, i));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // up left
                for i in (0..pos.0).rev() {
                    let p1: i32 = pos.1.try_into().unwrap();
                    let p0: i32 = pos.0.try_into().unwrap();
                    let j = p1 - (p0 - i as i32);
                    if j < 0 {
                        break;
                    }

                    match checker((i, j as usize)) {
                        piece_match::None => moves.push((i, j as usize)),
                        piece_match::Different => {
                            moves.push((i, j as usize));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // up right
                for i in (0..pos.0).rev() {
                    let j = pos.1 + (pos.0 - i);
                    if j > 7 {
                        break;
                    }
                    match checker((i, j)) {
                        piece_match::None => moves.push((i, j)),
                        piece_match::Different => {
                            moves.push((i, j));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // down left
                for i in (pos.0 + 1)..8 {
                    let p1: i32 = pos.1.try_into().unwrap();
                    let p0: i32 = pos.0.try_into().unwrap();

                    let j = p1 - (i as i32 - p0);
                    if j < 0 {
                        break;
                    }

                    match checker((i, j as usize)) {
                        piece_match::None => moves.push((i, j as usize)),
                        piece_match::Different => {
                            moves.push((i, j as usize));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                // down right
                for i in pos.0 + 1..8 {
                    let j = pos.1 + (i - pos.0);
                    if j > 7 {
                        break;
                    }

                    match checker((i, j)) {
                        piece_match::None => moves.push((i, j)),
                        piece_match::Different => {
                            moves.push((i, j));
                            break;
                        }
                        piece_match::Same => break,
                    }
                }

                moves
            }
            Piece::King => {
                let mut moves = Vec::new();

                // king can move in eight directions
                // up
                if pos.0 > 0 {
                    match checker((pos.0 - 1, pos.1)) {
                        piece_match::None => moves.push((pos.0 - 1, pos.1)),
                        piece_match::Different => moves.push((pos.0 - 1, pos.1)),
                        piece_match::Same => (),
                    }
                }

                // down
                if pos.0 < 7 {
                    match checker((pos.0 + 1, pos.1)) {
                        piece_match::None => moves.push((pos.0 + 1, pos.1)),
                        piece_match::Different => moves.push((pos.0 + 1, pos.1)),
                        piece_match::Same => (),
                    }
                }

                // left
                if pos.1 > 0 {
                    match checker((pos.0, pos.1 - 1)) {
                        piece_match::None => moves.push((pos.0, pos.1 - 1)),
                        piece_match::Different => moves.push((pos.0, pos.1 - 1)),
                        piece_match::Same => (),
                    }
                }

                // right
                if pos.1 < 7 {
                    match checker((pos.0, pos.1 + 1)) {
                        piece_match::None => moves.push((pos.0, pos.1 + 1)),
                        piece_match::Different => moves.push((pos.0, pos.1 + 1)),
                        piece_match::Same => (),
                    }
                }

                // up left
                if pos.0 > 0 && pos.1 > 0 {
                    match checker((pos.0 - 1, pos.1 - 1)) {
                        piece_match::None => moves.push((pos.0 - 1, pos.1 - 1)),
                        piece_match::Different => moves.push((pos.0 - 1, pos.1 - 1)),
                        piece_match::Same => (),
                    }
                }

                // up right
                if pos.0 > 0 && pos.1 < 7 {
                    match checker((pos.0 - 1, pos.1 + 1)) {
                        piece_match::None => moves.push((pos.0 - 1, pos.1 + 1)),
                        piece_match::Different => moves.push((pos.0 - 1, pos.1 + 1)),
                        piece_match::Same => (),
                    }
                }

                // down left
                if pos.0 < 7 && pos.1 > 0 {
                    match checker((pos.0 + 1, pos.1 - 1)) {
                        piece_match::None => moves.push((pos.0 + 1, pos.1 - 1)),
                        piece_match::Different => moves.push((pos.0 + 1, pos.1 - 1)),
                        piece_match::Same => (),
                    }
                }

                // down right
                if pos.0 < 7 && pos.1 < 7 {
                    match checker((pos.0 + 1, pos.1 + 1)) {
                        piece_match::None => moves.push((pos.0 + 1, pos.1 + 1)),
                        piece_match::Different => moves.push((pos.0 + 1, pos.1 + 1)),
                        piece_match::Same => (),
                    }
                }

                moves
            }
            Piece::Pawn => {
                let mut moves = Vec::new();

                match pair.kind {
                    Kind::White => {
                        // white pawn can move up

                        // a pawn can move only one square forward
                        // a pawn can move diagonally forward if there's an enemy on the square
                        // a pawn can move two squares forward if it hasn't moved yet

                        // up
                        if pos.0 > 0 {
                            match checker((pos.0 - 1, pos.1)) {
                                piece_match::None => moves.push((pos.0 - 1, pos.1)),
                                piece_match::Different => (),
                                piece_match::Same => (),
                            }
                        }

                        // up left
                        if pos.0 > 0 && pos.1 > 0 {
                            match checker((pos.0 - 1, pos.1 - 1)) {
                                piece_match::None => (),
                                piece_match::Different => moves.push((pos.0 - 1, pos.1 - 1)),
                                piece_match::Same => (),
                            }
                        }

                        // up right
                        if pos.0 > 0 && pos.1 < 7 {
                            match checker((pos.0 - 1, pos.1 + 1)) {
                                piece_match::None => (),
                                piece_match::Different => moves.push((pos.0 - 1, pos.1 + 1)),
                                piece_match::Same => (),
                            }
                        }

                        // two squares up
                        if pos.0 == 6 {
                            match checker((pos.0 - 2, pos.1)) {
                                piece_match::None => moves.push((pos.0 - 2, pos.1)),
                                piece_match::Different => (),
                                piece_match::Same => (),
                            }
                        }
                    }
                    Kind::Black => {
                        // black pawn can move down

                        // a pawn can move only one square forward
                        // a pawn can move diagonally forward if there's an enemy on the square
                        // a pawn can move two squares forward if it hasn't moved yet

                        // down
                        if pos.0 < 7 {
                            match checker((pos.0 + 1, pos.1)) {
                                piece_match::None => moves.push((pos.0 + 1, pos.1)),
                                piece_match::Different => (),
                                piece_match::Same => (),
                            }
                        }

                        // down left
                        if pos.0 < 7 && pos.1 > 0 {
                            match checker((pos.0 + 1, pos.1 - 1)) {
                                piece_match::None => (),
                                piece_match::Different => moves.push((pos.0 + 1, pos.1 - 1)),
                                piece_match::Same => (),
                            }
                        }

                        // down right
                        if pos.0 < 7 && pos.1 < 7 {
                            match checker((pos.0 + 1, pos.1 + 1)) {
                                piece_match::None => (),
                                piece_match::Different => moves.push((pos.0 + 1, pos.1 + 1)),
                                piece_match::Same => (),
                            }
                        }

                        // two squares down
                        if pos.0 == 1 {
                            match checker((pos.0 + 2, pos.1)) {
                                piece_match::None => moves.push((pos.0 + 2, pos.1)),
                                piece_match::Different => (),
                                piece_match::Same => (),
                            }
                        }
                    }
                }

                moves
            }
        }
    }

    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), anyhow::Error> {
        let moves = self.all_moves(from);
        if moves.iter().find(|&&x| x == to).is_none() {
            return Err(anyhow!("Invalid move"));
        }

        match self[from.0][from.1] {
            Some(_) => {
                self[to.0][to.1] = self[from.0][from.1].clone();
                self[from.0][from.1] = None;
            }
            None => {
                panic!("how can we move a nonexisting piece?")
            }
        }
        Ok(())
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::new();

        // white
        for i in 0..8 {
            board[6][i] = Some(Pair {
                kind: Kind::White,
                piece: Piece::Pawn,
            })
        }

        board[7][0] = Some(Pair {
            kind: Kind::White,
            piece: Piece::Rook,
        });
        board[7][7] = Some(Pair {
            kind: Kind::White,
            piece: Piece::Rook,
        });

        board[7][1] = Some(Pair {
            kind: Kind::White,
            piece: Piece::Knight,
        });
        board[7][6] = Some(Pair {
            kind: Kind::White,
            piece: Piece::Knight,
        });

        board[7][2] = Some(Pair {
            kind: Kind::White,
            piece: Piece::Bishop,
        });
        board[7][5] = Some(Pair {
            kind: Kind::White,
            piece: Piece::Bishop,
        });

        board[7][3] = Some(Pair {
            kind: Kind::White,
            piece: Piece::Queen,
        });
        board[7][4] = Some(Pair {
            kind: Kind::White,
            piece: Piece::King,
        });

        // black

        for i in 0..8 {
            board[1][i] = Some(Pair {
                kind: Kind::Black,
                piece: Piece::Pawn,
            })
        }

        board[0][0] = Some(Pair {
            kind: Kind::Black,
            piece: Piece::Rook,
        });
        board[0][7] = Some(Pair {
            kind: Kind::Black,
            piece: Piece::Rook,
        });

        board[0][1] = Some(Pair {
            kind: Kind::Black,
            piece: Piece::Knight,
        });
        board[0][6] = Some(Pair {
            kind: Kind::Black,
            piece: Piece::Knight,
        });

        board[0][2] = Some(Pair {
            kind: Kind::Black,
            piece: Piece::Bishop,
        });
        board[0][5] = Some(Pair {
            kind: Kind::Black,
            piece: Piece::Bishop,
        });

        board[0][3] = Some(Pair {
            kind: Kind::Black,
            piece: Piece::Queen,
        });
        board[0][4] = Some(Pair {
            kind: Kind::Black,
            piece: Piece::King,
        });

        board
    }
}

// test chess board by moving it
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board() {
        let mut board = Board::default();

        board.move_piece((1, 1), (1, 3)).unwrap();
        assert_eq!(
            board[1][3],
            Some(Pair {
                kind: Kind::Black,
                piece: Piece::Pawn,
            })
        );

        assert_eq!(board[1][1], None);

        board.move_piece((6, 1), (6, 4)).unwrap();
        assert_eq!(
            board[6][4],
            Some(Pair {
                kind: Kind::White,
                piece: Piece::Pawn,
            })
        );
        assert_eq!(board[6][1], None);
    }

    // test `all_moves`function for a rook
    #[test]

    // use default board but remove the pawns before test
    fn test_all_moves_rook() {
        let mut board = Board::default();

        for i in 0..8 {
            board[1][i] = None;
            board[6][i] = None;
        }

        // test rook
        assert_eq!(
            board.all_moves((0, 0)),
            vec![(1, 0), (2, 0), (3, 0), (4, 0), (5, 0), (6, 0), (7, 0)]
        );
        assert_eq!(
            board.all_moves((0, 7)),
            vec![(1, 7), (2, 7), (3, 7), (4, 7), (5, 7), (6, 7), (7, 7)]
        );
        assert_eq!(
            board.all_moves((7, 0)),
            vec![(6, 0), (5, 0), (4, 0), (3, 0), (2, 0), (1, 0), (0, 0)]
        );
        assert_eq!(
            board.all_moves((7, 7)),
            vec![(6, 7), (5, 7), (4, 7), (3, 7), (2, 7), (1, 7), (0, 7)]
        );
    }

    #[test]
    fn test_all_moves_knight() {
        let mut board = Board::default();

        // move all knights to the center
        // test each knights possible moves out of starting position

        // test knight
        assert_eq!(board.all_moves((0, 1)), vec![(2, 0), (2, 2)]);
        assert_eq!(board.all_moves((0, 6)), vec![(2, 5), (2, 7)]);
        assert_eq!(board.all_moves((7, 1)), vec![(5, 0), (5, 2)]);
        assert_eq!(board.all_moves((7, 6)), vec![(5, 5), (5, 7)]);

        // move knights to the center
        board.move_piece((0, 1), (4, 4)).unwrap();

        assert_eq!(
            board.all_moves((4, 4)).sort(),
            vec![
                (6, 3),
                (6, 5),
                (5, 2),
                (5, 6),
                (3, 2),
                (3, 6),
                (2, 3),
                (2, 5)
            ]
            .sort()
        );
    }

    #[test]
    fn test_all_moves_bishop() {
        let mut board = Board::default();

        // test that bishops initially have no valid moves
        assert_eq!(board.all_moves((0, 2)), vec![]);
        assert_eq!(board.all_moves((0, 5)), vec![]);
        assert_eq!(board.all_moves((7, 2)), vec![]);
        assert_eq!(board.all_moves((7, 5)), vec![]);

        // move one bishop to the center
        board.move_piece((0, 2), (4, 4)).unwrap();

        // test that centered bishop has valid moves
        assert_eq!(
            board.all_moves((4, 4)),
            vec![
                (3, 3),
                (2, 2),
                (3, 5),
                (2, 6),
                (5, 3),
                (6, 2),
                (5, 5),
                (6, 6)
            ]
        );
    }

    #[test]
    fn test_all_moves_queen() {
        let mut board = Board::default();

        // test that queens initially have no valid moves
        assert_eq!(board.all_moves((0, 3)), vec![]);
        assert_eq!(board.all_moves((7, 3)), vec![]);

        // move one queen to the center
        board.move_piece((0, 3), (4, 4)).unwrap();

        // test that centered queen has valid moves
        let mut moves = board.all_moves((4, 4));
        let mut looking_for_moves = vec![
            (2, 2),
            (2, 4),
            (2, 6),
            (3, 3),
            (3, 4),
            (3, 5),
            (4, 0),
            (4, 1),
            (4, 2),
            (4, 3),
            (4, 5),
            (4, 6),
            (4, 7),
            (5, 3),
            (5, 4),
            (5, 5),
            (6, 2),
            (6, 4),
            (6, 6),
        ];

        looking_for_moves.sort();
        moves.sort();

        assert_eq!(moves, looking_for_moves);
    }

    #[test]
    fn test_all_moves_king() {
        let mut board = Board::default();

        // test that kings initially have no valid moves
        assert_eq!(board.all_moves((0, 4)), vec![]);
        assert_eq!(board.all_moves((7, 4)), vec![]);

        // move one king to the center
        board.move_piece((0, 4), (4, 4)).unwrap();

        // test that centered king has valid moves
        let mut moves = board.all_moves((4, 4));
        let mut looking_for_moves = vec![
            (3, 3),
            (3, 4),
            (3, 5),
            (4, 3),
            (4, 5),
            (5, 3),
            (5, 4),
            (5, 5),
        ];

        looking_for_moves.sort();
        moves.sort();

        assert_eq!(moves, looking_for_moves);
    }

    #[test]
    fn test_all_moves_pawn_white() {
        let board = Board::default();

        // assert that all pawns initially have valid moves
        for i in 0..8 {
            assert_eq!(board.all_moves((1, i)), vec![(2, i), (3, i)]);
        }

        {
            // move a pawn up by one square and assert it can only move by one square from then on
            let mut board = Board::default();

            board.move_piece((1, 0), (2, 0)).unwrap();
            assert_eq!(board.all_moves((2, 0)), vec![(3, 0)]);

            board.move_piece((2, 0), (3, 0)).unwrap();
            assert_eq!(board.all_moves((3, 0)), vec![(4, 0)]);
        }
    }

    #[test]
    fn test_all_moves_pawn_black() {
        {
            let board = Board::default();

            // assert that all black pawns can move
            for i in 0..8 {
                assert_eq!(board.all_moves((6, i)), vec![(5, i), (4, i)]);
            }
        }

        {
            // move a pawn down by one square and assert it can only move by one square from then on
            let mut board = Board::default();

            board.move_piece((6, 0), (5, 0)).unwrap();
            assert_eq!(board.all_moves((5, 0)), vec![(4, 0)]);

            board.move_piece((5, 0), (4, 0)).unwrap();
            assert_eq!(board.all_moves((4, 0)), vec![(3, 0)]);
        }
    }
}

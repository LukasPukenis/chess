use core::fmt;

use serde::Serialize;
use std::{
    error::Error,
    ops::{Deref, DerefMut},
};

// TODO: instead of option make it an enum of Piece
#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum Piece {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
pub enum Kind {
    White,
    Black,
}

#[derive(Clone, Debug, Serialize, PartialEq)]
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

#[derive(Debug, Serialize)]
pub struct Board([[Option<Pair>; 8]; 8]);

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

    pub fn move_piece(&mut self, from: Position, to: Position) -> Result<(), Box<dyn Error>> {
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
}

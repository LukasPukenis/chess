use core::fmt;

#[derive(Clone, Debug)]
pub enum Piece {
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
    Pawn,
}

#[derive(Clone, Debug)]
pub enum Kind {
    White,
    Black,
}

#[derive(Clone, Debug)]
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

use crossterm::{
    style::{Color, Print, ResetColor, SetBackgroundColor},
    ExecutableCommand, Result,
};
use std::{
    io::stdout,
    ops::{Deref, DerefMut},
};

pub struct Board(Vec<Vec<Option<Pair>>>);

impl Deref for Board {
    type Target = Vec<Vec<Option<Pair>>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Board {
    pub fn new() -> Board {
        Board(Vec::new())
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut board = Board::new();

        for _i in 0..8 {
            let mut j = Vec::new();
            j.resize(8, None);
            board.push(j);
        }

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

pub fn render_board(board: &Board, perspective: Kind) -> Result<()> {
    let inner = |i: usize, j: usize, color: Color| -> Result<()> {
        let ascii = match &board[i][j] {
            Some(pair) => {
                format!("{}", pair)
            }
            None => " ".to_string(),
        };

        stdout()
            .execute(SetBackgroundColor(color))?
            .execute(Print(ascii))?
            .execute(ResetColor)?;

        Ok(())
    };

    match perspective {
        Kind::White => {
            for i in 0..8 {
                for j in 0..8 {
                    let color = match (j + i) % 2 == 0 {
                        true =>  Color::White,
                        false =>  Color::Grey,
                    };

                    inner(i, j, color)?
                }
                println!();
            }
        }

        Kind::Black => {
            for i in (0..8).rev() {
                for j in (0..8).rev() {
                    let color = 
                    match (j + i) % 2 != 0 {
                        true => Color::White,
                        false => Color::Grey,
                    };

                    inner(i, j, color)?
                }
                println!();
            }
        }
    }

    Ok(())
}

use std::collections::HashMap;

use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Debug)]
// a1, h8, etc
pub struct RequestMove {
    from: String,
    to: String,
}

impl RequestMove {
    pub fn from(&self) -> String {
        self.from.clone()
    }

    pub fn to(&self) -> String {
        self.to.clone()
    }
}

#[derive(Serialize, Debug)]
struct BoardItem {
    piece: String,
    color: String,
}

#[derive(Serialize, Debug)]
pub struct Board {
    #[serde(flatten)]
    board: HashMap<String, BoardItem>,
}

use crate::chess;
use crate::chess::Pair;

// TODO: chess::Board() is a wrapper but as its inside mutex, dereferencing mutexguard causes it to be dereferenced and type is missing after that when invoking this trait
impl From<[[Option<Pair>; 8]; 8]> for Board {
    fn from(board: [[Option<Pair>; 8]; 8]) -> Self {
        let mut board_map = HashMap::new();
        for row in 0..8 {
            for col in 0..8 {
                let item = &board[row][col];
                if let Some(item) = item {
                    let color = if item.kind == chess::Kind::White {
                        "white"
                    } else {
                        "black"
                    };

                    let piece = match item.piece {
                        chess::Piece::Pawn => "pawn",
                        chess::Piece::Knight => "knight",
                        chess::Piece::Bishop => "bishop",
                        chess::Piece::Rook => "rook",
                        chess::Piece::Queen => "queen",
                        chess::Piece::King => "king",
                    };
                    let piece = piece.to_string();
                    let pos = format!("{}{}", (col as u8 + 'a' as u8) as char, 8 - row);
                    board_map.insert(
                        pos,
                        BoardItem {
                            piece,
                            color: color.to_string(),
                        },
                    );
                }
            }
        }
        Board { board: board_map }
    }
}

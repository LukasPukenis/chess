use chess::Board;
mod api;
mod chess;

use serde::Deserialize;
use std::sync::{Arc, Mutex};

#[derive(Deserialize, Debug)]
// a1, h8, etc
struct RequestMove {
    from: String,
    to: String,
}

use warp::Filter;

#[tokio::main]
async fn main() {
    let board = Arc::new(Mutex::new(Board::default()));

    let board_clone_get_board = board.clone();
    let board_clone_get_moves = board.clone();
    let board_clone_post = board.clone();

    use crate::api::Board as ApiBoard;
    use std::convert::From;

    let get_board_route = warp::path("board").map(move || {
        let board = board_clone_get_board.lock().unwrap();
        let api_board: ApiBoard = ApiBoard::from(board.clone());
        warp::reply::json(&api_board)
    });

    let get_moves_route = warp::path!("moves" / String).map(move |pos: String| {
        let board = board_clone_get_moves.lock().unwrap().clone();
        let convpos = |s: &str| {
            let mut chars = s.chars();
            let col = chars.next().unwrap() as usize - 'a' as usize;
            let row = 7 - (chars.next().unwrap() as usize - '1' as usize);
            (row, col)
        };

        let moves = Board::from_data(board).all_moves(convpos(&pos.clone()));
        // convert moves to chess notation
        let moves = moves
            .iter()
            .map(|m| format!("{}{}", (m.1 as u8 + 'a' as u8) as char, 8 - m.0))
            .collect::<Vec<String>>();

        warp::reply::json(&moves)
    });

    let pdir = std::env::current_dir().unwrap();
    let dir = pdir.to_string_lossy();

    let get_static_route = warp::path("static").and(warp::fs::dir(dir.to_string()));

    let post_move_route = warp::post()
        .and(warp::path("move"))
        .and(warp::body::json())
        .map(move |r: RequestMove| {
            let mut board = board_clone_post.lock().unwrap();
            let convpos = |s: &str| {
                let mut chars = s.chars();
                let col = chars.next().unwrap() as usize - 'a' as usize;
                let row = 7 - (chars.next().unwrap() as usize - '1' as usize);
                (row, col)
            };

            board.move_piece(convpos(&r.from), convpos(&r.to)).unwrap();
            warp::reply::reply()
        });

    let routes = post_move_route
        .or(warp::get().and(get_board_route))
        .or(warp::get().and(get_moves_route))
        .or(warp::get().and(get_static_route));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

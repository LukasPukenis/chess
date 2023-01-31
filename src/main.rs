use chess::Board;
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

    let board_clone_get = board.clone();
    let board_clone_post = board.clone();
    let get_board = warp::path("board").map(move || {
        let board = board_clone_get.lock().unwrap();
        let board = board.clone();
        warp::reply::json(&board)
    });

    let pdir = std::env::current_dir().unwrap();
    let dir = pdir.to_string_lossy();
    let static_route = warp::path("static").and(warp::fs::dir(dir.to_string()));
    let routes = warp::post()
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
        })
        .or(warp::get().and(get_board))
        .or(warp::get().and(static_route));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

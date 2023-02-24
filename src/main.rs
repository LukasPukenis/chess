use chess::Board;
mod api;
mod chess;

use std::{
    convert::Infallible,
    sync::{Arc, Mutex},
};
use warp::{hyper::StatusCode, Filter};

use crate::api::RequestMove;

#[derive(Debug)]
struct InvalidMove {}

impl warp::reject::Reject for InvalidMove {}

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

fn with_board(
    board: Arc<Mutex<Board>>,
) -> impl Filter<Extract = (Arc<Mutex<Board>>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || board.clone())
}

async fn post_move_route(
    b: Arc<Mutex<Board>>,
    r: RequestMove,
) -> Result<impl warp::Reply, Infallible> {
    let convpos = |s: &str| {
        let mut chars = s.chars();
        let col = chars.next().unwrap() as usize - 'a' as usize;
        let row = 7 - (chars.next().unwrap() as usize - '1' as usize);
        (row, col)
    };

    let mut board = b.lock().unwrap();
    let r = r.clone();
    let res = board.move_piece(convpos(&r.from().clone()), convpos(&r.to().clone()));

    match res {
        Ok(_) => Ok(StatusCode::OK),
        Err(_e) => Ok(StatusCode::BAD_REQUEST),
    }
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let board = Arc::new(Mutex::new(Board::default()));

    let board_clone_get_board = board.clone();
    let board_clone_get_moves = board.clone();
    let _board_clone_post = board.clone();

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

    let routes = warp::post()
        .and(warp::path("move"))
        .and(with_board(board.clone()))
        .and(warp::body::json())
        .and_then(post_move_route)
        .or(warp::get().and(get_board_route))
        .or(warp::get().and(get_moves_route))
        .or(warp::get().and(get_static_route));

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

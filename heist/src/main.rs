use std::str::FromStr;
mod board;
use axum::{routing::post, Json, Router};
use serde_json::{json, Value};

async fn rob_boggle(Json(board): Json<board::Board>) -> Result<Json<Value>, String> {
    println!("{:?}", board);
    Ok(Json(json!(board.good_words())))
}

async fn rob_boggle_str(Json(board): Json<String>) -> Result<Json<Value>, String> {
    println!("Incoming request to solve Boggle: {board:?}");
    let board = board::Board::from_str(&board)?;
    Ok(Json(json!(board.good_words())))
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/rob_harder", post(rob_boggle))
        .route("/rob", post(rob_boggle_str));

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Launched server");
    axum::serve(listener, app).await.unwrap();
}

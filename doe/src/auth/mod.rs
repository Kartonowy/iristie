use std::{sync::{Arc, Mutex}, vec};

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response, Result},
};
use axum_session::{SessionNullPool, Session};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

pub async fn auth(session: Session<SessionNullPool>, req: Request, next: Next) -> Result<Response, StatusCode> {

    let board = match session.get::<u32>("board") {
        Some(b) => b,
        None => {
            return Err(StatusCode::UNAUTHORIZED)
        }
    };

    let header = match req.headers().get("Authorization") {
        Some(b) => match b.to_str().unwrap().parse::<u32>() {
            Ok(e) => e,
            Err(_) => { return Err(StatusCode::UNAUTHORIZED) }
        },
        None => {
            return Err(StatusCode::UNAUTHORIZED)
        }
    };
    print!("Verifying if session exists: ");
    println!("Session: {}", board);
    println!("Header Authorzation: {}", header);

    if board == header {
        println!("Exists");
        return Ok(next.run(req).await)
    } 


    println!("Does not exist");

    Err(StatusCode::UNAUTHORIZED)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Board {
    id: u32,
    name: String,
}


pub async fn get_boards(State(state): State<Arc<Mutex<Connection>>>) -> impl IntoResponse {
    let state = state.lock().expect("Poisoned: Couldn't place lock on conn");
    let mut stmt = state
        .prepare("SELECT name, rowid FROM boards")
        .unwrap();
    let card_iter = stmt
        .query_map([], |row| {
            Ok(Board {
                name: row.get(0)?,
                id: row.get(1)?,
            })
        })
        .unwrap();
    let mut cards = vec![];
    for card in card_iter {
        cards.push(card.unwrap());
    }
    (StatusCode::OK, serde_json::to_string(&cards).unwrap()).into_response()
}

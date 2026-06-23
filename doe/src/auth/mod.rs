use std::{sync::{Arc, Mutex}, vec};

use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response, Result}, Json,
};
use axum_session::{SessionNullPool, Session};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Auth {
    id: u32,
    pass: String
}


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
    (StatusCode::OK, Json(&cards)).into_response()
}

pub async fn auth_handler(State(state): State<Arc<Mutex<Connection>>>, session: Session<SessionNullPool>, Json(payload): Json<Auth>) -> impl IntoResponse {
    let state = state.lock().expect("Poisoned");

    if payload.id == 0 {
        return (StatusCode::NOT_FOUND, "what the hell is this board, id 0?? id -1??").into_response()
    }

    let sql = format!("SELECT pass FROM boards WHERE rowid = {}", payload.id);
    println!("{sql}");

    let mut stmt = state
        .prepare(&sql)
        .unwrap();
    let mut rows = stmt.query([]).unwrap();
    let mut names: Vec<String> = Vec::new();
    while let Some(row) = rows.next().unwrap() {
        names.push(row.get(0).unwrap());
    }

    if payload.pass == names[0] {
        print!("Password correct. Setting session... ");
        session.set("board", payload.id);
        println!("Session Set: {}", session.get("board").unwrap_or(0));
        return (StatusCode::OK, "Logged in").into_response()
    }
    
    (StatusCode::FORBIDDEN, "Wrong password bud").into_response()
}

use std::sync::{Arc, Mutex};

use axum::{Json, extract::{Path, State}, http::{self, StatusCode}, response::IntoResponse};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};


fn insert_into(conn: &Connection, trivia: Trivia) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO trivia (text) VALUES (?1, ?2)",
        (&trivia.text, false),
    )
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Trivia {
    text: String
}

pub async fn add_trivia(
    State(state): State<Arc<Mutex<Connection>>>,
    Json(payload): Json<Trivia>,
) -> impl IntoResponse {
    let state = state.lock().expect("Poisoned");
    println!("Got request with body: {:?}", payload);

    match insert_into(&state, payload) {
        Ok(_) => {
            (StatusCode::OK, "OK").into_response()
        },
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Couldn't insert into database {:?}", error),
        )
            .into_response(),
    }
}

pub async fn get_trivia(State(state): State<Arc<Mutex<Connection>>>) -> impl IntoResponse {
    print!("Printing trivia requested! ");
    let state = state.lock().expect("Poisoned: Couldn't place lock on conn");
    let mut stmt = state
        .prepare("SELECT rowid, text FROM trivia WHERE used = false order by random() LIMIT 1;")
        .unwrap();
    let mut trivia_iter = stmt
        .query_map([], |row| {
            Ok(Trivia {
                text: row.get(1)?,
            })
        })
        .unwrap();
    let mut trivia = trivia_iter.next().unwrap().unwrap();

    (StatusCode::OK, Json(trivia)).into_response()
}
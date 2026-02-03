use std::sync::{Arc, Mutex};

use axum::{extract::{Path, State}, http::StatusCode, response::IntoResponse, Json};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    alt: String,    // ALT always consists of character's name
    src: String,    // SRC meaning url to the image of the character
    series: String, // SERIES for where the character comes from
    tier: String,   // TIER for tier alignment purposes
    short: Option<String>, // SHORT for short description of the character, reasoning behind its placement
    board_id: u32, // id of the board to support multiple boards
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keyring<'a> {
    board_id: u32,
    alt: &'a str,
    series: &'a str,
}

impl<'a> Keyring<'a> {
    pub fn from_card(card: &Card) -> Keyring<'_> {
        Keyring {
            board_id: card.board_id,
            alt: &card.alt,
            series: &card.series,
        }
    }
    pub fn from(board_id: u32, alt: &'a str, series: &'a str) -> Keyring<'a> {
        Keyring { board_id, alt, series }
    }
}

pub fn insert_into(conn: &Connection, card: Card) -> rusqlite::Result<usize> {
    conn.execute(
        "INSERT INTO card (alt, src, series, tier, short, board_id) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (&card.alt, &card.src, &card.series, &card.tier, &card.short, card.board_id),
    )
}

pub fn delete(conn: &Connection, key: Keyring) -> rusqlite::Result<()> {
    conn.execute("DELETE FROM card WHERE alt = ?1 AND series = ?2 AND board_id = ?3", (&key.alt, &key.series, key.board_id))?;
    Ok(())
}

pub fn update(conn: &Connection, key: Keyring, new_card: Card) -> rusqlite::Result<()> {
    // IDEA: Add version control, where you can check what options were changed, why and when
    // smth like git
    //
    // also check if info is right
    conn.execute(
        "UPDATE card SET alt = ?1, src = ?2, series = ?3, tier = ?4, short = ?5 WHERE alt = ?6 AND series = ?7 AND board_id = ?8",
        (&new_card.alt, &new_card.src, &new_card.series, &new_card.tier, &new_card.short, &key.alt, &key.series, key.board_id),
    )?;
    Ok(())
}

pub async fn get_cards(State(state): State<Arc<Mutex<Connection>>>, Path(path): Path<String>) -> impl IntoResponse {
    print!("Printing cards requested! ");
    let state = state.lock().expect("Poisoned: Couldn't place lock on conn");
    let mut stmt = state
        .prepare(format!("SELECT alt, src, series, tier, short, board_id FROM card WHERE board_id = '{}'
                ORDER BY series, alt", path).as_str())
        .unwrap();
    let card_iter = stmt
        .query_map([], |row| {
            Ok(Card {
                alt: row.get(0)?,
                src: row.get(1)?,
                series: row.get(2)?,
                tier: row.get(3)?,
                short: row.get(4)?,
                board_id: row.get(5)?
            })
        })
        .unwrap();
    let mut cards = vec![];
    for card in card_iter {
        cards.push(card.unwrap());
    }
    print!("Success\n");

    (StatusCode::OK, serde_json::to_string(&cards).unwrap()).into_response()
}

pub async fn add_card(
    State(state): State<Arc<Mutex<Connection>>>,
    Json(payload): Json<Card>,
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

#[derive(Debug, Deserialize)]
pub struct ReqR {
    board_id: u32,
    alt: String,
    series: String,
}

pub async fn delete_card(
    State(state): State<Arc<Mutex<Connection>>>,
    Json(payload): Json<ReqR>,
) -> impl IntoResponse {
    let state = state.lock().expect("Poisoned");

    match delete(&state, Keyring::from(payload.board_id, &payload.alt, &payload.series)) {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Couldn't insert into database {:?}", error),
        )
            .into_response(),
    }
}


pub async fn update_card(
    State(state): State<Arc<Mutex<Connection>>>,
    Json(payload): Json<Card>,
) -> impl IntoResponse {
    let state = state.lock().expect("Poisoned");

    let kr = Keyring::from_card(&payload);

    match update(&state, kr, payload.clone()) {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Couldn't insert into database {:?}", error),
        )
            .into_response(),
    }
}

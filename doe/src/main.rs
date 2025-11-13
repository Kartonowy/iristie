use std::{sync::{Arc, Mutex}};

use axum::{extract::State, http::StatusCode, response::IntoResponse, routing::{get, post}, Json, Router};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

use crate::{auth::auth_pass, inserts::{delete, insert_into, update}};

pub mod inserts;
pub mod auth;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    alt: String,    // ALT always consists of character's name
    src: String,    // SRC meaning url to the image of the character
    series: String, // SERIES for where the character comes from
    tier: String,   // TIER for tier alignment purposes
    short: Option<String>, // SHORT for short description of the character, reasoning behind its placement
                           // TODO: source of the image?
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keyring<'a> {
    alt: &'a str,
    series: &'a str
}

impl<'a> Keyring<'a> {
    pub fn from_card(card: &Card) -> Keyring {
        Keyring {
            alt: &card.alt,
            series: &card.series
        }
    }
    pub fn from(alt: &'a str, series: &'a str) -> Keyring<'a> {
        Keyring { alt, series }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::open("./iris.db")?;

    let app = Router::new()
        .route("/", get(|| async { "Hello world" }))
        .route("/print", get(get_cards))
        .route("/add", post(add_card))
        .route("/delete", post(delete_card))
        .route("/update", post(update_card))
        .with_state(Arc::new(Mutex::new(conn)));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2299").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_cards(State(state): State<Arc<Mutex<Connection>>>) -> impl IntoResponse {
    let state = state.lock().expect("Poisoned: Couldn't place lock on conn");
    let mut stmt = state.prepare("SELECT alt, src, series, tier, short FROM card").unwrap();
    let card_iter = stmt.query_map([], |row| {
        Ok(Card {
            alt: row.get(0)?,
            src: row.get(1)?,
            series: row.get(2)?,
            tier: row.get(3)?,
            short: row.get(4)?,
        })
    }).unwrap();
    let mut cards = vec![];
    for card in card_iter {
        cards.push(card.unwrap());
    }
    println!("{:?}", cards);

    (StatusCode::OK, serde_json::to_string(&cards).unwrap()).into_response()
}

#[derive(Deserialize)]
struct ReqB {
    pass: String,
    card: Card
}

async fn add_card(State(state): State<Arc<Mutex<Connection>>>, Json(payload): Json<ReqB>) -> impl IntoResponse {
    if !auth_pass(payload.pass).is_ok() {
        return (StatusCode::FORBIDDEN, "Failed password authorization").into_response()
    }

    let state = state.lock().expect("Poisoned");


    match insert_into(&state, payload.card) {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Couldn't insert into database {:?}", error)).into_response(), 
    }
}

#[derive(Debug, Deserialize)]
struct ReqR {
    pass: String,
    alt: String,
    series: String
}

async fn delete_card(State(state): State<Arc<Mutex<Connection>>>, Json(payload): Json<ReqR>) -> impl IntoResponse {
    if !auth_pass(payload.pass).is_ok() {
        return (StatusCode::FORBIDDEN, "Failed password authorization").into_response()
    }

    let state = state.lock().expect("Poisoned");

    match delete(&state, Keyring::from(&payload.alt, &payload.series)) {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Couldn't insert into database {:?}", error)).into_response(), 
    }
}

#[derive(Debug, Deserialize)]
struct ReqU {
    pass: String,
    card: Card
}


async fn update_card(State(state): State<Arc<Mutex<Connection>>>, Json(payload): Json<ReqU>) -> impl IntoResponse {
    if !auth_pass(payload.pass).is_ok() {
        return (StatusCode::FORBIDDEN, "Failed password authorization").into_response()
    }

    let state = state.lock().expect("Poisoned");

    let kr = Keyring::from(&payload.card.alt, &payload.card.series);

    match update(&state, kr, payload.card.clone()) {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(error) => (StatusCode::INTERNAL_SERVER_ERROR, format!("Couldn't insert into database {:?}", error)).into_response(), 
    }
}

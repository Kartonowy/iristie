use std::sync::{Arc, Mutex};
use axum_session::{SessionConfig, SessionLayer, SessionNullPool, SessionStore, Session};
use http::header::{AUTHORIZATION, SET_COOKIE};

use axum::{
    body::Body, extract::{Request, State}, http::{self, HeaderMap, Method, StatusCode}, middleware::{self, Next}, response::{AppendHeaders, IntoResponse, Response}, routing::{get, post}, Json, Router
};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    auth::auth,
    inserts::{delete, insert_into, update},
};

pub mod auth;
pub mod inserts;

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
    series: &'a str,
}

impl<'a> Keyring<'a> {
    pub fn from_card(card: &Card) -> Keyring {
        Keyring {
            alt: &card.alt,
            series: &card.series,
        }
    }
    pub fn from(alt: &'a str, series: &'a str) -> Keyring<'a> {
        Keyring { alt, series }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let conn = Connection::open("./iris.db")?;

    let session_config = SessionConfig::default()
        .with_table_name("sessions_table");

    let session_store = SessionStore::<SessionNullPool>::new(None, session_config).await.unwrap();

    let app = Router::new()
        .route("/add", post(add_card))
        .route("/delete", post(delete_card))
        .route("/update", post(update_card))
        .route_layer(middleware::from_fn(auth))
        .route("/", get(|| async { "Hello world" }))
        .route("/auth", post(auth_handler))
        .route("/print", get(get_cards))
        .with_state(Arc::new(Mutex::new(conn)))
        .layer(
            ServiceBuilder::new()
            .layer(SessionLayer::new(session_store))
            .layer(
                CorsLayer::new()
                .allow_methods([Method::GET, Method::POST])
                .allow_headers([AUTHORIZATION, SET_COOKIE])
                .allow_origin(Any),
            ),
        );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:2299").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn get_cards(State(state): State<Arc<Mutex<Connection>>>) -> impl IntoResponse {
    print!("Printing cards requested! ");
    let state = state.lock().expect("Poisoned: Couldn't place lock on conn");
    let mut stmt = state
        .prepare("SELECT alt, src, series, tier, short FROM card")
        .unwrap();
    let card_iter = stmt
        .query_map([], |row| {
            Ok(Card {
                alt: row.get(0)?,
                src: row.get(1)?,
                series: row.get(2)?,
                tier: row.get(3)?,
                short: row.get(4)?,
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


async fn add_card(
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
struct ReqR {
    alt: String,
    series: String,
}

async fn delete_card(
    State(state): State<Arc<Mutex<Connection>>>,
    Json(payload): Json<ReqR>,
) -> impl IntoResponse {
    let state = state.lock().expect("Poisoned");

    match delete(&state, Keyring::from(&payload.alt, &payload.series)) {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Couldn't insert into database {:?}", error),
        )
            .into_response(),
    }
}


async fn update_card(
    State(state): State<Arc<Mutex<Connection>>>,
    Json(payload): Json<Card>,
) -> impl IntoResponse {
    let state = state.lock().expect("Poisoned");

    let kr = Keyring::from(&payload.alt, &payload.series);

    match update(&state, kr, payload.clone()) {
        Ok(_) => (StatusCode::OK, "OK").into_response(),
        Err(error) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Couldn't insert into database {:?}", error),
        )
            .into_response(),
    }
}
#[derive(Serialize, Deserialize, Debug)]
struct Auth {
    pass: String
}

async fn auth_handler(session: Session<SessionNullPool>, Json(payload): Json<Auth>) -> impl IntoResponse {
    println!("Checking password, got: {}", payload.pass);
    if payload.pass == "teto" {
        print!("Password correct. Setting session... ");
        session.set("logged", true);
        println!("Session Set: {}", session.get("logged").unwrap_or("error".to_string()));
        return (StatusCode::OK, "Logged in").into_response()
    }
    
    (StatusCode::FORBIDDEN, "Wrong password bud").into_response()
}

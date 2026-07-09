use std::sync::{Arc, Mutex};
use axum_session::{SessionConfig, SessionLayer, SessionNullPool, SessionStore};
use http::header::{AUTHORIZATION, SET_COOKIE};

use axum::{
    http::{self, Method}, middleware, routing::{get, post}, Router
};
use rusqlite::{Connection, Result};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::{
    auth::{auth, auth_handler, get_boards}, tier_cards::{add_card, delete_card, get_cards, update_card},
    low::{get_trivia}
};

pub mod auth;
pub mod tier_cards;
// pub mod vault;
pub mod low;


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
        .route("/trivia", get(get_trivia))
        .route("/", get(|| async { "Hello world" }))
        .route("/auth", post(auth_handler))
        .route("/print/{id}", get(get_cards))
        .route("/boards", get(get_boards))
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

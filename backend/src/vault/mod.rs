use axum::{Json, extract::{Path, State}, http::{self, StatusCode}, response::IntoResponse};
use rusqlite::Connection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VaultCard<'a> {
    name: &'a str,
    owner: &'a str,
    cardtype: &'a str,
    cost: 
}

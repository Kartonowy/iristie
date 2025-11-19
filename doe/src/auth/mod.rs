use std::{collections::HashMap, vec};

use async_trait::async_trait;
use axum::{
    extract::Request,
    http::StatusCode,
    middleware::Next,
    response::{Response, Result},
};
use axum_session::{SessionNullPool, Session};
use dotenv;
use scrypt::{
    Scrypt,
    password_hash::{self, PasswordVerifier},
};

pub fn auth_pass(pass: String) -> scrypt::password_hash::Result<()> {
    dotenv::dotenv().ok();

    let fromdot = dotenv::var("hash").unwrap();
    let hasz = password_hash::PasswordHash::new(&fromdot).expect("Couldnt process hash");
    Scrypt.verify_password(pass.as_bytes(), &hasz)
}

// pub async fn auth(req: Request, next: Next) -> Result<Response, StatusCode> {
//     match cookie.get("Auth") {
//         Some(cookie) => {
//             if cookie.value() == "teto" {
//                 Ok(next.run(req).await)
//             } else {
//                 Err(StatusCode::UNAUTHORIZED)
//             }
//         }
//         None => Err(StatusCode::UNAUTHORIZED),
//     }
// }
pub async fn auth(session: Session<SessionNullPool>, req: Request, next: Next) -> Result<Response, StatusCode> {
    print!("Verifying if session exists: ");
    if session.get("logged").unwrap_or(false) {
        println!("Exists");
        return Ok(next.run(req).await)
    } 
    println!("Does not exist");
    Err(StatusCode::UNAUTHORIZED)
}

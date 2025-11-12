use scrypt::{password_hash::{self, PasswordVerifier, Result}, Scrypt};
use dotenv;

pub fn auth_pass(pass: String) -> Result<()> {
    dotenv::dotenv().ok();

    let fromdot = dotenv::var("hash").unwrap();
    let hasz = password_hash::PasswordHash::new(&fromdot).expect("Couldnt process hash");
    Scrypt.verify_password(pass.as_bytes(), &hasz)
}


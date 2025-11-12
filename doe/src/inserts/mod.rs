use rusqlite::Connection;

use crate::{Card, Keyring};


pub fn insert_into(conn: &Connection, card: Card) -> rusqlite::Result<usize> {
    // let car = Card {
    //     id: 1,
    //     alt: "Kasane Teto".to_string(),
    //     src: "https://safebooru.org//samples/544/sample_0bdecd34eff76c04979cac41f3b9c054908736ea.jpg?".to_string(),
    //     series: "UTAU".to_string(),
    //     tier: "A".to_string(),
    //     short: Some("teto is a utau which is cool".to_string())
    // };

    conn.execute(
        "INSERT INTO card (alt, src, series, tier, short) VALUES (?1, ?2, ?3, ?4, ?5)",
        (&card.alt, &card.src, &card.series, &card.tier, &card.short),
    )
}

pub fn delete(conn: &Connection, key: Keyring) -> rusqlite::Result<()> {
    let mut stmt = conn.prepare("DELETE FROM card WHERE alt = ?1 AND series = ?2")?;
    stmt.execute([&key.alt, &key.series])?;
    Ok(())
}

pub fn update_tier(conn: &Connection, key: Keyring, tier: String) -> rusqlite::Result<()> {
    // IDEA: Add version control, where you can check what options were changed, why and when
    // smth like git
    let mut stmt = conn.prepare("UPDATE card SET tier = ?3 WHERE alt = ?1 AND series = ?2 ")?;
    stmt.execute([&key.alt, &key.series, &tier])?;
    Ok(())
}

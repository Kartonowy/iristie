use rusqlite::Connection;

use crate::{Card, Keyring};

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

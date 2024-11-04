use rusqlite::{params, Connection, Result};

#[derive(Debug)]
pub struct Player {
    pub id: i32,
    pub username: String,
    pub score: i32,
}

#[derive(Debug)]
pub struct Map {
    pub id: i32,
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub data: String,  // Représentation de la carte (JSON, etc.)
}

pub fn initialize_db() -> Result<Connection> {
    // Crée ou ouvre la base de données
    let conn = Connection::open("game_data.sqlite3")?;

    // Crée les tables players et maps si elles n'existent pas déjà
    conn.execute(
        "CREATE TABLE IF NOT EXISTS players (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            username TEXT NOT NULL UNIQUE,
            score INTEGER NOT NULL DEFAULT 0
        )",
        [],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS maps (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            width INTEGER NOT NULL,
            height INTEGER NOT NULL,
            data TEXT NOT NULL
        )",
        [],
    )?;

    Ok(conn)
}

pub fn add_player(conn: &Connection, username: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO players (username) VALUES (?1)",
        params![username],
    )?;
    Ok(())
}

pub fn get_players(conn: &Connection) -> Result<Vec<Player>> {
    let mut stmt = conn.prepare("SELECT id, username, score FROM players")?;
    let player_iter = stmt.query_map([], |row| {
        Ok(Player {
            id: row.get(0)?,
            username: row.get(1)?,
            score: row.get(2)?,
        })
    })?;

    player_iter.collect()
}

pub fn update_score(conn: &Connection, player_id: i32, new_score: i32) -> Result<()> {
    conn.execute(
        "UPDATE players SET score = ?1 WHERE id = ?2",
        params![new_score, player_id],
    )?;
    Ok(())
}

pub fn delete_player(conn: &Connection, player_id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM players WHERE id = ?1",
        params![player_id],
    )?;
    Ok(())
}
pub fn add_map(conn: &Connection, name: &str, width: i32, height: i32, data: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO maps (name, width, height, data) VALUES (?1, ?2, ?3, ?4)",
        params![name, width, height, data],
    )?;
    Ok(())
}

pub fn get_maps(conn: &Connection) -> Result<Vec<Map>> {
    let mut stmt = conn.prepare("SELECT id, name, width, height, data FROM maps")?;
    let map_iter = stmt.query_map([], |row| {
        Ok(Map {
            id: row.get(0)?,
            name: row.get(1)?,
            width: row.get(2)?,
            height: row.get(3)?,
            data: row.get(4)?,
        })
    })?;

    map_iter.collect()
}

pub fn delete_map(conn: &Connection, map_id: i32) -> Result<()> {
    conn.execute(
        "DELETE FROM maps WHERE id = ?1",
        params![map_id],
    )?;
    Ok(())
}

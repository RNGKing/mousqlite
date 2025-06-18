use rusqlite::{params, Connection};
use anyhow::Result;

pub fn delete_db(path: &std::path::Path) -> Result<(), std::io::Error> {
    if std::fs::exists(path)? {
        std::fs::remove_file(path)?;
    }
    Ok(())
}

pub fn create_db(path: &std::path::Path) -> Result<(), std::io::Error> {
    std::fs::File::create(path)?;
    Ok(())
}

pub fn connect_db(path: &std::path::Path) -> Result<Connection> {
    let conn = rusqlite::Connection::open(path)?;
    Ok(conn)
}

pub fn run_test_database_tas(conn : &Connection) -> Result<()> {
    conn.execute("CREATE TABLE IF NOT EXISTS test (id INTEGER PRIMARY KEY, first_name TEXT, last_name TEXT);", [])?;
    conn.execute("INSERT INTO test VALUES (?, ?, ?);", params![0, "James", "Johnson"])?;
    conn.execute("INSERT INTO test VALUES (?, ?, ?);", params![1, "Johan", "Jameson"])?;
    conn.execute("INSERT INTO test VALUES (?, ?, ?);", params![2, "Klementine", "Karson"])?;
    conn.execute("INSERT INTO test VALUES (?, ?, ?);", params![3, "Robert", "Bondage"])?;
    Ok(())
}

pub fn run_helper(path : &std::path::Path) -> Result<()> {
    delete_db(path)?;
    create_db(path)?;
    let conn = connect_db(path)?;
    run_test_database_tas(&conn)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {

    }
}

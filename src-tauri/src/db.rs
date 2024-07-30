use std::path::PathBuf;

use rusqlite::{params, Connection};
use rusqlite_migration::{Migrations, M};
use lazy_static::lazy_static;
use include_dir::{include_dir, Dir};


static MIGRATIONS_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/migrations");
// Define migrations. These are applied atomically.
lazy_static! {
    static ref MIGRATIONS: Migrations<'static> =
        Migrations::from_directory(&MIGRATIONS_DIR).unwrap();
}

fn get_conn(db_path: PathBuf) -> Result<Connection, rusqlite::Error> {
    match Connection::open(db_path.join("bluetime_main.db3")) {
        Ok(conn) => {
            conn.pragma_update(None, "foreign_keys", true);
            Ok(conn)
        },
        Err(err) => Err(err)
    }

}

fn migrate(conn: &mut Connection) {
    MIGRATIONS.to_latest(conn);
    conn.pragma_update(None, "journal_mode", "wal").expect("Failed to set journal_mode to wal");
    conn.pragma_update(None, "synchronous", "normal").expect("failed to set synchronity to normal");
}

use rusqlite::{Connection, Result};
use time::{self, Timespec};
use std::sync::{Mutex};

pub struct LinksDB {
    conn: Mutex<Connection>,
}

impl LinksDB {
    pub fn with_tables() -> Result<LinksDB> {
        let conn = try!(Connection::open_in_memory());
        conn.execute("CREATE TABLE link (
                      id              INTEGER PRIMARY KEY,
                      target          TEXT NOT NULL,
                      created_at      TEXT NOT NULL,
                      use_count       INTEGER
                      )", &[]).unwrap();

        Ok(LinksDB {
            conn: Mutex::new(conn),
        })
    }

    /// Returns id of inserted link on success.
    pub fn insert_link(&self, target: &str) -> Result<i64> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = try!(conn.prepare("INSERT INTO link (target, created_at) VALUES ($1, $2)"));
        let created_at = time::get_time();
        stmt.insert(&[&target, &created_at])
    }

    /// Returns the Link object that is associated with the target URL
    pub fn find_link(&self, id: i64) -> Result<Link> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = try!(conn.prepare("SELECT * FROM link WHERE id = $1"));
        let mut rows = try!(stmt.query_map(&[&id], |row| {
            Link {
                id: row.get(0),
                target: row.get(1),
                created_at: row.get(2),
            }
        }));

        rows.next().unwrap()
    }

    /// Returns the Link object that is associated with the target URL
    pub fn link_for_url(&self, target: &str) -> Result<Link> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = try!(conn.prepare("SELECT * FROM link WHERE target = $1"));
        let mut rows = try!(stmt.query_map(&[&target], |row| {
            Link {
                id: row.get(0),
                target: row.get(1),
                created_at: row.get(2),
            }
        }));

        rows.next().unwrap()
    }
}

//
// DATA MODELS
//
#[derive(Debug)]
pub struct Link {
    pub id: i64,
    pub target: String,
    pub created_at: Timespec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn insert_links() {
        let db = LinksDB::with_tables().unwrap();
        let id = db.insert_link("example.com").unwrap();
        assert_eq!(id, 1);
        let id = db.insert_link("example2.com").unwrap();
        assert_eq!(id, 2);
    }

    #[test]
    fn retrieve_link_for_url() {
        let db = LinksDB::with_tables().unwrap();
        let id = db.insert_link("example.com").unwrap();
        assert_eq!(id, 1);

        let link = db.link_for_url("example.com").unwrap();
        assert_eq!(link.target, String::from("example.com"));
    }
}

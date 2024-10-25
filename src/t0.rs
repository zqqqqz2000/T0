use crate::config::Config;
use duckdb::Connection;

#[derive(Debug)]
pub struct T0 {
    config: Config,
    duckdb_conn: Connection,
}

impl T0 {
    pub fn new(config: Config) -> Self {
        let conn = Connection::open_in_memory().expect("cannot open duckdb connection");
        Self {
            config,
            duckdb_conn: conn,
        }
    }
}

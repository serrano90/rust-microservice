use diesel::pg::PgConnection;
use diesel::prelude::*;

use crate::infrastructure::config::config::Config;

pub struct ConnectionManager {
    pub conn: PgConnection,
}

impl ConnectionManager {
    pub fn new(config: &Config) -> Self {
        let conn = PgConnection::establish(&config.database.url)
            .expect(&format!("Error connecting to {}", config.database.url));

        ConnectionManager { conn: conn }
    }
}

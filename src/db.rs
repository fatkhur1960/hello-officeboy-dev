use diesel::{pg::PgConnection, prelude::*};

pub fn connect(db_url: &str) -> PgConnection {
    PgConnection::establish(db_url).expect(&format!("Cannot connect to `{}`", db_url))
}

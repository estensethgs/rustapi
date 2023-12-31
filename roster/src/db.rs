//use crate::error_handler::CustomError;
use diesel::pg::PgConnection;
use diesel::prelude::*;
//use diesel::r2d2::ConnectionManager;
//use lazy_static::lazy_static;
use dotenvy::dotenv;
//use r2d2;
use std::env;

use crate::models::{NewPlayer, Player};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn create_player(conn: &mut PgConnection, name: &str) -> Player {
    use crate::schema::players;

    let new_player = NewPlayer { name };

    diesel::insert_into(players::table)
        .values(&new_player)
        .get_result(conn)
        .expect("Error saving new player")
}

/*
type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;
embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let database_url = env::var("DATABASE_URL").expect("Database URL must be set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
    };
}

pub fn init() {
    lazy_static::initialize(&POOL);
    let conn = connection().expect("Failed to get database connection.");
    embedded_migrations::run(&conn).expect("Failed to run database migrations.");
}

pub fn connection() -> Result<DbConnection, CustomError> {
    POOL.get().map_err(|e| CustomError::new(500, format!("Failed getting db connection: {}", e)))
}*/

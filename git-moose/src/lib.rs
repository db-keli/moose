pub mod github_client;
pub mod models;
pub mod issues;
pub mod commits;
pub mod repository;
pub mod database_models;
pub mod schema;
pub mod crud;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;


pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}
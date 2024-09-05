pub mod commits;
pub mod crud;
pub mod database_models;
pub mod github_client;
pub mod issues;
pub mod models;
pub mod repository;
pub mod schema;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;

pub const GITHUB_API_URL: &str = "https://api.github.com";

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = dotenvy::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

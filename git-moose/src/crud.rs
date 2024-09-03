use crate::database_models::Admin;
use diesel::prelude::*;
use crate::schema::admin;

pub fn create_admin(connection: &mut PgConnection, username: String, token: String) -> Admin {
    
    let admin = Admin { username, token };

    diesel::insert_into(admin::table)
        .values(&admin)
        .get_result(connection)
        .expect("Error saving new admin")
    
}
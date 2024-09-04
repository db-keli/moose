use crate::database_models::Admin;
use crate::schema::admin;
use diesel::prelude::*;

pub fn create_admin(connection: &mut PgConnection, username: String, token: String) -> Admin {
    let admin = Admin { username, token };

    diesel::insert_into(admin::table)
        .values(&admin)
        .get_result(connection)
        .expect("Error saving new admin")
}

pub fn get_admin(connection: &mut PgConnection, username: String) -> Admin {
    let admin = admin::table
        .filter(admin::username.eq(username))
        .first(connection)
        .expect("Error loading admin");

    admin
}

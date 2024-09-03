use diesel::prelude::*;
use crate::schema::admin;

#[derive(Queryable, Insertable)]
#[diesel(table_name = admin)]
pub struct Admin {
    pub username: String,
    pub token: String,
}
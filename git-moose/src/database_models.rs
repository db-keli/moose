use crate::schema::admin;
use diesel::prelude::*;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = admin)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Admin {
    pub username: String,
    pub token: String,
}

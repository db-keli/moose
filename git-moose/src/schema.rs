// @generated automatically by Diesel CLI.

diesel::table! {
    admin (username) {
        #[max_length = 255]
        username -> Varchar,
        token -> Varchar,
    }
}

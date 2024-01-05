// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Integer,
        username -> Text,
        hash_password -> Text,
        email -> Text,
        phone -> Text,
        role -> Integer,
    }
}

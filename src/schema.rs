diesel::table! {
    users (id) {
        id -> Integer,
        name -> Text,
        email -> Text,
        phone -> Text,
        password -> Text,
        hash_password -> Text,
    }
}
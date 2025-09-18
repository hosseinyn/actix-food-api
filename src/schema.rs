// @generated automatically by Diesel CLI.

diesel::table! {
    food (id) {
        id -> Integer,
        name -> Text,
        quantity -> Integer,
        price -> Integer,
        date -> Timestamp,
    }
}

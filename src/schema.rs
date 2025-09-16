// @generated automatically by Diesel CLI.

diesel::table! {
    Food (id) {
        id -> Integer,
        name -> Text,
        quantity -> Integer,
        price -> Integer,
        date -> Timestamp,
    }
}

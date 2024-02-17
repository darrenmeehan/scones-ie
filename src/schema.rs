// @generated automatically by Diesel CLI.

diesel::table! {
    toggles (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        enabled -> Bool,
    }
}

// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int4,
        #[sql_name = "type"]
        #[max_length = 255]
        client_type -> Varchar,
        #[max_length = 255]
        client_id -> Varchar,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        #[max_length = 255]
        website -> Varchar,
        #[max_length = 255]
        email -> Varchar,
    }
}

diesel::table! {
    redirect_uris (id) {
        id -> Int4,
        #[max_length = 255]
        uri -> Varchar,
        #[max_length = 255]
        client_id -> Varchar,
    }
}

diesel::table! {
    toggles (id) {
        id -> Int4,
        #[max_length = 255]
        name -> Varchar,
        description -> Text,
        enabled -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(clients, redirect_uris, toggles,);

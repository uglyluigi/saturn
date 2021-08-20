table! {
    clubs (id) {
        id -> Int4,
        maintainer -> Int4,
        title -> Varchar,
        body -> Text,
        publish_date -> Timestamptz,
        expiry_date -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    clubs,
    users,
);

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

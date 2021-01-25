table! {
    customers (id) {
        id -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        email -> Varchar,
        hotel_id -> Nullable<Int4>,
        created_at -> Timestamp,
    }
}

table! {
    resorts (id) {
        id -> Int4,
        name -> Varchar,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    customers,
    resorts,
);

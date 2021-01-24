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

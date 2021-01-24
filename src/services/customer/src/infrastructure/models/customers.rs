use crate::schema::customers;

#[derive(Debug, Insertable)]
#[table_name = "customers"]
pub struct NewCustomer {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub hotel_id: i32,
}

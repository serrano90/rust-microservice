/**
 * Customer DTO
 */
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Queryable, PartialEq, Debug)]
pub struct CustomerDTO {
    id: String,
    first_name: String,
    last_name: String,
    email: String,
    hotel_id: Option<i32>,
    created_at: SystemTime,
}

#[derive(Serialize)]
pub struct CustomerDTOList(pub Vec<CustomerDTO>);
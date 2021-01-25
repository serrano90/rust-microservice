/**
 * Hotel DTO
 */
use serde::Serialize;
use std::time::SystemTime;

#[derive(Serialize, Queryable, PartialEq, Debug)]
pub struct ResortDTO {
    id: i32,
    name: String,
    created_at: SystemTime,
}

#[derive(Serialize)]
pub struct ResortDTOList(pub Vec<ResortDTO>);


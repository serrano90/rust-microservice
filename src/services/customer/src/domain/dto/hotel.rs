/**
 * Hotel DTO
 */
use serde::Serialize;

#[derive(Serialize)]
pub struct HotelDTO {
    id: i32,
    name: String,
}
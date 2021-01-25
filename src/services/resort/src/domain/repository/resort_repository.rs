/**
 * Resort Repository Contract
 */
use crate::domain::entity::resort::Resort;
use crate::domain::dto::resort::ResortDTO;
use crate::domain::dto::resort::ResortDTOList;

pub trait ResortRepository {
    fn all(&self) -> ResortDTOList;
    fn find(&self, id: i32) -> ResortDTO;
    fn delete(&self, id: i32) -> bool;
    fn update(&self, resort: Resort) -> bool;
    fn create(&self, resort: Resort) -> ResortDTO;
}

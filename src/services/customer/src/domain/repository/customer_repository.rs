/**
 * Customer Repository Contract
 */
use crate::domain::entity::customer::Customer;
use crate::domain::dto::customer::CustomerDTO;
use crate::domain::dto::customer::CustomerDTOList;

pub trait CustomerRepository {
    fn all(&self, filter: String, page: i32, limit: i32) -> CustomerDTOList;
    fn create(&self, customer: Customer) -> CustomerDTO;
}

/**
 * Register Customer Command
 */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RegisterCustomerCommand {
    name: String,
    last_name: String,
    email: String,
    hotel_id: i32,
}

impl RegisterCustomerCommand {
    pub fn new(
        first_name: String,
        last_name: String,
        email: String,
        hotel_id: i32,
    ) -> RegisterCustomerCommand {
        RegisterCustomerCommand {
            name: first_name,
            last_name: last_name,
            email: email,
            hotel_id: hotel_id,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn last_name(&self) -> &String {
        &self.last_name
    }

    pub fn email(&self) -> &String {
        &self.email
    }

    pub fn hotel_id(&self) -> &i32 {
        &self.hotel_id
    }
}

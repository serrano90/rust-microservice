/**
 * Add Resort Request
 */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AddResort {
    name: String
} 

impl AddResort {
    pub fn new(
        name: String,
    ) -> Self {
        AddResort {
            name: name,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }
}
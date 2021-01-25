/**
 * Add Resort Request
 */
use serde::{Deserialize, Serialize};

pub struct FindResort {
    id: i32
} 

impl FindResort {
    pub fn new(
        id: i32,
    ) -> Self {
        FindResort {
            id: id,
        }
    }

    pub fn id(&self) -> &i32 {
        &self.id
    }
}
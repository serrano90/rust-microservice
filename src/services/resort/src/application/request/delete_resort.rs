/**
 * Add Resort Request
 */
use serde::{Deserialize, Serialize};

pub struct DeleteResort {
    id: i32
} 

impl DeleteResort {
    pub fn new(
        id: i32,
    ) -> Self {
        DeleteResort {
            id: id,
        }
    }

    pub fn id(&self) -> &i32 {
        &self.id
    }
}
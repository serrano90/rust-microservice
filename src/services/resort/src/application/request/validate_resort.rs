/**
 * Add Resort Request
 */
use serde::{Deserialize, Serialize};

pub struct ValidateResort {
    id: i32
} 

impl ValidateResort {
    pub fn new(
        id: i32,
    ) -> Self {
        ValidateResort {
            id: id,
        }
    }

    pub fn id(&self) -> &i32 {
        &self.id
    }
}
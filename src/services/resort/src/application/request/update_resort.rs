/**
 * Add Resort Request
 */
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UpdateResort {
    id: i32,
    name: String
} 

impl UpdateResort {
    pub fn new(
        id: i32,
        name: String,
    ) -> Self {
        UpdateResort {
            id: id,
            name: name,
        }
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn id(&self) -> &i32 {
        &self.id
    }
}
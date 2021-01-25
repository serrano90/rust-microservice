/**
 * Resort Entity
 */

pub struct Resort {
    id: i32,
    name: String
}

impl Resort {
    pub fn new(id: i32, name: String) -> Self {
        Resort {
            id: id,
            name: name,
        }
    }

    pub fn create(name: String) -> Self {
        Resort {
            id: 0,
            name: name,
        }
    }

    pub fn update(&mut self, id: i32, name: String) {
        self.id = id;
        self.name = name;
    }

    pub fn set_id(&mut self, id: i32) {
        self.id = id;
    }

    pub fn get_id(&self) -> i32 {
        self.id.clone()
    }

    pub fn get_name(&self) -> String {
        self.name.clone()
    }
}
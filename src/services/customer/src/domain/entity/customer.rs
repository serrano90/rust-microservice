/**
 * Customer Entity
 */
use uuid::Uuid;

// Customer
pub struct Customer {
    id: String,
    name: String,
    last_name: String,
    email: String,
    hotel_id: i32,
}

impl Customer {
    //Create a new instance of Customer
    pub fn new(
        id: String,
        name: String,
        last_name: String,
        email: String,
        hotel_id: i32,
    ) -> Customer {
        Customer {
            id: id,
            name: name,
            last_name: last_name,
            email: email,
            hotel_id: hotel_id,
        }
    }

    pub fn create(name: String, last_name: String, email: String, hotel_id: i32) -> Customer {
        let id = Uuid::new_v4();
        Customer::new(
            id.to_string(),
            name,
            last_name,
            email,
            hotel_id,
        )
    }

    pub fn id(&self) -> String {
        self.id.clone()
    }

    pub fn name(&self) -> String {
        self.name.clone()
    }

    pub fn last_name(&self) -> String {
        self.last_name.clone()
    }

    pub fn email(&self) -> String {
        self.email.clone()
    }

    pub fn hotel_id(&self) -> i32 {
        self.hotel_id.clone()
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.name, self.last_name)
    }
}

/**
 * ORM Customer Repository
 */
use diesel::RunQueryDsl;

use crate::domain::dto::customer::CustomerDTO;
use crate::domain::dto::customer::CustomerDTOList;
use crate::domain::entity::customer::Customer;
use crate::domain::repository::customer_repository::CustomerRepository;
use crate::infrastructure::models::customers::NewCustomer;
use crate::infrastructure::repository::connection::ConnectionManager;
use crate::schema::customers;
use crate::schema::customers::dsl::customers as customers_list;

pub struct ORMCustomerRepository {
    db: ConnectionManager,
}

impl ORMCustomerRepository {
    pub fn new(db: ConnectionManager) -> Self {
        ORMCustomerRepository { db: db }
    }
}

impl CustomerRepository for ORMCustomerRepository {
    fn all(&self, filter: String, page: i32, limit: i32) -> CustomerDTOList {
        let conn = &self.db.conn;

        let results = customers_list
        .load::<CustomerDTO>(conn)
        .expect("Error loading customers");


        CustomerDTOList(results)
    }

    fn create(&self, customer: Customer) -> CustomerDTO {
        let conn = &self.db.conn;

        let new_customer = NewCustomer {
            id: customer.id(),
            first_name: customer.name(),
            last_name: customer.last_name(),
            email: customer.email(),
            hotel_id: customer.hotel_id(),
        };

        let result = diesel::insert_into(customers::table)
            .values(&new_customer)
            .get_result::<CustomerDTO>(conn)
            .expect("Error saving Customer");

        result
    }
}

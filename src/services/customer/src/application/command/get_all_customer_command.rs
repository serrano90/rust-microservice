/**
 * Get All Customers
 */

pub struct GetAllCustomerCommand {
    filter: String,
    limit: i32,
    page: i32,
}

impl GetAllCustomerCommand {
    pub fn new(
        filter: String,
        limit: i32,
        page: i32,
    ) -> Self {
        GetAllCustomerCommand {
            filter: filter,
            limit: limit,
            page: page
        }
    }

    pub fn filter(&self) -> &String {
        &self.filter
    }

    pub fn limit(&self) -> &i32 {
        &self.limit
    }

    pub fn page(&self) -> &i32 {
        &self.page
    }
}


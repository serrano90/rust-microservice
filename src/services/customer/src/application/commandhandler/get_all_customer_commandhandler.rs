/**
 * Get All Customer CommandHandler
 */
use crate::application::command::get_all_customer_command::GetAllCustomerCommand;
use crate::domain::dto::customer::CustomerDTOList;
use crate::domain::repository::customer_repository::CustomerRepository;
use crate::infrastructure::repository::customer_repository::ORMCustomerRepository;

pub struct GetAllCustomerCommandHandler {
    repository: ORMCustomerRepository,
}

impl GetAllCustomerCommandHandler {
    pub fn new(repository: ORMCustomerRepository) -> Self {
        GetAllCustomerCommandHandler {
            repository: repository,
        }
    }

    pub fn handle(&self, cmd: GetAllCustomerCommand) -> CustomerDTOList {
        let result = self.repository.all(
            cmd.filter().clone(),
            cmd.page().clone(),
            cmd.limit().clone(),
        );

        result
    }
}

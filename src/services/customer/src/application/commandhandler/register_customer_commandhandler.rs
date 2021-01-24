/**
 * Register Customer CommandHandler
 */
use crate::application::command::register_customer_command::RegisterCustomerCommand;
use crate::domain::dto::customer::CustomerDTO;
use crate::domain::entity::customer::Customer;
use crate::domain::repository::customer_repository::CustomerRepository;
use crate::infrastructure::repository::customer_repository::ORMCustomerRepository;

pub struct RegisterCustomerCommandHandler {
    repository: ORMCustomerRepository,
}

impl RegisterCustomerCommandHandler {
    pub fn new(repository: ORMCustomerRepository) -> RegisterCustomerCommandHandler {
        RegisterCustomerCommandHandler {
            repository: repository,
        }
    }

    pub fn handle(&self, cmd: RegisterCustomerCommand) -> CustomerDTO {
        let customer = Customer::create(
            cmd.name().clone(),
            cmd.last_name().clone(),
            cmd.email().clone(),
            cmd.hotel_id().clone(),
        );

        let result = self.repository.create(customer);
        result
    }
}

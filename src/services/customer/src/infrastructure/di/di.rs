// use mut_static::MutStatic;

use crate::application::commandhandler::get_all_customer_commandhandler::GetAllCustomerCommandHandler;
use crate::application::commandhandler::register_customer_commandhandler::RegisterCustomerCommandHandler;

pub struct DIContainer {
    pub register_handler: RegisterCustomerCommandHandler,
    pub get_all_handler: GetAllCustomerCommandHandler,
}

impl DIContainer {
    pub fn new(
        register_handler: RegisterCustomerCommandHandler,
        get_all_handler: GetAllCustomerCommandHandler,
    ) -> Self {
        DIContainer {
            register_handler: register_handler,
            get_all_handler: get_all_handler,
        }
    }
    // pub fn get_register_handler(&self) -> DIContainer { self.register_handler }
    // pub fn set_register_handler(&mut self, v: DIContainer) { self.register_handler = v }
}

// lazy_static! {
//     pub static ref DI: MutStatic<DIContainer> = {
//         MutStatic::new()
//     };
// }

use log::trace;
use rocket;
use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use std::sync::Mutex;

use crate::application::command::get_all_customer_command::GetAllCustomerCommand;
use crate::application::command::register_customer_command::RegisterCustomerCommand;
use crate::application::commandhandler::get_all_customer_commandhandler::GetAllCustomerCommandHandler;
use crate::application::commandhandler::register_customer_commandhandler::RegisterCustomerCommandHandler;
use crate::domain::dto::customer::CustomerDTO;
use crate::domain::dto::customer::CustomerDTOList;
use crate::infrastructure::config::config::Config;
use crate::infrastructure::di::di::DIContainer;
use crate::infrastructure::repository::connection::ConnectionManager;
use crate::infrastructure::repository::customer_repository::ORMCustomerRepository;

//#[get("/?<filter>&<page>&<limit>", format = "json")]
#[get("/", format = "json")]
fn all(
    di: State<Mutex<DIContainer>>,
) -> Json<CustomerDTOList> {
    trace!("Get all customer call");

    let f: String = "".to_string();
    let p: i32 = 1;
    let l: i32 = 20;

    let command = GetAllCustomerCommand::new(f, l, p);
    let dic = di.lock().expect("shared state lock");
    let result = dic.get_all_handler.handle(command);
    
    Json(result)
}

#[post("/", format = "json", data = "<customer>")]
fn register(
    customer: Json<RegisterCustomerCommand>,
    di: State<Mutex<DIContainer>>,
) -> Json<CustomerDTO> {
    trace!("Get register a new customer call");
    let command = RegisterCustomerCommand::new(
        customer.name().clone(),
        customer.last_name().clone(),
        customer.email().clone(),
        customer.hotel_id().clone(),
    );

    let dic = di.lock().expect("shared state lock");
    let result = dic.register_handler.handle(command);
    Json(result)
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

pub fn load_router(config: &Config) -> rocket::Rocket {
    // Pass parameters
    // Load Communication Database
    let dbConnection = ConnectionManager::new(&config);
    let dbConnection2 = ConnectionManager::new(&config);
    // Load Repository
    let customer_repository = ORMCustomerRepository::new(dbConnection);
    let customer_repository2 = ORMCustomerRepository::new(dbConnection2);
    // Init all Command Handlers
    let register_handler = RegisterCustomerCommandHandler::new(customer_repository);
    let get_all_handler = GetAllCustomerCommandHandler::new(customer_repository2);

    let di = DIContainer::new(register_handler, get_all_handler);

    rocket::ignite()
        .mount("/v1/customers", routes![all, register])
        .manage(Mutex::new(di))
        .register(catchers![not_found])
}

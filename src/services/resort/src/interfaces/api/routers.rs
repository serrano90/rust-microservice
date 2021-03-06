use log::trace;
use rocket;
use rocket::State;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use std::sync::Mutex;

use crate::application::request::add_resort::AddResort;
use crate::application::request::find_resort::FindResort;
use crate::application::request::get_all_resort::GetAllResort;
use crate::application::request::update_resort::UpdateResort;
use crate::application::request::delete_resort::DeleteResort;
use crate::application::service::resort::ResortService;
use crate::domain::dto::resort::ResortDTO;
use crate::domain::dto::resort::ResortDTOList;
use crate::infrastructure::config::config::Config;
use crate::infrastructure::di::di::DIContainer;
use crate::infrastructure::repository::connection::ConnectionManager;
use crate::infrastructure::repository::resort_repository::ORMResortRepository;

//#[get("/?<filter>&<page>&<limit>", format = "json")]
#[get("/", format = "json")]
fn all(di: State<Mutex<DIContainer>>) -> Json<ResortDTOList> {
    trace!("Get all resort call");

    let dic = di.lock().expect("shared state lock");
    let result = dic.resort_service.list();

    Json(result)
}

#[post("/", format = "json", data = "<resort>")]
fn add(resort: Json<AddResort>, di: State<Mutex<DIContainer>>) -> Json<ResortDTO> {
    trace!("Get register a new resort call");

    let request = AddResort::new(resort.name().clone());
    let dic = di.lock().expect("shared state lock");
    let result = dic.resort_service.add(request);

    Json(result)
}

#[get("/<id>", format = "json")]
fn find(id: i32, di: State<Mutex<DIContainer>>) -> Json<ResortDTO> {
    trace!("Get all resort call");

    let dic = di.lock().expect("shared state lock");
    let result = dic.resort_service.find(FindResort::new(id));

    Json(result)
}


#[put("/<id>", format = "json", data = "<resort>")]
fn update(id: i32, resort: Json<AddResort>, di: State<Mutex<DIContainer>>) -> JsonValue {
    trace!("Get update a new resort call");

    let request = UpdateResort::new(id, resort.name().clone());
    let dic = di.lock().expect("shared state lock");
    let result = dic.resort_service.update(request);

    json!(result)
}

#[delete("/<id>")]
fn delete(id: i32, di: State<Mutex<DIContainer>>) -> JsonValue {
    trace!("Get delete a new resort call");

    let request = DeleteResort::new(id);
    let dic = di.lock().expect("shared state lock");
    let result = dic.resort_service.delete(request);

    json!(result)
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
    // Load Repository
    let resort_repository = ORMResortRepository::new(dbConnection);
    // Init all Service
    let resort_service = ResortService::new(resort_repository);

    let di = DIContainer::new(resort_service);

    rocket::ignite()
        .mount("/v1/resorts", routes![add, all, find, update, delete])
        .manage(Mutex::new(di))
        .register(catchers![not_found])
}

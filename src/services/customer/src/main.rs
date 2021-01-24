#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate dotenv;
#[macro_use]
extern crate log;
#[macro_use]
extern crate lazy_static;
extern crate mut_static;

mod application;
mod domain;
mod infrastructure;
mod interfaces;
mod schema;

fn main() {
    let config = infrastructure::config::config::load_config();
    
    let app = interfaces::api::server::Adapter::new(config);
    app.start();
}

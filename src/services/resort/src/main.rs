#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate log;

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

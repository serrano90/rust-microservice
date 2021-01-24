/**
 * Global configuration access
*/
use dotenv::dotenv;
use std::env;

pub struct Config {
    pub app: App,
    pub database: Database,
}

pub struct App {
    pub name: String,
    pub port: String
}

pub struct Database {
    pub url: String
}

pub fn load_config() -> Config{
    dotenv().ok();

    let app = App {
        name: env::var("APP_NAME").expect("app name value doesn't exist"),
        port: env::var("ROCKET_PORT").expect("rocket port value doesn't exist"),
    };

    let database = Database {
        url: env::var("DATABASE_URL").expect("database url value doesn't exist")
    };

    Config {
        app: app,
        database: database,
    }
}
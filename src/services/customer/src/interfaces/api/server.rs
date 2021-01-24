/**
 * Api Server Adapter
 */
use log::{info};
use crate::infrastructure::config::config::Config;
use crate::interfaces::api::routers;


// Adapter definition for API access 
pub struct Adapter {
    config: Config,
}

impl Adapter {
    pub fn new(config: Config) -> Self {
        Adapter {
            config: config,
        }
    }

    pub fn start(&self) {
        let r = routers::load_router(&self.config);
        info!("Running server {} by port {}", self.config.app.name, self.config.app.port);
        r.launch();
    }
}
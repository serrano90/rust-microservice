// use mut_static::MutStatic;

use crate::application::service::resort::ResortService;

pub struct DIContainer {
    pub resort_service: ResortService,
}

impl DIContainer {
    pub fn new(
        resort_service: ResortService,
    ) -> Self {
        DIContainer {
            resort_service: resort_service,
        }
    }
}

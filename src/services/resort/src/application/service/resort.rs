/**
 * Resort Service
 */

use crate::domain::repository::resort_repository::ResortRepository;
use crate::infrastructure::repository::resort_repository::ORMResortRepository;
use crate::domain::entity::resort::Resort;
use crate::domain::dto::resort::ResortDTO;
use crate::domain::dto::resort::ResortDTOList;
use crate::application::request::add_resort::AddResort;
use crate::application::request::validate_resort::ValidateResort;
use crate::application::request::*;

pub struct ResortService {
    repository: ORMResortRepository,
}

impl ResortService {
    pub fn new(repository: ORMResortRepository) -> Self {
        ResortService {
            repository: repository,
        }
    }

    pub fn add(&self, request: AddResort) -> ResortDTO {
        let resort = Resort::create(
            request.name().clone(),
        );
        
        self.repository.create(resort)
    }

    pub fn validate(&self, request: ValidateResort) -> bool {
         true
    }
    
    pub fn list(&self) -> ResortDTOList {
        let result = self.repository.all();

        result
    }
}
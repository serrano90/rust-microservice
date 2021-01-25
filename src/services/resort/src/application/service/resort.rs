/**
 * Resort Service
 */

use crate::domain::repository::resort_repository::ResortRepository;
use crate::infrastructure::repository::resort_repository::ORMResortRepository;
use crate::domain::entity::resort::Resort;
use crate::domain::dto::resort::ResortDTO;
use crate::domain::dto::resort::ResortDTOList;
use crate::application::request::add_resort::AddResort;
use crate::application::request::delete_resort::DeleteResort;
use crate::application::request::find_resort::FindResort;
use crate::application::request::update_resort::UpdateResort;
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
    
    pub fn list(&self) -> ResortDTOList {
        let result = self.repository.all();

        result
    }

    pub fn find(&self, request: FindResort) -> ResortDTO {
        let result = self.repository.find(request.id().clone());

        result
    }

    pub fn update(&self, request: UpdateResort) -> bool {
        let resort = Resort::new(
            request.id().clone(),
            request.name().clone(),
        );

        self.repository.update(resort)
    }

    pub fn delete(&self, request: DeleteResort) -> bool {
        let result = self.repository.delete(request.id().clone());
        result
    }
}